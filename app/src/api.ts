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
