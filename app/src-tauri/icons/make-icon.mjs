// Generates `base.png`, the source image `cargo tauri icon` expands into
// every platform size. Written with Node's built-in zlib only -- an icon
// is not worth a dependency, and this way the artwork is reproducible
// from source instead of being a binary nobody can regenerate.
//
// The mark: twelve dots around a ring (the dozen), with the dek/el
// positions filled in the accent colour, on the app's dark panel colour.

import { deflateSync } from "node:zlib";
import { writeFileSync } from "node:fs";
import { Buffer } from "node:buffer";

const SIZE = 1024;
const BACKGROUND = [0x1c, 0x1f, 0x28];
const RING = [0xe8, 0xea, 0xf0];
const ACCENT = [0xf0, 0xa4, 0x4a];

const pixels = Buffer.alloc(SIZE * SIZE * 4);

function set(x, y, [r, g, b], alpha = 255) {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return;
  const at = (y * SIZE + x) * 4;
  pixels[at] = r;
  pixels[at + 1] = g;
  pixels[at + 2] = b;
  pixels[at + 3] = alpha;
}

// Rounded-square background.
const radius = SIZE * 0.22;
for (let y = 0; y < SIZE; y += 1) {
  for (let x = 0; x < SIZE; x += 1) {
    const dx = Math.max(radius - x, 0, x - (SIZE - radius));
    const dy = Math.max(radius - y, 0, y - (SIZE - radius));
    const inside = dx * dx + dy * dy <= radius * radius;
    set(x, y, BACKGROUND, inside ? 255 : 0);
  }
}

function disc(cx, cy, r, colour) {
  for (let y = Math.floor(cy - r); y <= Math.ceil(cy + r); y += 1) {
    for (let x = Math.floor(cx - r); x <= Math.ceil(cx + r); x += 1) {
      const dx = x - cx;
      const dy = y - cy;
      if (dx * dx + dy * dy <= r * r) set(x, y, colour);
    }
  }
}

const centre = SIZE / 2;
const ringRadius = SIZE * 0.3;
for (let i = 0; i < 12; i += 1) {
  const angle = (Math.PI * 2 * i) / 12 - Math.PI / 2;
  const cx = centre + Math.cos(angle) * ringRadius;
  const cy = centre + Math.sin(angle) * ringRadius;
  const isDekOrEl = i === 10 || i === 11;
  disc(cx, cy, SIZE * (isDekOrEl ? 0.055 : 0.042), isDekOrEl ? ACCENT : RING);
}
disc(centre, centre, SIZE * 0.075, ACCENT);

// PNG encoding: filter byte 0 per scanline, then one IDAT.
const raw = Buffer.alloc((SIZE * 4 + 1) * SIZE);
for (let y = 0; y < SIZE; y += 1) {
  raw[y * (SIZE * 4 + 1)] = 0;
  pixels.copy(raw, y * (SIZE * 4 + 1) + 1, y * SIZE * 4, (y + 1) * SIZE * 4);
}

const crcTable = Array.from({ length: 256 }, (_, n) => {
  let c = n;
  for (let k = 0; k < 8; k += 1) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});

function crc32(buffer) {
  let c = 0xffffffff;
  for (const byte of buffer) c = crcTable[(c ^ byte) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(body));
  return Buffer.concat([length, body, crc]);
}

const header = Buffer.alloc(13);
header.writeUInt32BE(SIZE, 0);
header.writeUInt32BE(SIZE, 4);
header[8] = 8; // bit depth
header[9] = 6; // colour type: RGBA
const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk("IHDR", header),
  chunk("IDAT", deflateSync(raw, { level: 9 })),
  chunk("IEND", Buffer.alloc(0)),
]);

const out = new URL("./base.png", import.meta.url);
writeFileSync(out, png);
console.log(`wrote ${out.pathname} (${png.length} bytes)`);
