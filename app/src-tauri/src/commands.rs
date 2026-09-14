//! The IPC surface: thin wrappers with no dozenal logic of their own.
//!
//! Each command borrows the session for the active backend/mode/scale,
//! builds a `dz_api::Request`, and returns what `dz-api` already produced:
//! formatted strings and plain data. Nothing that crosses this boundary
//! invites arithmetic on the JavaScript side, which is the purity rule
//! applied to a language boundary (`docs/04-fase6-interfaces.md`, 1.2d).
//!
//! `serde` appears here and nowhere below: the DTOs are this crate's, not
//! `dz-api`'s.

use crate::state::{AppState, Face, HistoryEntry};
use dz_api::{BackendKind, DisplayAlphabet, NumberMode, Operation, Request};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Mirrors `dz_api::Operation`. Tagged by `kind`, so the frontend sends
/// `{ kind: "eval", expr: "1+2" }` or `{ kind: "call", name: "sin",
/// args: ["1"] }`.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum OperationDto {
    Sum { a: String, b: String },
    Sub { a: String, b: String },
    Mul { a: String, b: String },
    Div { a: String, b: String },
    Mod { a: String, b: String },
    Pow { base: String, exponent: String },
    Neg { a: String },
    Call { name: String, args: Vec<String> },
    Eval { expr: String },
}

impl OperationDto {
    /// The text the history should show for this request.
    fn label(&self) -> String {
        match self {
            OperationDto::Sum { a, b } => format!("{} + {}", a, b),
            OperationDto::Sub { a, b } => format!("{} - {}", a, b),
            OperationDto::Mul { a, b } => format!("{} * {}", a, b),
            OperationDto::Div { a, b } => format!("{} / {}", a, b),
            OperationDto::Mod { a, b } => format!("{} % {}", a, b),
            OperationDto::Pow { base, exponent } => format!("{} ^ {}", base, exponent),
            OperationDto::Neg { a } => format!("-{}", a),
            OperationDto::Call { name, args } => format!("{}({})", name, args.join(", ")),
            OperationDto::Eval { expr } => expr.clone(),
        }
    }

    pub(crate) fn into_operation(self) -> Operation {
        match self {
            OperationDto::Sum { a, b } => Operation::Sum { a, b },
            OperationDto::Sub { a, b } => Operation::Sub { a, b },
            OperationDto::Mul { a, b } => Operation::Mul { a, b },
            OperationDto::Div { a, b } => Operation::Div { a, b },
            OperationDto::Mod { a, b } => Operation::Mod { a, b },
            OperationDto::Pow { base, exponent } => Operation::Pow { base, exponent },
            OperationDto::Neg { a } => Operation::Neg { a },
            OperationDto::Call { name, args } => Operation::Call { name, args },
            OperationDto::Eval { expr } => Operation::Eval { expr },
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct GateCountDto {
    pub cycle: u64,
    pub min: u64,
    pub max: u64,
    pub lit: u64,
    pub thr: u64,
    pub total: u64,
}

/// What the display renders. `gate_count` is populated from day one even
/// though the Phase 6 frontend only shows the total: the Phase 7 research
/// panel is then a frontend change with no Rust change at all.
#[derive(Debug, Serialize)]
pub struct ResponseDto {
    pub display: String,
    pub periodic_note: bool,
    pub approximate: bool,
    pub gate_count: Option<GateCountDto>,
    /// Circuit depth of the same run (Phase 7): measured alongside the
    /// counters, in the same pass, without changing them.
    pub depth: Option<u32>,
    pub backend: String,
    pub mode: String,
    pub scale: usize,
}

#[derive(Debug, Serialize)]
pub struct SessionDto {
    pub backend: String,
    pub mode: String,
    pub scale: usize,
    pub history: Vec<HistoryEntry>,
    pub memory: Option<String>,
    pub backends: Vec<String>,
    pub modes: Vec<String>,
    /// The presentation symbols for ten and eleven, as single-character
    /// strings. The frontend paints them on the keypad and in the settings
    /// modal; it never uses them to translate anything, because everything
    /// it receives is already translated here.
    pub alphabet_ten: String,
    pub alphabet_eleven: String,
    /// The base this face reads typed numbers in, as the same flag value
    /// `dz-cli` uses: `"12"` or `"10"` (`docs/10` section 6). The frontend
    /// shows it and, on the simple face, uses it to decide that the
    /// keypad has no `↊`/`↋` -- it never converts with it.
    pub input_base: String,
}

/// The chain-back conversion: the text to seed the input box with, and
/// what producing it cost.
#[derive(Debug, Serialize)]
pub struct ChainDto {
    pub text: String,
    pub gate_count: GateCountDto,
}

#[derive(Debug, Serialize)]
pub struct FunctionDto {
    pub name: String,
    pub min_args: usize,
    pub max_args: usize,
    pub category: String,
    pub description: String,
    pub always_exact: bool,
}

/// Everything the session hands out is rendered through the presentation
/// alphabet *here*, on the way out.
///
/// The stored text stays canonical, so this is also what makes changing
/// the alphabet repaint the whole history rather than only the entries
/// added afterwards (`docs/07` section 3.5). Only `display` is a dozenal
/// value; `input` is the expression the user typed, which was already in
/// their own alphabet, so translating it again would double-apply.
fn session_dto(session: &crate::state::Session) -> SessionDto {
    let alphabet = session.alphabet;
    SessionDto {
        backend: session.backend.label().to_string(),
        mode: session.mode.label().to_string(),
        scale: session.scale,
        history: session
            .history
            .iter()
            .map(|entry| HistoryEntry {
                input: entry.input.clone(),
                display: alphabet.from_canonical(&entry.display),
                approximate: entry.approximate,
                periodic: entry.periodic,
            })
            .collect(),
        memory: session
            .memory
            .as_deref()
            .map(|value| alphabet.from_canonical(value)),
        backends: BackendKind::ALL.iter().map(|b| b.label().to_string()).collect(),
        modes: NumberMode::ALL.iter().map(|m| m.label().to_string()).collect(),
        alphabet_ten: alphabet.ten().to_string(),
        alphabet_eleven: alphabet.eleven().to_string(),
        input_base: session.input_base.label().to_string(),
    }
}

/// Evaluate one operation with the settings of `face`'s session.
#[tauri::command]
pub fn calc(
    state: State<AppState>,
    face: Face,
    op: OperationDto,
) -> Result<ResponseDto, String> {
    let (backend, mode, scale, alphabet, input_base) = {
        let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        let session = sessions.get(face);
        (
            session.backend,
            session.mode,
            session.scale,
            session.alphabet,
            session.input_base,
        )
    };

    let label = op.label();
    let request = Request::new(op.into_operation())
        .with_backend(backend)
        .with_mode(mode)
        .with_scale(scale)
        .with_alphabet(alphabet)
        .with_input_base(input_base);

    // The error is already structured on the Rust side; it crosses the
    // bridge as its display text because that is all a dialog needs.
    let response = dz_api::execute(&request).map_err(|e| e.to_string())?;

    {
        let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        sessions.get_mut(face).push_history(HistoryEntry {
            input: label,
            display: response.display.clone(),
            approximate: response.approximate,
            periodic: response.periodic_note,
        });
    }

    Ok(ResponseDto {
        // The history above keeps `response.display` canonical; what the
        // screen shows goes through the alphabet.
        display: alphabet.from_canonical(&response.display),
        periodic_note: response.periodic_note,
        approximate: response.approximate,
        gate_count: response.gate_count.as_ref().map(gate_count_dto),
        depth: response.depth,
        backend: backend.label().to_string(),
        mode: mode.label().to_string(),
        scale,
    })
}

/// Hot backend switch -- already what the Phase 7 research panel needs,
/// so that phase adds a chart, not a command.
#[tauri::command]
pub fn set_backend(
    state: State<AppState>,
    face: Face,
    backend: String,
) -> Result<SessionDto, String> {
    let kind = BackendKind::parse(&backend).ok_or(format!("unknown backend '{}'", backend))?;
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.backend = kind;
    Ok(session_dto(session))
}

#[tauri::command]
pub fn set_mode(state: State<AppState>, face: Face, mode: String) -> Result<SessionDto, String> {
    let kind = NumberMode::parse(&mode).ok_or(format!("unknown mode '{}'", mode))?;
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.mode = kind;
    Ok(session_dto(session))
}

/// Set the base this face reads typed numbers in (`docs/10`).
///
/// Only the advanced face offers it: the simple one is pinned to ten, and
/// its UI has no control for this. It is an *input* setting -- what comes
/// back is dozenal either way -- so, like `:inputbase` in the REPL, it
/// repaints nothing: it changes how the next line is read.
#[tauri::command]
pub fn set_input_base(
    state: State<AppState>,
    face: Face,
    base: String,
) -> Result<SessionDto, String> {
    let kind = dz_api::InputBase::parse(&base).ok_or(format!("unknown input base '{}'", base))?;
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.input_base = kind;
    Ok(session_dto(session))
}

/// The result of the last calculation, rewritten into this face's input
/// base, for result chaining (`docs/09`, `docs/10` section 3).
///
/// It takes no value, for the same reason `memory_store` does not: the
/// canonical copy lives here, and letting the frontend pass the on-screen
/// string back would hand this whatever alphabet was showing.
///
/// On the base-ten face this is what stops a silent wrong answer. `144 +
/// 11` shows `10↋`; seeding that back into a box that is read as decimal
/// would continue from a different number, with nothing on screen saying
/// so. The conversion is gate work, so its cost comes back alongside the
/// text -- separate from the calculation's own bill, because the user
/// asked for it separately.
#[tauri::command]
pub fn chain_operand(state: State<AppState>, face: Face) -> Result<ChainDto, String> {
    let (backend, input_base, display) = {
        let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        let session = sessions.get(face);
        (
            session.backend,
            session.input_base,
            session.history.last().map(|entry| entry.display.clone()),
        )
    };
    let display = display.ok_or("nothing to chain from")?;
    let (text, count) =
        dz_api::chain_operand(backend, input_base, &display).map_err(|e| e.to_string())?;
    Ok(ChainDto {
        text,
        gate_count: gate_count_dto(&count),
    })
}

/// Working digits for the approximate functions. Bounded so a slip of the
/// keyboard cannot ask for a computation that never returns.
#[tauri::command]
pub fn set_scale(state: State<AppState>, face: Face, scale: usize) -> Result<SessionDto, String> {
    if scale == 0 || scale > 60 {
        return Err("scale must be between 1 and 60 digits".to_string());
    }
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.scale = scale;
    Ok(session_dto(session))
}

#[tauri::command]
pub fn get_session(state: State<AppState>, face: Face) -> Result<SessionDto, String> {
    let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    Ok(session_dto(sessions.get(face)))
}

#[tauri::command]
pub fn clear_history(state: State<AppState>, face: Face) -> Result<SessionDto, String> {
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.history.clear();
    Ok(session_dto(session))
}

/// Store the last result in the memory register.
///
/// It takes no value on purpose: the text it stores must be **canonical**,
/// and the only canonical copy lives here. Letting the frontend pass the
/// string back would store whatever alphabet was on screen at the time,
/// and the register would stop repainting when the alphabet changed. Same
/// shape as the REPL's `:mem+`.
#[tauri::command]
pub fn memory_store(state: State<AppState>, face: Face) -> Result<SessionDto, String> {
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.memory = session.history.last().map(|entry| entry.display.clone());
    Ok(session_dto(session))
}

/// Set the presentation symbols for ten and eleven.
///
/// Validation is `dz-api`'s -- reserved characters, a duplicated symbol,
/// and a pair that would swallow a catalog name (`p`/`i` and `pi`). The
/// error text crosses the bridge verbatim so the modal shows exactly what
/// Rust decided, rather than a second wording of the same rule.
#[tauri::command]
pub fn set_alphabet(
    state: State<AppState>,
    face: Face,
    ten: String,
    eleven: String,
) -> Result<SessionDto, String> {
    let one = |text: &str, which: &str| -> Result<char, String> {
        let mut chars = text.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(format!("the symbol for {} must be exactly one character", which)),
        }
    };
    let ten = one(&ten, "ten")?;
    let eleven = one(&eleven, "eleven")?;
    let alphabet = dz_api::display_alphabet(ten, eleven).map_err(|e| e.to_string())?;
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.alphabet = alphabet;
    Ok(session_dto(session))
}

/// Back to `↊`/`↋`, the project's canonical pair.
#[tauri::command]
pub fn reset_alphabet(state: State<AppState>, face: Face) -> Result<SessionDto, String> {
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.alphabet = DisplayAlphabet::canonical();
    Ok(session_dto(session))
}

#[tauri::command]
pub fn memory_clear(state: State<AppState>, face: Face) -> Result<SessionDto, String> {
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    let session = sessions.get_mut(face);
    session.memory = None;
    Ok(session_dto(session))
}

/// The keypad's scientific section is built from this list, so a function
/// registered in `dz-eval` appears in the GUI with no frontend change --
/// except for functions marked `cli_only` (currently just `gamma`), which
/// this filter keeps off the keypad on purpose. `dz-api::functions()`
/// itself stays unfiltered -- `dz-cli` calls it directly for `--functions`
/// and `:funcs`, and CLI availability is exactly what `cli_only` means to
/// preserve. This is the one place that reads the flag; nothing else in
/// the GUI re-derives or duplicates the function table.
#[tauri::command]
pub fn list_functions() -> Vec<FunctionDto> {
    dz_api::functions()
        .into_iter()
        .filter(|info| !info.cli_only)
        .map(|info| FunctionDto {
            name: info.name.to_string(),
            min_args: info.arity.min,
            max_args: info.arity.max,
            category: info.category.label().to_string(),
            description: info.description.to_string(),
            always_exact: info.always_exact,
        })
        .collect()
}

/// Shared mapping so the calculator display and the research panel's
/// comparison table cannot drift into two spellings of the same table.
pub(crate) fn gate_count_dto(count: &dz_api::GateCount) -> GateCountDto {
    GateCountDto {
        cycle: count.cycle,
        min: count.min,
        max: count.max,
        lit: count.lit,
        thr: count.thr,
        total: count.total(),
    }
}
