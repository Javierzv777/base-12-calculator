# GUI Tauri — calculadora dozenal

Interfaz gráfica de la Fase 6 (`docs/04-fase6-interfaces.md`). Tauri 2 con un
frontend vanilla TypeScript + Vite: cuatro componentes visuales (teclado,
display, historial, memoria) no justifican un framework.

## Requisitos

- Node 20+ (probado con v24) y npm.
- Rust y `cargo` (en este equipo, `C:\Users\ADMIN\.cargo\bin`).
- WebView2 (viene con Windows 11).

## Uso

```bash
npm install            # una vez
npm run tauri dev      # ventana con recarga en caliente
npm run tauri build    # instalador; --no-bundle deja solo el .exe
npm run build          # typecheck + bundle del frontend, sin Rust
```

## Cómo está organizada

```
src-tauri/          # Rust: la ventana y la sesión
  src/main.rs         registro de comandos
  src/commands.rs     wrappers finos sobre dz-api, DTOs con serde
  src/state.rs        AppState: backend, modo, escala, historial, memoria
  icons/make-icon.mjs genera base.png (fuente del icono) sin dependencias
src/                # TypeScript: solo presentación
  api.ts              invoke tipado
  keypad.ts           teclado dozenal fijo + pad científico generado del catálogo
  glyphs.ts           detecta soporte de ↊/↋ y cae a X/E si falta
  history.ts, main.ts
```

Dos reglas que conviene no romper:

1. **El frontend no decide semántica dozenal.** Recibe strings ya formateados
   por Rust y los pinta. No hay aritmética sobre valores dozenales en
   JavaScript: sería exactamente lo que el proyecto existe para evitar, pero
   cruzando además una frontera de lenguaje.
2. **El teclado científico se genera del catálogo** que devuelve
   `list_functions`. Registrar una función en `dz-eval` la hace aparecer aquí
   sola, con su aridad y su descripción; no hay una lista de teclas que
   mantener en paralelo.

`src-tauri` es un workspace de Cargo propio, aparte del de la raíz, para que
`cargo test --workspace` del núcleo no arrastre el árbol de dependencias de
Tauri.
