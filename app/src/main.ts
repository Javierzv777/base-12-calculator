// Wiring. This file owns no dozenal semantics: it moves text between an
// input box and Rust, and paints what comes back.

import {
  calc,
  clearHistory,
  getSession,
  listFunctions,
  memoryClear,
  memoryStore,
  setBackend,
  setMode,
  setScale,
  type CalcResponse,
  type Operation,
  type Session,
} from "./api";
import { initAlphabetModal, restoreStoredAlphabet } from "./alphabet";
// A separate tool that shares the repository but not the calculator: see
// `docs/08-conversion-base10.md` and the header of `base10.ts`.
import { convertBase10Text, convertBase12Text, type Base10Style } from "./base10";
import { display, initGlyphMode } from "./glyphs";
import { renderHistory } from "./history";
import { renderKeypad, renderScientificPad } from "./keypad";
import { initResearch, paintResearchGates } from "./research";

function element<T extends HTMLElement>(id: string): T {
  const found = document.getElementById(id);
  if (!found) {
    throw new Error(`missing element #${id}`);
  }
  return found as T;
}

const expression = element<HTMLInputElement>("expression");
const result = element<HTMLOutputElement>("result");
const notes = element<HTMLDivElement>("notes");
const keypad = element<HTMLDivElement>("keypad");
const science = element<HTMLDivElement>("science");
const modeSelect = element<HTMLSelectElement>("mode");
const backendSelect = element<HTMLSelectElement>("backend");
const scaleInput = element<HTMLInputElement>("scale");
const memoryValue = element<HTMLSpanElement>("memory-value");
const historyList = element<HTMLUListElement>("history-list");

let session: Session | null = null;
let lastResult = "";
// The last thing actually evaluated, kept so the research panel can re-run
// it against the three backends without asking the user to retype it.
let lastOperation: Operation | null = null;
let lastLabel = "";
// Paints the alphabet dialog and the sidebar button from a session.
// Assigned in `boot`; the modal owns its own DOM.
let syncAlphabet: (next: Session) => void = () => {};

function fillSelect(select: HTMLSelectElement, values: string[], current: string): void {
  select.replaceChildren(
    ...values.map((value) => {
      const option = document.createElement("option");
      option.value = value;
      option.textContent = value;
      option.selected = value === current;
      return option;
    }),
  );
}

function paintSession(next: Session): void {
  session = next;
  // The keypad carries two configurable keys and the history holds
  // canonical text that Rust just re-rendered, so both are repainted from
  // every session -- changing the alphabet must not leave a list half in
  // one spelling and half in the other (`docs/07` section 3.5).
  syncAlphabet(next);
  renderKeypad(keypad, next.alphabet_ten, next.alphabet_eleven);
  fillSelect(modeSelect, next.modes, next.mode);
  fillSelect(backendSelect, next.backends, next.backend);
  scaleInput.value = String(next.scale);
  memoryValue.textContent = next.memory ? display(next.memory) : "vacía";
  renderHistory(historyList, next.history, (input) => {
    expression.value = input;
    expression.focus();
  });
}

function paintGates(response: CalcResponse | null): void {
  const counts = response?.gate_count ?? null;
  const cells: Array<[string, number]> = [
    ["g-cycle", counts?.cycle ?? 0],
    ["g-min", counts?.min ?? 0],
    ["g-max", counts?.max ?? 0],
    ["g-lit", counts?.lit ?? 0],
    ["g-thr", counts?.thr ?? 0],
    ["g-total", counts?.total ?? 0],
    ["g-depth", response?.depth ?? 0],
  ];
  for (const [id, value] of cells) {
    element(id).textContent = value.toLocaleString("es");
  }
  // The research view shows the same numbers with more room around them.
  // One painter, two destinations -- the two tables cannot drift.
  paintResearchGates(counts, response?.depth ?? null, lastLabel);
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

function showError(message: string): void {
  result.textContent = message;
  result.classList.add("error");
  notes.replaceChildren();
}

// ---------------------------------------------------------------------------
// Chaining on the result (`docs/09-encadenado-de-resultado.md`).
//
// After a successful evaluation, the FIRST character that arrives decides: an
// operator replaces the whole box with the result and that operator (`5+3`
// Enter, `*` -> `8*`); anything else just types, and the offer is gone either
// way. Seeding is text movement only -- nothing here calls `calc`.
// ---------------------------------------------------------------------------

/**
 * The binary operators of the language, read off the parser
 * (`crates/dz-parse/src/parser.rs`): `+ -`, then `* / %`, then `^`. The two
 * unicode spellings the lexer also accepts for times and divide are in the set
 * too, so a chain never depends on which of two synonyms was typed.
 */
const OPERATORS = new Set(["+", "-", "*", "/", "%", "^", "×", "÷"]);

/** The text to seed with, ready to write, or `null` when nothing is armed. */
let chainSeed: string | null = null;
// The value the box had when the offer was armed. Same whitelist shape as the
// base-10 undo (`docs/08` section 2.3): the offer belongs to THIS text, and any
// other value the box takes -- restoring from history, a conversion, a route
// added later -- retires it, instead of us trying to list the ways it can
// change. Programmatic writes fire no event, so a frame loop does the comparing.
let chainAnchor: string | null = null;
let chainWatching = 0;

function disarmChain(): void {
  chainSeed = null;
  chainAnchor = null;
  if (chainWatching !== 0) {
    cancelAnimationFrame(chainWatching);
    chainWatching = 0;
  }
}

/**
 * Whether a shown result can be seeded bare.
 *
 * Only a plain number qualifies: digits, the two configured symbols for ten and
 * eleven (in whatever spelling this machine can actually paint) and the
 * Humphrey point. Everything else -- a fraction's slash, a sign, an exponent --
 * gets parentheses, because `583/2` seeded raw and followed by `^2` parses as
 * `583/(2^2)` and quietly answers a different question. Over-wrapping is free;
 * under-wrapping is a wrong number nobody sees.
 */
function atomicResult(text: string): boolean {
  const symbols = new Set([";"]);
  for (const symbol of [session?.alphabet_ten ?? "↊", session?.alphabet_eleven ?? "↋"]) {
    symbols.add(symbol);
    symbols.add(display(symbol));
  }
  if (text.length === 0) {
    return false;
  }
  for (const character of text) {
    if (character >= "0" && character <= "9") {
      continue;
    }
    if (!symbols.has(character)) {
      return false;
    }
  }
  return true;
}

/**
 * Arm the offer with the text the user is READING (`display()` output, in the
 * alphabet of `docs/07`), not the canonical `lastResult`.
 *
 * The leading `~` that marks an approximation (`dz-api` adds it) is dropped:
 * the lexer rejects that character outright, so seeding it would leave a box
 * that cannot evaluate. The chain therefore continues from the truncated value,
 * which is what the badge `paintNotes` shows already says it is.
 */
function armChain(shown: string): void {
  const bare = shown.replace(/^~/, "").trim();
  if (bare.length === 0) {
    disarmChain();
    return;
  }
  chainSeed = atomicResult(bare) ? bare : `(${bare})`;
  chainAnchor = expression.value;
  const tick = (): void => {
    chainWatching = 0;
    if (chainAnchor === null) {
      return;
    }
    if (expression.value !== chainAnchor) {
      disarmChain();
      return;
    }
    chainWatching = requestAnimationFrame(tick);
  };
  chainWatching = requestAnimationFrame(tick);
}

/**
 * Consume one incoming character. Returns `true` when it was swallowed by a
 * seed, so the caller must not also type it.
 *
 * Every route that can put a character in the box funnels through here: the
 * physical keyboard by `keydown`, and the on-screen keypad, the scientific pad
 * and memory recall by `insert()`. A `keydown` handler alone would cover half
 * the ways in -- the mistake `docs/08` section 2.3 already avoided once.
 */
function chainOn(text: string): boolean {
  const seed = chainSeed;
  if (seed === null) {
    return false;
  }
  disarmChain();
  if (!OPERATORS.has(text)) {
    return false;
  }
  expression.value = seed + text;
  const caret = expression.value.length;
  expression.setSelectionRange(caret, caret);
  expression.focus();
  return true;
}

async function evaluate(): Promise<void> {
  const source = expression.value.trim();
  if (source.length === 0) {
    return;
  }
  const operation: Operation = { kind: "eval", expr: source };
  try {
    const response = await calc(operation);
    result.classList.remove("error");
    const shown = display(response.display);
    result.textContent = shown;
    lastResult = response.display;
    lastOperation = operation;
    lastLabel = source;
    paintNotes(response);
    paintGates(response);
    paintSession(await getSession());
    // Last, once nothing else can still write to the box: the offer is
    // anchored to the value standing there right now.
    armChain(shown);
  } catch (error) {
    // An error leaves nothing to chain from.
    disarmChain();
    showError(String(error));
  }
}

function insert(text: string): void {
  if (chainOn(text)) {
    return;
  }
  const start = expression.selectionStart ?? expression.value.length;
  const end = expression.selectionEnd ?? expression.value.length;
  expression.value = expression.value.slice(0, start) + text + expression.value.slice(end);
  const caret = start + text.length;
  expression.setSelectionRange(caret, caret);
  expression.focus();
}

function backspace(): void {
  const start = expression.selectionStart ?? expression.value.length;
  const end = expression.selectionEnd ?? expression.value.length;
  if (start !== end) {
    expression.value = expression.value.slice(0, start) + expression.value.slice(end);
    expression.setSelectionRange(start, start);
  } else if (start > 0) {
    expression.value = expression.value.slice(0, start - 1) + expression.value.slice(start);
    expression.setSelectionRange(start - 1, start - 1);
  }
  expression.focus();
}

function wireKeys(container: HTMLElement): void {
  container.addEventListener("click", (event) => {
    const target = (event.target as HTMLElement).closest("button");
    if (!target) {
      return;
    }
    const insertText = target.dataset.insert;
    if (insertText !== undefined) {
      insert(insertText);
      return;
    }
    switch (target.dataset.action) {
      case "equals":
        void evaluate();
        break;
      case "clear":
        expression.value = "";
        result.textContent = "0";
        result.classList.remove("error");
        notes.replaceChildren();
        expression.focus();
        break;
      case "backspace":
        backspace();
        break;
    }
  });
}

/// Two top-level views the user alternates between, not a panel that
/// swaps the calculator's sidebar: the research widgets need the whole
/// width (a three-backend table, and a step viewer that lists a dozen
/// digit positions side by side). "Calculadora" is the default.
function showView(name: string): void {
  for (const view of ["calculator", "research"]) {
    const active = view === name;
    element(`view-${view}`).classList.toggle("hidden", !active);
    const tab = element(`tab-${view}`);
    tab.classList.toggle("active", active);
    tab.setAttribute("aria-selected", String(active));
  }
  if (name === "calculator") {
    expression.focus();
  }
}

// ---------------------------------------------------------------------------
// Connection point for the base conversion helper -- a SEPARATE TOOL, not part
// of the calculator (`docs/08-conversion-base10.md`, section 1.2). Everything
// below is UI: it hands `base10.ts` a string and, only when the user clicks,
// writes the answer into the input's `value`. No result, no history entry, no
// gate count and no memory register ever comes from here, and `base10.ts` calls
// nothing of the calculator's in return.
//
// One control, two directions (section 2.4): a button that converts the box on
// demand, and a swap next to it that flips which way it reads -- `10→12` or
// `12→10`, the way a translator swaps its two languages. There is no popup and
// nothing appears by itself: the tool only ever speaks after a click.
// ---------------------------------------------------------------------------
function initBase10Tool(): void {
  const convertButton = element<HTMLButtonElement>("base10-convert");
  const swapButton = element<HTMLButtonElement>("base10-swap");
  const undoButton = element<HTMLButtonElement>("base10-undo");
  const status = element<HTMLSpanElement>("base10-status");
  const selectionLabel = element<HTMLSpanElement>("base10-selection");

  /** Which way the button reads the box. Starts on the direction of section 2. */
  let direction: "to12" | "to10" = "to12";
  // Set after a conversion, so undo can restore exactly what was there before.
  let undoTo: string | null = null;
  // The exact text the undo offer belongs to: the conversion's own output.
  //
  // `docs/08` section 2.3 is a WHITELIST, not a blacklist: undo is armed only by
  // our own write, and retires on ANY other value the box takes. Listing the
  // ways a value can change (typing, deleting, pasting, cutting, the dozenal
  // keypad, restoring from history, dragging text in, dictation, autofill, a
  // route added six months from now) always leaves some out, and every omission
  // is an undo that silently overwrites the user's work. Compared against the
  // value itself, the rule has no holes.
  let anchor: string | null = null;
  // Handle of the frame loop that does the comparing, 0 when it is not running.
  // Programmatic writes to `value` fire no event at all -- not `input`, not
  // anything -- so watching the value is the only hole-free way to see them. It
  // runs only while undo is armed, and stops itself as soon as it is not.
  let watching = 0;

  function stopWatching(): void {
    if (watching !== 0) {
      cancelAnimationFrame(watching);
      watching = 0;
    }
  }

  /** Put the tool back to its resting state: no undo, nothing said. */
  function retire(): void {
    undoTo = null;
    anchor = null;
    stopWatching();
    undoButton.hidden = true;
    status.textContent = "";
    status.title = "";
  }

  function watch(): void {
    stopWatching();
    const tick = (): void => {
      watching = 0;
      if (anchor === null) {
        return;
      }
      if (expression.value !== anchor) {
        // Someone other than us changed the box, so restoring would overwrite
        // work the user did on top of the conversion.
        retire();
        return;
      }
      watching = requestAnimationFrame(tick);
    };
    watching = requestAnimationFrame(tick);
  }

  function style(): Base10Style {
    // Read off the live session so the text is spelled the way the user has the
    // interface configured: the alphabet (`docs/07`), and the mode and scale
    // that decide what a non-terminating fraction looks like (`docs/08` section
    // 3.2). Plain values cross the boundary; the module never sees a `Session`,
    // and never asks anyone for one.
    const mode = session?.mode;
    return {
      ten: session?.alphabet_ten ?? "↊",
      eleven: session?.alphabet_eleven ?? "↋",
      mode: mode === "rational" || mode === "dfp" ? mode : "fixed",
      scale: session?.scale ?? 12,
    };
  }

  /** A short line in the row, and the whole story in its tooltip. */
  function say(short: string, detail: string): void {
    status.textContent = short;
    status.title = detail;
  }

  /**
   * The selected range inside the box, or `null` when nothing is selected
   * (docs/08 section 2.5).
   *
   * `selectionStart`/`selectionEnd` are properties of the input, not of the
   * document's focus, so they survive the blur that pressing a button causes --
   * which is exactly why the click handler reads them FIRST, before anything
   * else moves the caret.
   */
  function selectedRange(): { start: number; end: number } | null {
    const start = expression.selectionStart;
    const end = expression.selectionEnd;
    if (start === null || end === null || end <= start) {
      return null;
    }
    return { start, end };
  }

  /**
   * The label is governed by the STATE of the selection, never by a list of the
   * events that can change it -- the same lesson the whitelist of section 2.3
   * left. Dragging with the mouse, `shift`+arrows, `ctrl`+`a`, a double click,
   * the system's own edit menu and whatever route the platform adds next all
   * end in the same place: the range on the element.
   */
  let selectionShown = false;
  function paintSelection(): void {
    const shown = selectedRange() !== null;
    if (shown !== selectionShown) {
      selectionShown = shown;
      selectionLabel.hidden = !shown;
    }
  }

  // `selectionchange` on the document is the modern route and Chromium (so
  // WebView2) does fire it for `<input>`, but it is only the trigger: what is
  // read is always the range itself, so a route that fails to fire it is
  // caught by the frame loop below instead of being missed.
  document.addEventListener("selectionchange", paintSelection);
  // Cheap belt to that braces: two integer comparisons a frame. It costs
  // nothing measurable and it means the label cannot get stuck showing a
  // selection that is no longer there.
  const followSelection = (): void => {
    paintSelection();
    requestAnimationFrame(followSelection);
  };
  requestAnimationFrame(followSelection);

  function paintDirection(): void {
    const to12 = direction === "to12";
    convertButton.textContent = to12 ? "10→12" : "12→10";
    convertButton.title = to12
      ? "Convertir el contenido de base 10 a base 12 (herramienta aparte)"
      : "Convertir el contenido de base 12 a base 10 (herramienta aparte)";
  }

  swapButton.addEventListener("click", () => {
    direction = direction === "to12" ? "to10" : "to12";
    paintDirection();
    // The offer belonged to the other direction's click, and the message to the
    // other direction's outcome. Neither survives the flip.
    retire();
    expression.focus();
  });

  convertButton.addEventListener("click", () => {
    // Read the range BEFORE anything else: this click already blurred the box,
    // and every later step (`focus()`, writing `value`) can move the caret.
    const range = selectedRange();
    const previous = expression.value;
    // With a selection the scope is that slice, handed to the module as a text
    // of its own -- `base10.ts` does not change (section 2.5).
    const source = range === null ? previous : previous.slice(range.start, range.end);
    const conversion =
      direction === "to12"
        ? convertBase10Text(source, style())
        : convertBase12Text(source, style());
    const target = direction === "to12" ? "12" : "10";

    /** Give the box back its focus and, when there was one, its selection. */
    const restore = (start: number, end: number): void => {
      expression.focus();
      expression.setSelectionRange(start, end);
    };

    if (!conversion.changed) {
      retire();
      say(
        "nada que convertir",
        range === null
          ? `el texto se lee igual en base ${target}, así que no se cambió nada`
          : `la selección se lee igual en base ${target}, así que no se cambió nada`,
      );
      if (range === null) {
        expression.focus();
      } else {
        restore(range.start, range.end);
      }
      return;
    }
    expression.value =
      range === null
        ? conversion.text
        : previous.slice(0, range.start) + conversion.text + previous.slice(range.end);
    const detail: string[] = [`convertido a base ${target}`];
    if (range !== null) {
      // Say it, so a selection cutting a number in half does not read as a bug:
      // `12 + 11` with `12 + 1` selected leaves `10 + 11`, because the halves
      // convert on their own (section 2.5).
      detail.push("solo se convirtió la parte seleccionada");
    }
    if (conversion.keptDozenal) {
      // A text with dozenal glyphs is not readable as base 10 as a whole, so
      // say plainly that only the decimal runs moved (section 2.0).
      detail.push("los símbolos dozenales se dejaron intactos");
    }
    if (conversion.madeFraction) {
      detail.push("un decimal no termina en esa base y se escribió como fracción exacta");
    }
    if (conversion.madeTruncated) {
      // The only place the preview's `…` is ever shown, now that the popup is
      // gone: it says the expansion goes on, and it never reaches the box.
      detail.push(`un decimal no termina en esa base y se truncó a la escala → ${conversion.preview}`);
    }
    say(
      range === null ? `base ${target}` : `base ${target} (selección)`,
      `${detail.join("; ")}.`,
    );
    // Armed here, after the write, because the value at this instant is what
    // the offer is anchored to (section 2.3). Both ends are the WHOLE value:
    // undo does not know, and does not need to know, that this was partial.
    undoTo = previous;
    anchor = expression.value;
    undoButton.hidden = false;
    watch();
    if (range === null) {
      expression.focus();
    } else {
      // Put the selection back over the converted text -- a different length
      // than what was selected -- so the user sees exactly what changed.
      restore(range.start, range.start + conversion.text.length);
    }
  });

  undoButton.addEventListener("click", () => {
    // Last line of the whitelist: an edit that landed after the last frame and
    // before this click still retires the offer, instead of overwriting it.
    if (undoTo === null || (anchor !== null && expression.value !== anchor)) {
      retire();
      expression.focus();
      return;
    }
    expression.value = undoTo;
    retire();
    expression.focus();
  });

  // Typing answers immediately; the frame loop above covers everything else.
  expression.addEventListener("input", retire);

  paintDirection();
}

async function boot(): Promise<void> {
  initGlyphMode();
  syncAlphabet = initAlphabetModal({
    onSession: (next) => {
      paintSession(next);
      // The big result is the one piece of dozenal text not driven by the
      // session, so it needs an explicit repaint: Rust just re-rendered
      // the history with the new alphabet, and the last entry is what the
      // display is showing.
      const last = next.history[next.history.length - 1];
      if (last && !result.classList.contains("error")) {
        result.textContent = display(last.display);
        lastResult = last.display;
      }
    },
  });
  renderScientificPad(science, await listFunctions());
  wireKeys(keypad);
  wireKeys(science);

  // A stored preference is re-validated by Rust before it is applied; a
  // pair the current catalog no longer allows is dropped silently and
  // the session stays canonical.
  paintSession((await restoreStoredAlphabet()) ?? (await getSession()));
  paintGates(null);

  for (const view of ["calculator", "research"]) {
    element(`tab-${view}`).addEventListener("click", () => showView(view));
  }
  initResearch({
    lastOperation: () => lastOperation,
    onError: showError,
  });

  expression.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      void evaluate();
      return;
    }
    // The physical half of `chainOn` (`docs/09` section 3.3). A modifier
    // combination is a shortcut, not a character, so it is left alone; a real
    // character either seeds or spends the offer.
    if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      if (chainOn(event.key)) {
        event.preventDefault();
      }
    }
  });
  // Anything else that edits the box -- deleting, pasting, dictation -- spends
  // the offer at once, without waiting for the watcher's next frame, so a fast
  // `2` then `+` reads as `8*2+` and never seeds twice.
  expression.addEventListener("input", disarmChain);

  modeSelect.addEventListener("change", async () => {
    paintSession(await setMode(modeSelect.value));
  });
  backendSelect.addEventListener("change", async () => {
    paintSession(await setBackend(backendSelect.value));
  });
  scaleInput.addEventListener("change", async () => {
    const digits = Number.parseInt(scaleInput.value, 10);
    if (!Number.isFinite(digits)) {
      return;
    }
    try {
      paintSession(await setScale(digits));
    } catch (error) {
      showError(String(error));
      if (session) {
        scaleInput.value = String(session.scale);
      }
    }
  });

  element("mem-store").addEventListener("click", async () => {
    if (lastResult.length > 0) {
      // No argument: Rust stores its own canonical copy of the last
      // result, so the register repaints when the alphabet changes.
      paintSession(await memoryStore());
    }
  });
  element("mem-recall").addEventListener("click", () => {
    if (session?.memory) {
      insert(session.memory);
    }
  });
  element("mem-clear").addEventListener("click", async () => {
    paintSession(await memoryClear());
  });
  element("history-clear").addEventListener("click", async () => {
    paintSession(await clearHistory());
  });

  initBase10Tool();

  expression.focus();
}

void boot();
