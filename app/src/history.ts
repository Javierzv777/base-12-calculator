// History list rendering. Entries come from the Rust session, already
// formatted; clicking one puts its input back in the expression box,
// which is the one interaction a calculator history really needs.

import type { HistoryEntry } from "./api";
import { display } from "./glyphs";

export function renderHistory(
  list: HTMLElement,
  entries: HistoryEntry[],
  onPick: (input: string) => void,
): void {
  const items = entries
    .slice()
    .reverse()
    .map((entry) => {
      const item = document.createElement("li");
      item.className = "history-item";

      const input = document.createElement("span");
      input.className = "history-input";
      input.textContent = display(entry.input);

      const result = document.createElement("span");
      result.className = "history-result";
      result.textContent = display(entry.display);
      if (entry.approximate) {
        result.classList.add("approximate");
      }
      if (entry.periodic) {
        result.classList.add("periodic");
        result.title = "periódica en base 12";
      }

      item.append(input, result);
      item.addEventListener("click", () => onPick(entry.input));
      return item;
    });

  if (items.length === 0) {
    const empty = document.createElement("li");
    empty.className = "history-empty";
    empty.textContent = "sin operaciones todavía";
    list.replaceChildren(empty);
    return;
  }
  list.replaceChildren(...items);
}
