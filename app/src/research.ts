// The "Investigación" view: gate cost and depth of the last operation, the
// three-backend comparison, the step-by-step circuit viewer, and CSV
// export.
//
// This file owns no dozenal semantics. Everything it prints arrived from
// Rust as text (`api.ts`), including the CSV, which it never even sees --
// it only forwards the path the user picked in the native dialog. Its own
// arithmetic is on array indices and step counters, never on a digit.

import { save } from "@tauri-apps/plugin-dialog";
import {
  compareBackends,
  exportCsv,
  traceOperation,
  type BackendCost,
  type Comparison,
  type Face,
  type Operation,
  type Trace,
  type TraceKind,
  type TraceStep,
} from "./api";
import { display } from "./glyphs";

function element<T extends HTMLElement>(id: string): T {
  const found = document.getElementById(id);
  if (!found) {
    throw new Error(`missing element #${id}`);
  }
  return found as T;
}

function text(tag: string, className: string, value: string): HTMLElement {
  const node = document.createElement(tag);
  node.className = className;
  node.textContent = value;
  return node;
}

/// The name each backend gets in the comparison table, plus the honest
/// caveat for the oracle. `docs/02-crt-vs-mvl.md` earns CRT the same kind
/// of label: showing the number is right, letting it read as a verdict is
/// not.
const BACKEND_NOTE: Record<string, string> = {
  mvl: "Post-álgebra 12-valuada, el backend de producción",
  crt: "Z12 ≅ Z4×Z3; reconstruye el orden en cada comparación",
  table: "oráculo por tabla — no es un backend de producción",
};

let currentTrace: Trace | null = null;
let currentSteps: TraceStep[] = [];
let cursor = -1; // -1 = show everything

export interface ResearchHooks {
  /// The operation last evaluated, and by which of the two calculators,
  /// so "comparar backends" re-runs exactly that instead of asking the
  /// user to retype it. The face travels with the operation because the
  /// operand *text* only means what it showed when it is re-read in the
  /// input base of the face that produced it (`docs/10`).
  lastOperation: () => { face: Face; op: Operation } | null;
  onError: (message: string) => void;
}

// --- last operation ---------------------------------------------------

export function paintResearchGates(
  counts: { cycle: number; min: number; max: number; lit: number; thr: number; total: number } | null,
  depth: number | null,
  label: string,
): void {
  const cells: Array<[string, number]> = [
    ["r-cycle", counts?.cycle ?? 0],
    ["r-min", counts?.min ?? 0],
    ["r-max", counts?.max ?? 0],
    ["r-lit", counts?.lit ?? 0],
    ["r-thr", counts?.thr ?? 0],
    ["r-total", counts?.total ?? 0],
    ["r-depth", depth ?? 0],
  ];
  for (const [id, value] of cells) {
    element(id).textContent = value.toLocaleString("es");
  }
  element("r-expression").textContent = label.length > 0 ? display(label) : "todavía no has calculado nada";
}

// --- backend comparison ----------------------------------------------

function renderComparison(comparison: Comparison): void {
  const table = document.createElement("table");
  table.className = "grid comparison";

  const head = document.createElement("thead");
  const headRow = document.createElement("tr");
  headRow.appendChild(text("th", "", ""));
  for (const cost of comparison.costs) {
    const cell = document.createElement("th");
    cell.appendChild(text("span", "backend-name", cost.backend));
    if (cost.oracle) {
      cell.appendChild(text("span", "badge warn", "oráculo"));
    }
    cell.title = BACKEND_NOTE[cost.backend] ?? "";
    headRow.appendChild(cell);
  }
  head.appendChild(headRow);
  table.appendChild(head);

  const body = document.createElement("tbody");
  const rows: Array<[string, (c: BackendCost) => number]> = [
    ["cycle", (c) => c.gate_count.cycle],
    ["min", (c) => c.gate_count.min],
    ["max", (c) => c.gate_count.max],
    ["lit", (c) => c.gate_count.lit],
    ["thr", (c) => c.gate_count.thr],
    ["total", (c) => c.gate_count.total],
    ["profundidad", (c) => c.depth],
    ["reconstrucciones", (c) => c.reconstruction_ops],
  ];
  for (const [name, pick] of rows) {
    const row = document.createElement("tr");
    if (name === "total" || name === "profundidad" || name === "reconstrucciones") {
      row.className = "total";
    }
    row.appendChild(text("th", "", name));
    for (const cost of comparison.costs) {
      row.appendChild(text("td", "", pick(cost).toLocaleString("es")));
    }
    body.appendChild(row);
  }
  table.appendChild(body);

  const host = element("compare-table");
  host.replaceChildren(table);
  // The verdict arrives already written by `dz_api::verdict`. Nothing is
  // composed here on purpose: the terminal prints the same sentence, and
  // a second author would be a second answer (`docs/06`, section 7).
  const note = document.createElement("p");
  note.className = "verdict";
  note.textContent = comparison.verdict?.summary ?? "";
  host.appendChild(note);
  element("compare-status").textContent = `Resultado común: ${display(comparison.display)}`;
  element<HTMLButtonElement>("compare-export").disabled = false;
}

// --- step-by-step viewer ---------------------------------------------

function stepTable(steps: TraceStep[], offset: number): HTMLElement {
  const table = document.createElement("table");
  table.className = "grid steps";

  const head = document.createElement("thead");
  const headRow = document.createElement("tr");
  for (const name of ["pos", "a", "b", "acarreo→", "suma", "→acarreo"]) {
    headRow.appendChild(text("th", "", name));
  }
  head.appendChild(headRow);
  table.appendChild(head);

  const body = document.createElement("tbody");
  steps.forEach((step, i) => {
    const row = document.createElement("tr");
    // `offset + i` is the step's index in the flattened playback order:
    // list bookkeeping, not a dozenal value.
    if (cursor >= 0 && offset + i === cursor) {
      row.className = "current";
    } else if (cursor >= 0 && offset + i > cursor) {
      row.className = "pending";
    }
    row.appendChild(text("td", "", String(step.position)));
    row.appendChild(text("td", "digit", display(step.a)));
    row.appendChild(text("td", "digit", display(step.b)));
    row.appendChild(text("td", "digit carry", display(step.carry_in)));
    row.appendChild(text("td", "digit sum", display(step.sum)));
    row.appendChild(text("td", "digit carry", display(step.carry_out)));
    body.appendChild(row);
  });
  table.appendChild(body);
  return table;
}

function stageBlock(title: string, note: string, value?: string): HTMLElement {
  const block = document.createElement("div");
  block.className = "stage";
  block.appendChild(text("h3", "", title));
  if (value !== undefined) {
    block.appendChild(text("p", "digits", display(value)));
  }
  block.appendChild(text("p", "muted small-note", note));
  return block;
}

/// Flatten a trace into the single ordered list the playback controls walk.
function flatten(trace: Trace): TraceStep[] {
  switch (trace.kind) {
    case "add":
      return trace.steps;
    case "sub":
      return [...trace.complement_steps, ...trace.add_steps, ...trace.decode_steps];
    case "mul":
      return trace.rows.flatMap((row) => row.accumulate_steps);
  }
}

function renderTrace(trace: Trace): void {
  const host = element("trace-output");
  const parts: HTMLElement[] = [];
  let offset = 0;

  if (trace.kind === "add") {
    parts.push(stageBlock(`${display(trace.a)} + ${display(trace.b)}`, "suma con acarreo en cadena, dígito a dígito"));
    parts.push(stepTable(trace.steps, offset));
    parts.push(stageBlock("Resultado", "el último acarreo saliente es el dígito extra", trace.result));
  } else if (trace.kind === "sub") {
    parts.push(
      stageBlock(
        `${display(trace.a)} − ${display(trace.b)}`,
        "No existe un circuito restador: restar es complementar y sumar. Los pasos de abajo son literalmente lo que se ejecutó.",
      ),
    );
    parts.push(
      stageBlock("1. Complemento a 11 de B", "posicional, dígito a dígito, sin acarreos", trace.ones_complement),
    );
    parts.push(stageBlock("2. …+1 → complemento a 12 de B", "esta suma sí tiene acarreo:"));
    parts.push(stepTable(trace.complement_steps, offset));
    offset += trace.complement_steps.length;
    parts.push(stageBlock("Complemento a 12 de B", "", trace.radix_complement));
    parts.push(stageBlock("3. A + complemento", "la suma de verdad, la que produce el resultado:"));
    parts.push(stepTable(trace.add_steps, offset));
    offset += trace.add_steps.length;
    parts.push(
      stageBlock(
        "Suma cruda",
        trace.borrowed
          ? "A < B: la suma quedó en forma complementada, hay que decodificarla"
          : "A ≥ B: sobra el dígito superior, el resto es la respuesta",
        trace.raw_sum,
      ),
    );
    if (trace.borrowed) {
      parts.push(stageBlock("4. Complemento a 11 de la suma", "", trace.decode_ones_complement));
      parts.push(stageBlock("5. …+1 → magnitud del resultado", ""));
      parts.push(stepTable(trace.decode_steps, offset));
      offset += trace.decode_steps.length;
    }
    parts.push(stageBlock("Resultado", trace.borrowed ? "negativo, porque A < B" : "", trace.result));
  } else {
    parts.push(
      stageBlock(
        `${display(trace.a)} × ${display(trace.b)}`,
        "Una fila parcial por cada dígito del multiplicador; cada fila se acumula con una suma en cadena propia.",
      ),
    );
    for (const row of trace.rows) {
      const group = document.createElement("div");
      group.className = "mul-row";
      group.appendChild(
        stageBlock(
          `Dígito ${row.index} del multiplicador: ${display(row.multiplier_digit)}`,
          `fila parcial ${display(row.row)}, desplazada ${row.index} posiciones → ${display(row.shifted)}`,
        ),
      );
      group.appendChild(text("p", "muted small-note", "acumulando:"));
      group.appendChild(stepTable(row.accumulate_steps, offset));
      offset += row.accumulate_steps.length;
      group.appendChild(text("p", "digits", display(row.accumulated)));
      parts.push(group);
    }
    parts.push(stageBlock("Producto", "el acumulado de la última fila", trace.result));
  }

  host.replaceChildren(...parts);
  element("trace-position").textContent =
    cursor < 0
      ? `${currentSteps.length} pasos`
      : `paso ${cursor + 1} de ${currentSteps.length}`;
}

function setPlaybackEnabled(enabled: boolean): void {
  for (const id of [
    "trace-prev",
    "trace-next",
    "trace-all",
    "trace-export",
    "trace-summary-export",
  ]) {
    element<HTMLButtonElement>(id).disabled = !enabled;
  }
}

function redraw(): void {
  if (currentTrace) {
    renderTrace(currentTrace);
  }
}

// --- CSV export -------------------------------------------------------

async function exportWith(what: string, suggested: string, onError: (m: string) => void): Promise<void> {
  try {
    const path = await save({
      title: "Guardar CSV",
      defaultPath: suggested,
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!path) {
      return; // the user cancelled
    }
    await exportCsv(what, path);
  } catch (error) {
    onError(String(error));
  }
}

// --- wiring -----------------------------------------------------------

export function initResearch(hooks: ResearchHooks): void {
  // The form seeds an example operand in the markup. It is the one place
  // the HTML spells a dozenal digit out, so it goes through the same
  // fallback as everything else rather than showing a box on a machine
  // whose font lacks U+218A/U+218B.
  const seed = element<HTMLInputElement>("trace-a");
  seed.value = display(seed.value);

  element("compare-run").addEventListener("click", async () => {
    const op = hooks.lastOperation();
    if (!op) {
      element("compare-status").textContent =
        "Primero calcula algo en la pestaña «Calculadora».";
      return;
    }
    element("compare-status").textContent = "Ejecutando los tres backends…";
    try {
      renderComparison(await compareBackends(op.face, op.op));
    } catch (error) {
      element("compare-status").textContent = String(error);
    }
  });

  element("compare-export").addEventListener("click", () => {
    void exportWith("gates", "backends.csv", hooks.onError);
  });

  element("trace-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    const kind = element<HTMLSelectElement>("trace-kind").value as TraceKind;
    const a = element<HTMLInputElement>("trace-a").value.trim();
    const b = element<HTMLInputElement>("trace-b").value.trim();
    try {
      const trace = await traceOperation(kind, a, b);
      currentTrace = trace;
      currentSteps = flatten(trace);
      cursor = -1;
      setPlaybackEnabled(currentSteps.length > 0);
      redraw();
    } catch (error) {
      currentTrace = null;
      currentSteps = [];
      cursor = -1;
      setPlaybackEnabled(false);
      element("trace-output").replaceChildren(text("p", "error", String(error)));
      element("trace-position").textContent = "";
    }
  });

  element("trace-prev").addEventListener("click", () => {
    if (currentSteps.length === 0) {
      return;
    }
    cursor = cursor <= 0 ? 0 : cursor - 1;
    redraw();
  });

  element("trace-next").addEventListener("click", () => {
    if (currentSteps.length === 0) {
      return;
    }
    const last = currentSteps.length - 1;
    cursor = cursor >= last ? last : cursor + 1;
    redraw();
  });

  element("trace-all").addEventListener("click", () => {
    cursor = -1;
    redraw();
  });

  element("trace-export").addEventListener("click", () => {
    void exportWith("trace", "traza-pasos.csv", hooks.onError);
  });

  // The second of the two trace documents `dz-api` builds: the context a
  // step table cannot carry (operands, complements, partial rows, result).
  // Two buttons rather than one that writes two files: the user picks each
  // path in the same native dialog, and neither document is written behind
  // their back.
  element("trace-summary-export").addEventListener("click", () => {
    void exportWith("trace-summary", "traza-resumen.csv", hooks.onError);
  });
}
