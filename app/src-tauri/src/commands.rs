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

use crate::state::{AppState, HistoryEntry};
use dz_api::{BackendKind, NumberMode, Operation, Request};
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

fn session_dto(session: &crate::state::Session) -> SessionDto {
    SessionDto {
        backend: session.backend.label().to_string(),
        mode: session.mode.label().to_string(),
        scale: session.scale,
        history: session.history.clone(),
        memory: session.memory.clone(),
        backends: BackendKind::ALL.iter().map(|b| b.label().to_string()).collect(),
        modes: NumberMode::ALL.iter().map(|m| m.label().to_string()).collect(),
    }
}

/// Evaluate one operation with the session's current settings.
#[tauri::command]
pub fn calc(state: State<AppState>, op: OperationDto) -> Result<ResponseDto, String> {
    let (backend, mode, scale) = {
        let session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        (session.backend, session.mode, session.scale)
    };

    let label = op.label();
    let request = Request::new(op.into_operation())
        .with_backend(backend)
        .with_mode(mode)
        .with_scale(scale);

    // The error is already structured on the Rust side; it crosses the
    // bridge as its display text because that is all a dialog needs.
    let response = dz_api::execute(&request).map_err(|e| e.to_string())?;

    {
        let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        session.push_history(HistoryEntry {
            input: label,
            display: response.display.clone(),
            approximate: response.approximate,
            periodic: response.periodic_note,
        });
    }

    Ok(ResponseDto {
        display: response.display,
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
pub fn set_backend(state: State<AppState>, backend: String) -> Result<SessionDto, String> {
    let kind = BackendKind::parse(&backend).ok_or(format!("unknown backend '{}'", backend))?;
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.backend = kind;
    Ok(session_dto(&session))
}

#[tauri::command]
pub fn set_mode(state: State<AppState>, mode: String) -> Result<SessionDto, String> {
    let kind = NumberMode::parse(&mode).ok_or(format!("unknown mode '{}'", mode))?;
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.mode = kind;
    Ok(session_dto(&session))
}

/// Working digits for the approximate functions. Bounded so a slip of the
/// keyboard cannot ask for a computation that never returns.
#[tauri::command]
pub fn set_scale(state: State<AppState>, scale: usize) -> Result<SessionDto, String> {
    if scale == 0 || scale > 60 {
        return Err("scale must be between 1 and 60 digits".to_string());
    }
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.scale = scale;
    Ok(session_dto(&session))
}

#[tauri::command]
pub fn get_session(state: State<AppState>) -> Result<SessionDto, String> {
    let session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    Ok(session_dto(&session))
}

#[tauri::command]
pub fn clear_history(state: State<AppState>) -> Result<SessionDto, String> {
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.history.clear();
    Ok(session_dto(&session))
}

#[tauri::command]
pub fn memory_store(state: State<AppState>, value: String) -> Result<SessionDto, String> {
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.memory = Some(value);
    Ok(session_dto(&session))
}

#[tauri::command]
pub fn memory_clear(state: State<AppState>) -> Result<SessionDto, String> {
    let mut session = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    session.memory = None;
    Ok(session_dto(&session))
}

/// The keypad's scientific section is built from this list, so a function
/// registered in `dz-eval` appears in the GUI with no frontend change.
#[tauri::command]
pub fn list_functions() -> Vec<FunctionDto> {
    dz_api::functions()
        .into_iter()
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
