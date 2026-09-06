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
}

export interface FunctionInfo {
  name: string;
  min_args: number;
  max_args: number;
  category: string;
  description: string;
  always_exact: boolean;
}

export const calc = (op: Operation): Promise<CalcResponse> => invoke("calc", { op });

export const getSession = (): Promise<Session> => invoke("get_session");

export const setBackend = (backend: string): Promise<Session> =>
  invoke("set_backend", { backend });

export const setMode = (mode: string): Promise<Session> => invoke("set_mode", { mode });

export const setScale = (scale: number): Promise<Session> => invoke("set_scale", { scale });

export const clearHistory = (): Promise<Session> => invoke("clear_history");

export const memoryStore = (value: string): Promise<Session> =>
  invoke("memory_store", { value });

export const memoryClear = (): Promise<Session> => invoke("memory_clear");

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

export const compareBackends = (op: Operation): Promise<Comparison> =>
  invoke("compare_backends", { op });

/// Rust builds the CSV and writes it; this only hands over the path the
/// user picked in the native dialog.
export const exportCsv = (what: string, path: string): Promise<string> =>
  invoke("export_csv", { what, path });
