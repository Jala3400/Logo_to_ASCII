# Web Deployment Guide

This guide covers how to build and deploy the Logo to ASCII web application.

## Prerequisites

- [Rust](https://rustup.rs/) (with the `wasm32-unknown-unknown` target)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) ≥ 0.13
- [Node.js](https://nodejs.org/) ≥ 18
- [pnpm](https://pnpm.io/) ≥ 9

### Install the WASM target

```bash
rustup target add wasm32-unknown-unknown
```

## Project Structure

```
web/
├── wasm/          # WASM build output (generated, not committed)
├── build/         # Static site output (generated, not committed)
├── src/
│   ├── styles/    # Global CSS
│   ├── lib/
│   │   ├── wasm.ts        # WASM bindings wrapper
│   │   ├── stores.ts      # Svelte reactive stores
│   │   ├── converter.ts   # Conversion logic
│   │   └── components/
│   │       ├── atoms/      # Button, Slider, Toggle, Select, TextInput, Section
│   │       ├── molecules/  # DropZone, CharacterControls, ImageControls, etc.
│   │       └── organisms/  # Header, Sidebar, PreviewArea
│   └── routes/
│       ├── +layout.svelte
│       ├── +layout.ts
│       └── +page.svelte
├── svelte.config.js
├── vite.config.ts
└── package.json
```

## Local Development

### 1. Build the WASM package

From the repository root:

```bash
wasm-pack build --target web --out-dir web/wasm
```

This compiles the Rust code to WebAssembly and generates the JS/TS bindings in `web/wasm/`.

### 2. Install dependencies

```bash
cd web
pnpm install
```

### 3. Start the dev server

```bash
pnpm run dev
```

The app will be available at `http://localhost:5173`.

### Rebuilding WASM after Rust changes

If you modify any Rust source code (`src/`), re-run `wasm-pack build` from the root before the changes are reflected in the web app:

```bash
wasm-pack build --target web --out-dir web/wasm
```

The Vite dev server will pick up the new WASM automatically.

## Production Build

### Full build (WASM + web)

From the repository root:

```bash
wasm-pack build --target web --out-dir web/wasm
cd web
pnpm run build
```

The static site is written to `web/build/`. You can preview it locally:

```bash
pnpm run preview
```

## Deployment

The web app is a fully static site (no server-side rendering). It can be deployed to any static hosting provider.

### Cloudflare Pages (recommended)

#### Option A: Git integration (automatic deploys)

1. Push the repository to GitHub/GitLab.
2. Go to [Cloudflare Pages](https://dash.cloudflare.com/) → **Workers & Pages** → **Create application** → **Pages** → **Connect to Git**.
3. Select the repository and configure:

    | Setting | Value |
    |---|---|
    | **Framework preset** | None |
    | **Build command** | `wasm-pack build --target web --out-dir web/wasm && cd web && pnpm install && pnpm run build` |
    | **Build output directory** | `web/build` |
    | **Root directory** | `/` |

4. Under **Environment variables**, add:

    | Variable | Value |
    |---|---|
    | `NODE_VERSION` | `22` |

5. Click **Save and Deploy**.

> **Note:** Cloudflare Pages build machines include Rust and `wasm-pack` by default. If the build fails due to a missing WASM target, add `rustup target add wasm32-unknown-unknown &&` at the start of the build command.

#### Option B: Direct upload (manual)

1. Build locally:

    ```bash
    wasm-pack build --target web --out-dir web/wasm
    cd web
    pnpm run build
    ```

1. Install Wrangler:

    ```bash
    pnpm add -g wrangler
    ```

1. Deploy:

    ```bash
    wrangler pages deploy web/build --project-name logo-to-ascii
    ```

### GitHub Pages

1. Build locally (same commands as above).
2. Deploy the `web/build/` directory using [GitHub Actions](https://docs.github.com/en/pages) or manually.

Example workflow (`.github/workflows/deploy.yml`):

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: wasm-pack build --target web --out-dir web/wasm

      - uses: pnpm/action-setup@v4
        with:
          version: 10

      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm
          cache-dependency-path: web/pnpm-lock.yaml

      - name: Install dependencies
        run: cd web && pnpm install

      - name: Build
        run: cd web && pnpm run build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: web/build

      - name: Deploy to GitHub Pages
        uses: actions/deploy-pages@v4
```

### Netlify / Vercel

Both work the same way — set the build command and output directory as described above. The WASM file is served as a static asset with no special configuration needed.

## Troubleshooting

### WASM file not found

Make sure you run `wasm-pack build` before the web build. The `web/wasm/` directory must contain `logo_to_ascii_bg.wasm` and `logo_to_ascii.js`.

### Large WASM binary

The WASM binary is ~4 MB uncompressed (~2 MB gzipped). Cloudflare Pages and most CDNs serve it with compression automatically. If not, enable gzip/brotli in your hosting config.

### CORS errors in development

The Vite dev server handles WASM MIME types correctly. If you serve the `build/` directory with another server, ensure it serves `.wasm` files with `Content-Type: application/wasm`.
