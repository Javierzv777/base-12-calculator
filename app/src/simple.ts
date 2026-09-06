// The base-ten face (`docs/10-entrada-base-10.md`, section 8).
//
// A whole second calculator, not a mode of the first one: it has its own
// session in Rust (`state.rs`, `Face::Simple`), its own input box, its own
// result, its own history and its own settings, and none of that is shared
// with the dozenal face. Switching tabs is navigation, never a reset.
//
// It owns no dozenal semantics either. It sends the text the user typed and
// paints the strings that come back. The two places where it would be
// tempting to compute something here are exactly the two it does not:
//
//   - reading base ten is `dz-parse::input_base`, gate arithmetic in Rust;
//   - chaining a result back into the box is `dz_api::chain_operand`, also
//     gate arithmetic in Rust. Seeding the dozenal text raw would be a
//     silently wrong number -- `144 + 11` shows `10↋`, and a box read as
//     decimal would continue from something else entirely.

import {
  calc,
  chainOperand,
  clearHistory,
  getSession,
  type CalcResponse,
  type GateCount,
  type Operation,
  type Session,
} from "./api";
import { display } from "./glyphs";
import { renderHistory } from "./history";

const FACE = "simple" as const;

function element<T extends HTMLElement>(id: string): T {
  const found = document.getElementById(id);
  if (!found) {
    throw new Error(`missing element #${id}`);
  }
  return found as T;
}

export interface SimpleHooks {
  /// Report a finished calculation so the shared research view can show
  /// its breakdown. The numbers are the ones this call already came back
  /// with; nothing is recomputed.
  onCalculation: (
    op: Operation,
    label: string,
    counts: GateCount | null,
    depth: number | null,
  ) => void;
  /// Jump to the research view -- what the gate line at the foot does.
  onShowResearch: () => void;
}

/// The keypad of this face: decimal digits, a decimal point, the four
/// operators, parentheses and the two controls. No `↊`, no `↋`, no
/// Humphrey point, no mode selector, no backend selector -- someone who
/// has never heard of base twelve has to be able to use it.
interface SimpleKey {
  label: string;
  insert?: string;
  action?: "equals" | "clear" | "backspace";
  variant: "digit" | "operator" | "control" | "accent";
}

const KEYS: SimpleKey[] = [
  { label: "C", action: "clear", variant: "control" },
  { label: "⌫", action: "backspace", variant: "control" },
  { label: "(", insert: "(", variant: "operator" },
  { label: ")", insert: ")", variant: "operator" },

  { label: "7", insert: "7", variant: "digit" },
  { label: "8", insert: "8", variant: "digit" },
  { label: "9", insert: "9", variant: "digit" },
  { label: "÷", insert: "/", variant: "operator" },

  { label: "4", insert: "4", variant: "digit" },
  { label: "5", insert: "5", variant: "digit" },
  { label: "6", insert: "6", variant: "digit" },
  { label: "×", insert: "*", variant: "operator" },

  { label: "1", insert: "1", variant: "digit" },
  { label: "2", insert: "2", variant: "digit" },
  { label: "3", insert: "3", variant: "digit" },
  { label: "−", insert: "-", variant: "operator" },

  { label: "0", insert: "0", variant: "digit" },
  { label: ".", insert: ".", variant: "digit" },
  { label: "=", action: "equals", variant: "accent" },
  { label: "+", insert: "+", variant: "operator" },
];

/// The same operator set the dozenal face chains on, read off the parser
/// (`crates/dz-parse/src/parser.rs`), minus nothing: an operator is an
/// operator in either input base.
const OPERATORS = new Set(["+", "-", "*", "/", "%", "^", "×", "÷"]);

/// The two dozenal digits, which this face refuses outright.
///
/// Rust would reject them anyway, with a better sentence than any this
/// file could write ("en base 10 no existe el dígito once"). Refusing the
/// keystroke is not a second copy of that rule: it is the keypad's own
/// promise, that a face with no dek key cannot end up with a dek in its
/// box. Anything else the user types still goes to Rust to be judged.
const DOZENAL_ONLY = new Set(["↊", "↋", ";"]);

let box: HTMLInputElement;
let output: HTMLOutputElement;
let notes: HTMLDivElement;
let gateLine: HTMLElement;
let gateTotal: HTMLElement;
let gateChain: HTMLElement;
let historyList: HTMLUListElement;
let hooks: SimpleHooks;

/// Whether a result is standing, so the next character can chain on it.
/// The seed itself is NOT held here: it is fetched from Rust at the moment
/// it is needed, because converting it is gate work and doing it eagerly
/// would spend it on every calculation the user never chains from.
let chainArmed = false;

function paintSession(session: Session): void {
  renderHistory(historyList, session.history, (input) => {
    box.value = input;
    chainArmed = false;
    box.focus();
  });
}

function paintNotes(response: CalcResponse): void {
  const badges: string[] = [];
  if (response.approximate) {
    badges.push(`aproximación a ${response.scale} dígitos`);
  }
  if (response.periodic_note) {
    badges.push("periódica en base 12");
  }
  notes.replaceChildren(
    ...badges.map((text) => {
      const badge = document.createElement("span");
      badge.className = "badge";
      badge.textContent = text;
      return badge;
    }),
  );
}

/// The hook of this face: a quiet line saying what the answer cost.
///
/// Someone types `144 + 11`, reads `10↋`, and this is what makes them ask
/// what happened underneath -- which is the research view, one click away.
function paintGateLine(counts: GateCount | null): void {
  if (counts === null) {
    gateLine.hidden = true;
    return;
  }
  gateLine.hidden = false;
  gateTotal.textContent = `${counts.total.toLocaleString("es")} puertas`;
  gateChain.textContent = "";
}

function showError(message: string): void {
  output.textContent = message;
  output.classList.add("error");
  notes.replaceChildren();
  gateLine.hidden = true;
  chainArmed = false;
}

async function evaluate(): Promise<void> {
  const source = box.value.trim();
  if (source.length === 0) {
    return;
  }
  const operation: Operation = { kind: "eval", expr: source };
  try {
    const response = await calc(FACE, operation);
    output.classList.remove("error");
    output.textContent = display(response.display);
    paintNotes(response);
    paintGateLine(response.gate_count);
    hooks.onCalculation(operation, source, response.gate_count, response.depth);
    paintSession(await getSession(FACE));
    chainArmed = true;
    // Last, once nothing else can still write to the box. The focus
    // matters: `=` on the keypad leaves it on the button, and the chain
    // offer is only reachable from the box.
    box.focus();
  } catch (error) {
    showError(String(error));
  }
}

/**
 * Consume one incoming character, and say whether it was swallowed.
 *
 * Asynchronous, unlike the dozenal face's version, and that is the whole
 * point of this file. There the result and the box speak the same
 * language, so seeding is text movement. Here the box is read as base ten
 * and the result is dozenal, so the seed has to be *converted back* --
 * `dz_api::chain_operand`, gate arithmetic, a round trip to Rust. The cost
 * of that trip is shown next to the calculation's own, separately,
 * because the user asked for it separately (`docs/10` section 5.2: the way
 * back is restoring division, and it is not cheap).
 */
async function chainOn(text: string): Promise<boolean> {
  if (!chainArmed) {
    return false;
  }
  chainArmed = false;
  if (!OPERATORS.has(text)) {
    return false;
  }
  try {
    const chained = await chainOperand(FACE);
    // Wrapped unless it is a bare run of decimal digits. A rational result
    // is `n/d`, and `155/2` seeded raw and followed by `^2` would parse as
    // `155/(2^2)` and quietly answer a different question. Over-wrapping is
    // free; under-wrapping is a wrong number nobody sees.
    const atomic = /^[0-9]+$/.test(chained.text);
    box.value = (atomic ? chained.text : `(${chained.text})`) + text;
    const caret = box.value.length;
    box.setSelectionRange(caret, caret);
    box.focus();
    gateChain.textContent = ` · ${chained.gate_count.total.toLocaleString(
      "es",
    )} puertas al reescribirlo en base 10`;
    return true;
  } catch (error) {
    showError(String(error));
    return false;
  }
}

function insert(text: string): void {
  const start = box.selectionStart ?? box.value.length;
  const end = box.selectionEnd ?? box.value.length;
  box.value = box.value.slice(0, start) + text + box.value.slice(end);
  const caret = start + text.length;
  box.setSelectionRange(caret, caret);
  box.focus();
}

function backspace(): void {
  const start = box.selectionStart ?? box.value.length;
  const end = box.selectionEnd ?? box.value.length;
  if (start !== end) {
    box.value = box.value.slice(0, start) + box.value.slice(end);
    box.setSelectionRange(start, start);
  } else if (start > 0) {
    box.value = box.value.slice(0, start - 1) + box.value.slice(start);
    box.setSelectionRange(start - 1, start - 1);
  }
  box.focus();
}

function renderKeypad(container: HTMLElement): void {
  container.replaceChildren(
    ...KEYS.map((spec) => {
      const key = document.createElement("button");
      key.type = "button";
      key.textContent = spec.label;
      key.className = `key ${spec.variant}`;
      if (spec.insert !== undefined) {
        key.dataset.insert = spec.insert;
      }
      if (spec.action) {
        key.dataset.action = spec.action;
      }
      return key;
    }),
  );
}

/// Repaint everything this face draws from its own session.
///
/// Called when the presentation alphabet changes: the pair is one visual
/// preference for one screen, applied to both sessions, and this face's
/// history and standing result are dozenal text that has to follow it.
export async function repaintSimpleFace(): Promise<void> {
  const session = await getSession(FACE);
  paintSession(session);
  const last = session.history[session.history.length - 1];
  if (last && !output.classList.contains("error")) {
    output.textContent = display(last.display);
  }
}

export async function initSimpleFace(next: SimpleHooks): Promise<void> {
  hooks = next;
  box = element<HTMLInputElement>("s-expression");
  output = element<HTMLOutputElement>("s-result");
  notes = element<HTMLDivElement>("s-notes");
  gateLine = element("s-gate-line");
  gateTotal = element("s-gate-total");
  gateChain = element("s-gate-chain");
  historyList = element<HTMLUListElement>("s-history-list");

  renderKeypad(element("s-keypad"));

  element("s-keypad").addEventListener("click", (event) => {
    const target = (event.target as HTMLElement).closest("button");
    if (!target) {
      return;
    }
    const text = target.dataset.insert;
    if (text !== undefined) {
      void (async () => {
        if (await chainOn(text)) {
          return;
        }
        insert(text);
      })();
      return;
    }
    switch (target.dataset.action) {
      case "equals":
        void evaluate();
        break;
      case "clear":
        box.value = "";
        output.textContent = "0";
        output.classList.remove("error");
        notes.replaceChildren();
        gateLine.hidden = true;
        chainArmed = false;
        box.focus();
        break;
      case "backspace":
        chainArmed = false;
        backspace();
        break;
    }
  });

  box.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      void evaluate();
      return;
    }
    if (event.key.length !== 1 || event.ctrlKey || event.metaKey || event.altKey) {
      // A modifier combination is a shortcut, not a character.
      return;
    }
    if (DOZENAL_ONLY.has(event.key)) {
      event.preventDefault();
      return;
    }
    if (chainArmed) {
      // The offer is spent by the first character either way; only an
      // operator seeds. `chainOn` is async, so the keystroke is always
      // stopped here and re-inserted afterwards when it was not a chain,
      // rather than racing the round trip.
      event.preventDefault();
      const typed = event.key;
      void (async () => {
        if (!(await chainOn(typed))) {
          insert(typed);
        }
      })();
    }
  });

  // Pasting and dictation do not fire `keydown`, so the refusal is
  // re-applied to whatever ends up in the box.
  box.addEventListener("input", () => {
    const cleaned = [...box.value].filter((c) => !DOZENAL_ONLY.has(c)).join("");
    if (cleaned !== box.value) {
      const caret = Math.min(box.selectionStart ?? cleaned.length, cleaned.length);
      box.value = cleaned;
      box.setSelectionRange(caret, caret);
    }
    chainArmed = false;
  });

  element("s-gates-link").addEventListener("click", () => hooks.onShowResearch());
  element("s-history-clear").addEventListener("click", async () => {
    paintSession(await clearHistory(FACE));
  });

  paintSession(await getSession(FACE));
}
