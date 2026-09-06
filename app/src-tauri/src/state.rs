//! Session state for the GUI: which backend and display mode are active,
//! the working scale, the history list and the memory register.
//!
//! `dz-api` is stateless by design (`docs/04-fase6-interfaces.md`, 1.4):
//! history and memory are user-interface concepts, so they live here, on
//! the Rust side of the bridge. The frontend never decides anything about
//! dozenal semantics -- it renders what this state and `dz-api` hand it.

use dz_api::{BackendCost, BackendKind, DisplayAlphabet, InputBase, NumberMode, Trace};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// How many entries the history keeps. A calculator's history is a
/// convenience, not a log; an unbounded one would grow for the lifetime of
/// the process.
const HISTORY_LIMIT: usize = 100;

#[derive(Clone, Debug, Serialize)]
pub struct HistoryEntry {
    pub input: String,
    pub display: String,
    pub approximate: bool,
    pub periodic: bool,
}

/// Which of the two calculators a command is talking to.
///
/// They are two calculators side by side, not two views of one: each keeps
/// its own half-typed input, its own last result, its own history, its own
/// memory register and its own settings (`docs/10-entrada-base-10.md`,
/// section 8). Every command therefore names the face it operates on;
/// there is deliberately no "current face" stored here, because a hidden
/// one would be a third piece of state the two visible ones could
/// disagree with.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Face {
    /// The base-ten front: decimal in, dozenal out. Its settings are
    /// fixed, so its face shows no controls for them.
    Simple,
    /// The dozenal calculator exactly as it was before this phase.
    Advanced,
}

#[derive(Debug)]
pub struct Session {
    pub backend: BackendKind,
    pub mode: NumberMode,
    pub scale: usize,
    /// The base the *typed* text is read in (`docs/10`). An input setting
    /// only: what comes back is dozenal in both faces.
    pub input_base: InputBase,
    /// The presentation alphabet (`docs/07-alfabeto-configurable.md`).
    ///
    /// It lives here even though it is a per-machine *visual* preference
    /// persisted in `localStorage`, because the translation itself is
    /// dozenal semantics and must run in Rust: the frontend re-applies
    /// the stored preference at boot and this field is what Rust
    /// translates with. Everything stored below -- history, memory --
    /// holds **canonical** text, so switching alphabets repaints the
    /// whole list instead of mixing two spellings.
    pub alphabet: DisplayAlphabet,
    pub history: Vec<HistoryEntry>,
    /// The M+/MR register: the last stored result, as text.
    pub memory: Option<String>,
    /// Phase 7: what the research panel last produced. Kept here so the
    /// CSV export re-serializes the very table the user is looking at,
    /// rather than recomputing something that might differ (a different
    /// backend, a different operand) between the click and the save.
    pub last_comparison: Option<Vec<BackendCost>>,
    pub last_trace: Option<Trace>,
}

impl Default for Session {
    fn default() -> Session {
        Session {
            backend: BackendKind::Mvl,
            mode: NumberMode::Rational,
            scale: dz_api::default_scale(),
            input_base: InputBase::Twelve,
            alphabet: DisplayAlphabet::canonical(),
            history: Vec::new(),
            memory: None,
            last_comparison: None,
            last_trace: None,
        }
    }
}

impl Session {
    /// The simple face's fixed settings: base ten in, rational mode, MVL,
    /// the default scale.
    ///
    /// Rational is the load-bearing one. It is the exact mode, so a
    /// displayed result is always `n` or `n/d` -- two integers -- and the
    /// conversion back to base ten for chaining (`docs/10` section 3.1)
    /// never has to face the fixed-point case, where the value is
    /// `magnitude / 12^scale` and often has no finite decimal form.
    fn simple() -> Session {
        Session {
            input_base: InputBase::Ten,
            mode: NumberMode::Rational,
            ..Session::default()
        }
    }

    pub fn push_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
        if self.history.len() > HISTORY_LIMIT {
            self.history.remove(0);
        }
    }
}

/// Both sessions, behind one lock: a command names a face and then picks
/// one, so splitting them into two mutexes would only add a way to take
/// them in the wrong order.
#[derive(Debug)]
pub struct Sessions {
    pub simple: Session,
    pub advanced: Session,
}

impl Default for Sessions {
    fn default() -> Sessions {
        Sessions {
            simple: Session::simple(),
            advanced: Session::default(),
        }
    }
}

impl Sessions {
    pub fn get(&self, face: Face) -> &Session {
        match face {
            Face::Simple => &self.simple,
            Face::Advanced => &self.advanced,
        }
    }

    pub fn get_mut(&mut self, face: Face) -> &mut Session {
        match face {
            Face::Simple => &mut self.simple,
            Face::Advanced => &mut self.advanced,
        }
    }
}

/// The managed state handle. A blocking `Mutex` is right here: every
/// command is short and synchronous, so there is nothing to await.
#[derive(Default)]
pub struct AppState(pub Mutex<Sessions>);
