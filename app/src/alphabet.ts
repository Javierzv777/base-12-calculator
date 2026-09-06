// The presentation-alphabet dialog (`docs/07-alfabeto-configurable.md`).
//
// This module collects two characters and asks Rust to accept them. It
// deliberately contains **no** rule about what a valid pair is and no
// translation of any kind: deciding that `abs` is a name and `ab` is a
// number is dozenal semantics, and the project's rule is that the
// frontend never decides dozenal semantics. Both live in
// `dz_parse::display_alphabet`, reached through `dz_api::display_alphabet`.
//
// What does live here is the one thing that is genuinely a frontend
// concern: remembering the choice on this machine (`localStorage`) and
// warning when the font cannot draw the chosen symbol.

import { resetAlphabet, setAlphabet, type Session } from "./api";
import { supportsSymbol } from "./glyphs";

const STORAGE_KEY = "dz.alphabet";

const CANONICAL_TEN = "↊";
const CANONICAL_ELEVEN = "↋";

/** The example the preview renders: one of every interesting position. */
const PREVIEW = `1${CANONICAL_TEN};${CANONICAL_ELEVEN}`;

interface Stored {
  ten: string;
  eleven: string;
}

function readStored(): Stored | null {
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return null;
    }
    const parsed = JSON.parse(raw) as Partial<Stored>;
    if (typeof parsed.ten === "string" && typeof parsed.eleven === "string") {
      return { ten: parsed.ten, eleven: parsed.eleven };
    }
  } catch {
    // A corrupt or unavailable store is not an error worth surfacing:
    // the default alphabet is always a correct answer.
  }
  return null;
}

function writeStored(value: Stored | null): void {
  try {
    if (value === null) {
      window.localStorage.removeItem(STORAGE_KEY);
    } else {
      window.localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
    }
  } catch {
    // Persisting is a convenience; failing to persist must not break the
    // running session.
  }
}

function element<T extends HTMLElement>(id: string): T {
  const found = document.getElementById(id);
  if (!found) {
    throw new Error(`missing element #${id}`);
  }
  return found as T;
}

export interface AlphabetHooks {
  /** Repaint everything that shows dozenal text with the new session. */
  onSession: (session: Session) => void;
}

/**
 * Re-apply the preference stored on this machine.
 *
 * Rust validates it again on every boot, because the catalog can grow: a
 * pair that was legal yesterday can swallow a function registered today
 * (`docs/07` section 4). A rejected preference is dropped and the session
 * stays canonical rather than refusing to start.
 */
export async function restoreStoredAlphabet(): Promise<Session | null> {
  const stored = readStored();
  if (!stored) {
    return null;
  }
  try {
    return await setAlphabet(stored.ten, stored.eleven);
  } catch {
    writeStored(null);
    return null;
  }
}

export function initAlphabetModal(hooks: AlphabetHooks): (session: Session) => void {
  const modal = element<HTMLDialogElement>("alphabet-modal");
  const tenInput = element<HTMLInputElement>("alphabet-ten");
  const elevenInput = element<HTMLInputElement>("alphabet-eleven");
  const preview = element<HTMLSpanElement>("alphabet-preview");
  const error = element<HTMLParagraphElement>("alphabet-error");
  const current = element<HTMLSpanElement>("alphabet-current");

  let active: Stored = { ten: CANONICAL_TEN, eleven: CANONICAL_ELEVEN };

  /** The one place the two fields are read, so they cannot drift. */
  function typed(): Stored {
    return {
      ten: [...tenInput.value][0] ?? "",
      eleven: [...elevenInput.value][0] ?? "",
    };
  }

  function setError(message: string, warning = false): void {
    error.textContent = message;
    error.classList.toggle("warning", warning);
  }

  /**
   * The live preview. It is a plain substitution *of the preview string
   * only* -- a fixed literal with no identifiers in it -- and never of
   * anything the user typed. Real text is translated by Rust.
   */
  function paintPreview(): void {
    const { ten, eleven } = typed();
    preview.textContent = PREVIEW.split(CANONICAL_TEN)
      .join(ten || "?")
      .split(CANONICAL_ELEVEN)
      .join(eleven || "?");
    const missing = [ten, eleven].filter((s) => s.length > 0 && !supportsSymbol(s));
    if (missing.length > 0) {
      setError(`la fuente del sistema no dibuja ${missing.join(" ni ")}`, true);
    } else if (error.classList.contains("warning")) {
      setError("");
    }
  }

  /** Paint the dialog and the sidebar button from the session Rust owns. */
  function sync(session: Session): void {
    active = { ten: session.alphabet_ten, eleven: session.alphabet_eleven };
    tenInput.value = active.ten;
    elevenInput.value = active.eleven;
    current.textContent = `${active.ten}${active.eleven}`;
    paintPreview();
  }

  async function apply(next: Stored): Promise<void> {
    if (next.ten.length === 0 || next.eleven.length === 0) {
      setError("hacen falta los dos símbolos");
      return;
    }
    try {
      const session = await setAlphabet(next.ten, next.eleven);
      // Only persist a pair Rust accepted, so a stored preference is
      // always one that at least once was valid.
      writeStored(
        next.ten === CANONICAL_TEN && next.eleven === CANONICAL_ELEVEN ? null : next,
      );
      sync(session);
      hooks.onSession(session);
      modal.close();
    } catch (rejected) {
      // Verbatim: the sentence Rust wrote names the rule that failed and,
      // for the catalog rule, the function that would break.
      setError(String(rejected));
    }
  }

  element("alphabet-open").addEventListener("click", () => {
    setError("");
    tenInput.value = active.ten;
    elevenInput.value = active.eleven;
    paintPreview();
    modal.showModal();
  });

  element("alphabet-cancel").addEventListener("click", () => modal.close());

  element("alphabet-apply").addEventListener("click", () => {
    void apply(typed());
  });

  element("alphabet-reset").addEventListener("click", async () => {
    const session = await resetAlphabet();
    writeStored(null);
    sync(session);
    hooks.onSession(session);
    setError("");
  });

  element("alphabet-presets").addEventListener("click", (event) => {
    const target = (event.target as HTMLElement).closest("button");
    if (!target?.dataset.ten || !target.dataset.eleven) {
      return;
    }
    tenInput.value = target.dataset.ten;
    elevenInput.value = target.dataset.eleven;
    setError("");
    paintPreview();
  });

  for (const input of [tenInput, elevenInput]) {
    input.addEventListener("input", () => {
      // One character per field: keep the last one typed, so replacing a
      // symbol does not need a delete first.
      const characters = [...input.value];
      if (characters.length > 1) {
        input.value = characters[characters.length - 1];
      }
      paintPreview();
    });
  }

  return sync;
}
