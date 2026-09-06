// The two keypads.
//
// The digit/operator pad is fixed -- twelve digits is what base twelve
// means -- but the scientific pad is *generated from the registry
// catalog* that Rust sends over. Registering a function in `dz-eval` is
// therefore enough for it to appear here, with its arity and its
// description, which is the whole point of routing every interface
// through one catalog instead of hand-listing keys three times.

import type { FunctionInfo } from "./api";
import { display } from "./glyphs";

export interface KeySpec {
  /** What the button shows. */
  label: string;
  /** What it inserts into the expression, when it inserts text. */
  insert?: string;
  /** A named action instead of an insertion. */
  action?: "equals" | "clear" | "backspace";
  /** Extra CSS class for layout emphasis. */
  variant?: "digit" | "operator" | "control" | "accent";
  title?: string;
}

const DIGIT_KEYS: KeySpec[] = [
  { label: "7", insert: "7", variant: "digit" },
  { label: "8", insert: "8", variant: "digit" },
  { label: "9", insert: "9", variant: "digit" },
  { label: "↊", insert: "↊", variant: "digit", title: "dek (diez)" },
  { label: "↋", insert: "↋", variant: "digit", title: "el (once)" },
  { label: "÷", insert: "/", variant: "operator" },

  { label: "4", insert: "4", variant: "digit" },
  { label: "5", insert: "5", variant: "digit" },
  { label: "6", insert: "6", variant: "digit" },
  { label: "(", insert: "(", variant: "operator" },
  { label: ")", insert: ")", variant: "operator" },
  { label: "×", insert: "*", variant: "operator" },

  { label: "1", insert: "1", variant: "digit" },
  { label: "2", insert: "2", variant: "digit" },
  { label: "3", insert: "3", variant: "digit" },
  { label: "^", insert: "^", variant: "operator" },
  { label: "%", insert: "%", variant: "operator" },
  { label: "−", insert: "-", variant: "operator" },

  { label: "0", insert: "0", variant: "digit" },
  { label: ";", insert: ";", variant: "digit", title: "punto Humphrey (separador fraccionario)" },
  { label: "C", action: "clear", variant: "control" },
  { label: "⌫", action: "backspace", variant: "control" },
  { label: "=", action: "equals", variant: "accent" },
  { label: "+", insert: "+", variant: "operator" },
];

function button(spec: KeySpec): HTMLButtonElement {
  const element = document.createElement("button");
  element.type = "button";
  element.textContent = display(spec.label);
  element.className = `key ${spec.variant ?? ""}`.trim();
  if (spec.title) {
    element.title = spec.title;
  }
  if (spec.insert !== undefined) {
    element.dataset.insert = spec.insert;
  }
  if (spec.action) {
    element.dataset.action = spec.action;
  }
  return element;
}

export function renderKeypad(container: HTMLElement): void {
  container.replaceChildren(...DIGIT_KEYS.map(button));
}

const CATEGORY_TITLES: Record<string, string> = {
  constant: "Constantes",
  rounding: "Redondeo",
  root: "Raíces",
  exponential: "Exponenciales",
  trigonometry: "Trigonometría",
};

/**
 * A key for each registered function. Zero-argument entries (constants)
 * insert their bare name; everything else opens a call, and the tooltip
 * carries the arity and description the registry declared.
 */
export function renderScientificPad(container: HTMLElement, functions: FunctionInfo[]): void {
  const groups = new Map<string, FunctionInfo[]>();
  for (const info of functions) {
    const bucket = groups.get(info.category) ?? [];
    bucket.push(info);
    groups.set(info.category, bucket);
  }

  const sections: HTMLElement[] = [];
  for (const [category, entries] of groups) {
    const section = document.createElement("section");
    section.className = "science-group";

    const heading = document.createElement("h3");
    heading.textContent = CATEGORY_TITLES[category] ?? category;
    section.append(heading);

    const row = document.createElement("div");
    row.className = "science-keys";
    for (const info of entries) {
      const isConstant = info.max_args === 0;
      const spec: KeySpec = {
        label: info.name,
        insert: isConstant ? info.name : `${info.name}(`,
        variant: "operator",
        title: arityHint(info),
      };
      row.append(button(spec));
    }
    section.append(row);
    sections.push(section);
  }
  container.replaceChildren(...sections);
}

function arityHint(info: FunctionInfo): string {
  const exactness = info.always_exact ? "exacta" : "aproximada";
  if (info.max_args === 0) {
    return `${info.description} (constante, ${exactness})`;
  }
  const args =
    info.min_args === info.max_args
      ? `${info.min_args} argumento(s)`
      : `${info.min_args}-${info.max_args} argumentos`;
  return `${info.description} (${args}, ${exactness})`;
}
