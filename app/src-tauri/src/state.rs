//! Session state for the GUI: which backend and display mode are active,
//! the working scale, the history list and the memory register.
//!
//! `dz-api` is stateless by design (`docs/04-fase6-interfaces.md`, 1.4):
//! history and memory are user-interface concepts, so they live here, on
//! the Rust side of the bridge. The frontend never decides anything about
//! dozenal semantics -- it renders what this state and `dz-api` hand it.

use dz_api::{BackendCost, BackendKind, NumberMode, Trace};
use serde::Serialize;
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

#[derive(Debug)]
pub struct Session {
    pub backend: BackendKind,
    pub mode: NumberMode,
    pub scale: usize,
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
            history: Vec::new(),
            memory: None,
            last_comparison: None,
            last_trace: None,
        }
    }
}

impl Session {
    pub fn push_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
        if self.history.len() > HISTORY_LIMIT {
            self.history.remove(0);
        }
    }
}

/// The managed state handle. A blocking `Mutex` is right here: every
/// command is short and synchronous, so there is nothing to await.
#[derive(Default)]
pub struct AppState(pub Mutex<Session>);
