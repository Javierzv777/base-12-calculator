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
  type Session,
} from "./api";
import { display, initGlyphMode } from "./glyphs";
import { renderHistory } from "./history";
import { renderKeypad, renderScientificPad } from "./keypad";

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
  ];
  for (const [id, value] of cells) {
    element(id).textContent = value.toLocaleString("es");
  }
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

async function evaluate(): Promise<void> {
  const source = expression.value.trim();
  if (source.length === 0) {
    return;
  }
  try {
    const response = await calc({ kind: "eval", expr: source });
    result.classList.remove("error");
    result.textContent = display(response.display);
    lastResult = response.display;
    paintNotes(response);
    paintGates(response);
    paintSession(await getSession());
  } catch (error) {
    showError(String(error));
  }
}

function insert(text: string): void {
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

async function boot(): Promise<void> {
  initGlyphMode();
  renderKeypad(keypad);
  renderScientificPad(science, await listFunctions());
  wireKeys(keypad);
  wireKeys(science);

  paintSession(await getSession());
  paintGates(null);

  expression.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      void evaluate();
    }
  });

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
      paintSession(await memoryStore(lastResult));
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

  expression.focus();
}

void boot();
