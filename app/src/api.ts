// Typed wrappers over the Tauri commands in `src-tauri/src/commands.rs`.
//
// This is the only file that knows the bridge exists. Everything the
// frontend receives is already formatted text plus plain metadata: no
// dozenal value is ever a JavaScript number, because arithmetic on one
// here would be exactly the thing the project forbids.

import { invoke } from "@tauri-apps/api/core";

export type Operation =
  | { kind: "sum"; a: string; b: string }
  | { kind: "sub"; a: string; b: string }
  | { kind: "mul"; a: string; b: string }
  | { kind: "div"; a: string; b: string }
  | { kind: "mod"; a: string; b: string }
  | { kind: "pow"; base: string; exponent: string }
  | { kind: "neg"; a: string }
  | { kind: "call"; name: string; args: string[] }
  | { kind: "eval"; expr: string };

export interface GateCount {
  cycle: number;
  min: number;
  max: number;
  lit: number;
  thr: number;
  total: number;
}

export interface CalcResponse {
  display: string;
  periodic_note: boolean;
  approximate: boolean;
  gate_count: GateCount | null;
  depth: number | null;
  backend: string;
  mode: string;
  scale: number;
}

export interface HistoryEntry {
  input: string;
  display: string;
  approximate: boolean;
  periodic: boolean;
}

export interface Session {
  backend: string;
  mode: string;
  scale: number;
  history: HistoryEntry[];
  memory: string | null;
  backends: string[];
  modes: string[];
  /// The presentation symbols for ten and eleven. The frontend paints
  /// them; it never translates with them. Everything it receives from
  /// Rust is already written in this alphabet, and everything it sends
  /// back is translated by Rust on arrival (`docs/07`).
  alphabet_ten: string;
  alphabet_eleven: string;
  /// The base this face reads typed numbers in: `"12"` or `"10"`
  /// (`docs/10`). Shown and used to pick a keypad; never used to convert
  /// anything here.
  input_base: string;
}

export interface FunctionInfo {
  name: string;
  min_args: number;
  max_args: number;
  category: string;
  description: string;
  always_exact: boolean;
}

/// Which of the two calculators a call is about.
///
/// They are two calculators side by side, each with its own input,
/// result, history, memory and settings, and all of that state lives in
/// Rust (`src-tauri/src/state.rs`). This string is the only thing the
/// frontend has to remember about the split.
export type Face = "simple" | "advanced";

/// The chain-back conversion (`docs/10` section 3): the last result
/// rewritten into this face's input base, plus what that cost.
export interface ChainOperand {
  text: string;
  gate_count: GateCount;
}

export const calc = (face: Face, op: Operation): Promise<CalcResponse> =>
  invoke("calc", { face, op });

export const getSession = (face: Face): Promise<Session> => invoke("get_session", { face });

export const setBackend = (face: Face, backend: string): Promise<Session> =>
  invoke("set_backend", { face, backend });

export const setMode = (face: Face, mode: string): Promise<Session> =>
  invoke("set_mode", { face, mode });

export const setScale = (face: Face, scale: number): Promise<Session> =>
  invoke("set_scale", { face, scale });

export const setInputBase = (face: Face, base: string): Promise<Session> =>
  invoke("set_input_base", { face, base });

export const clearHistory = (face: Face): Promise<Session> =>
  invoke("clear_history", { face });

/// The last result, converted back into the face's input base so it can
/// be typed on again. It takes no value for the same reason `memoryStore`
/// does not: the canonical copy lives in Rust, and the conversion is gate
/// work that only Rust may do.
export const chainOperand = (face: Face): Promise<ChainOperand> =>
  invoke("chain_operand", { face });

/// Stores the last result. It takes no value because the canonical
/// text lives on the Rust side; sending the on-screen string back would
/// freeze the register in whatever alphabet was showing.
export const memoryStore = (face: Face): Promise<Session> => invoke("memory_store", { face });

export const memoryClear = (face: Face): Promise<Session> => invoke("memory_clear", { face });

/// Set the symbols for ten and eleven. Rejected pairs come back as the
/// rejection Rust wrote, which the modal shows verbatim.
export const setAlphabet = (face: Face, ten: string, eleven: string): Promise<Session> =>
  invoke("set_alphabet", { face, ten, eleven });

export const resetAlphabet = (face: Face): Promise<Session> =>
  invoke("reset_alphabet", { face });

export const listFunctions = (): Promise<FunctionInfo[]> => invoke("list_functions");

// --- Phase 7: the research panel -------------------------------------
//
// Every dozenal value below is a string of already-resolved glyphs. There
// is deliberately no shape here a caller could do arithmetic on.

export interface TraceStep {
  position: number;
  a: string;
  b: string;
  carry_in: string;
  sum: string;
  carry_out: string;
}

export interface MulRow {
  index: number;
  multiplier_digit: string;
  row: string;
  shifted: string;
  accumulate_steps: TraceStep[];
  accumulated: string;
}

export type TraceKind = "add" | "sub" | "mul";

export type Trace =
  | { kind: "add"; a: string; b: string; steps: TraceStep[]; result: string }
  | {
      kind: "sub";
      a: string;
      b: string;
      ones_complement: string;
      complement_steps: TraceStep[];
      radix_complement: string;
      add_steps: TraceStep[];
      raw_sum: string;
      borrowed: boolean;
      decode_ones_complement: string;
      decode_steps: TraceStep[];
      result: string;
    }
  | { kind: "mul"; a: string; b: string; rows: MulRow[]; result: string };

export interface BackendCost {
  backend: string;
  gate_count: GateCount;
  depth: number;
  /// Internal-representation -> tag reconstructions: 0 for mvl and for
  /// the oracle, non-zero only for crt. The five primitive counters are
  /// equal across backends by construction; this is the column that is
  /// not.
  reconstruction_ops: number;
  oracle: boolean;
}

/// Written by Rust (`dz_api::verdict`), never composed here. The panel
/// prints `summary` as it arrives, so the GUI and the terminal cannot
/// disagree about how to read the table.
export interface Verdict {
  winner: string;
  decided_by_reconstructions: boolean;
  model: string;
  summary: string;
}

export interface Comparison {
  display: string;
  costs: BackendCost[];
  verdict: Verdict | null;
}

export const traceOperation = (kind: TraceKind, a: string, b: string): Promise<Trace> =>
  invoke("trace_operation", { kind, a, b });

/// `face` says whose operation this is, so the three runs re-read the
/// operand text in the same input base the face that produced it did.
export const compareBackends = (face: Face, op: Operation): Promise<Comparison> =>
  invoke("compare_backends", { face, op });

/// Rust builds the CSV and writes it; this only hands over the path the
/// user picked in the native dialog.
export const exportCsv = (what: string, path: string): Promise<string> =>
  invoke("export_csv", { what, path });
