// Rendering the dozenal digits ten and eleven.
//
// PLAN.md Section 12 lists the font risk as the main open issue of this
// phase: U+218A (DEK) and U+218B (EL) are missing from most Windows
// fonts, and a missing glyph renders as a box, which for a calculator is
// worse than a wrong-looking-but-readable letter.
//
// The policy here, per `docs/04-fase6-interfaces.md` section 5: the data
// always carries the canonical glyphs -- Rust formats them, the frontend
// never invents a spelling -- and this module decides only how to *paint*
// them. If the browser has no glyph, every rendered string falls back to
// the ASCII aliases `X`/`E`, which is the same alias table the lexer
// already accepts on input, so what the user sees can be typed back in.
//
// The check is a measurement, not a font list: draw the glyph and a
// private-use codepoint (guaranteed to have no glyph) on a canvas and
// compare widths. A font that does cover U+218A wins and its real glyphs
// are used.

const DEK = "↊";
const EL = "↋";
// A private-use codepoint: no font gives it a glyph, so its width is the
// width of whatever the browser draws for "missing".
const DEFINITELY_MISSING = "\uE000";

function measure(context: CanvasRenderingContext2D, text: string): number {
  return context.measureText(text).width;
}

/** Whether the current font stack can actually draw the dozenal digits. */
export function supportsDozenalGlyphs(): boolean {
  const canvas = document.createElement("canvas");
  const context = canvas.getContext("2d");
  if (!context) {
    return false;
  }
  const style = window.getComputedStyle(document.body);
  context.font = `48px ${style.fontFamily}`;
  const tofu = measure(context, DEFINITELY_MISSING);
  const dek = measure(context, DEK);
  const el = measure(context, EL);
  const different = Math.abs(dek - tofu) > 0.5 && Math.abs(el - tofu) > 0.5;
  return different && dek > 0 && el > 0;
}

/** Presentation-only substitution; never applied to what gets sent to Rust. */
export function toAscii(text: string): string {
  return text.split(DEK).join("X").split(EL).join("E");
}

let glyphsAvailable = true;

export function initGlyphMode(): boolean {
  glyphsAvailable = supportsDozenalGlyphs();
  if (!glyphsAvailable) {
    document.body.classList.add("ascii-glyphs");
  }
  return glyphsAvailable;
}

/** Render a value the way this machine can actually show it. */
export function display(text: string): string {
  return glyphsAvailable ? text : toAscii(text);
}
