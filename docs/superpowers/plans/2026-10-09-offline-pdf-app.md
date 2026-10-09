# Offline PDF Tools Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Aplikasi desktop offline untuk manipulasi PDF (Merge, Split, Compress, Convert).

**Architecture:** Frontend Svelte memanggil IPC Tauri. Backend Rust membaca path absolut, mengolah via PDFium, dan menyimpan hasil native.

**Tech Stack:** Tauri v2, Svelte (Vite), Rust, `pdfium-render`.

## Global Constraints
- Aplikasi ukuran kecil (<30MB).
- Frontend hanya memegang path, tidak meload isi file.

---

### Task 1: Scaffold Tauri App & Config

**Files:**
- Create: `package.json`
- Create: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Inisialisasi proyek Tauri**
```bash
npm create tauri-app@latest . -- --manager npm --template svelte-ts -y
npm install
```

- [ ] **Step 2: Tambah dependensi Rust**
```bash
cd src-tauri
cargo add pdfium-render
cargo add tauri -p tauri-build
cd ..
```

- [ ] **Step 3: Commit**
```bash
git add .
git commit -m "chore: scaffold tauri svelte app"
```

### Task 2: Backend PDF Merge (Rust)

**Files:**
- Create: `src-tauri/src/pdf_engine.rs`
- Modify: `src-tauri/src/main.rs`

**Interfaces:**
- Produces: `merge_pdfs(paths: Vec<String>, out_path: String) -> Result<(), String>`

- [ ] **Step 1: Buat engine PDFium minimalis**
```rust
// src-tauri/src/pdf_engine.rs
use pdfium_render::prelude::*;

#[tauri::command]
pub fn merge_pdfs(paths: Vec<String>, out_path: String) -> Result<String, String> {
    let pdfium = Pdfium::default();
    let mut doc = pdfium.create_new_pdf().map_err(|e| e.to_string())?;

    for path in paths {
        let src = pdfium.load_pdf_from_file(&path, None).map_err(|e| e.to_string())?;
        doc.pages().append(&src).map_err(|e| e.to_string())?;
    }

    doc.save_to_file(&out_path).map_err(|e| e.to_string())?;
    Ok(out_path)
}
```

- [ ] **Step 2: Daftarkan command ke Tauri**
```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod pdf_engine;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pdf_engine::merge_pdfs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 3: Test kompilasi Rust**
```bash
cd src-tauri && cargo check && cd ..
```

- [ ] **Step 4: Commit**
```bash
git add src-tauri/
git commit -m "feat(rust): add merge_pdfs command"
```

### Task 3: Frontend Dashboard UI

**Files:**
- Modify: `src/App.svelte`

**Interfaces:**
- Consumes: `merge_pdfs` (IPC command)

- [ ] **Step 1: Buat UI Grid minimalis**
```svelte
<!-- src/App.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let status = "Menunggu file...";

  async function handleMerge() {
    status = "Memproses...";
    try {
      // Mock data untuk MVP I/O test
      const res = await invoke("merge_pdfs", {
        paths: ["/tmp/a.pdf", "/tmp/b.pdf"],
        outPath: "/tmp/merged.pdf"
      });
      status = "Sukses: " + res;
    } catch (e) {
      status = "Error: " + e;
    }
  }
</script>

<main style="padding: 20px; font-family: sans-serif;">
  <h1>PDF Offline Tools</h1>
  <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
    <button on:click={handleMerge} style="padding: 20px;">Merge PDF</button>
    <button disabled style="padding: 20px;">Split PDF</button>
    <button disabled style="padding: 20px;">Compress PDF</button>
    <button disabled style="padding: 20px;">Convert Image</button>
  </div>
  <p>{status}</p>
</main>
```

- [ ] **Step 2: Build frontend**
```bash
npm run build
```

- [ ] **Step 3: Commit**
```bash
git add src/App.svelte
git commit -m "feat(ui): add grid dashboard and merge integration"
```
