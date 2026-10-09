# Offline PDF Tools - Design Specification

## 1. Overview
Aplikasi desktop offline untuk manipulasi file PDF. Fokus pada ukuran aplikasi kecil dan efisiensi memori (mirip iLovePDF/Smallpdf versi lokal).

## 2. Architecture
- **Framework:** Tauri v2
- **Frontend:** Svelte (Vite) - dipilih karena tidak ada overhead runtime Virtual DOM, sangat ringan.
- **Backend:** Rust
- **PDF Engine:** `pdfium-render` (binding Rust untuk PDFium). Eksekusi native C++, cepat dan efisien.

## 3. Core Features (MVP)
1. **Merge:** Gabung beberapa PDF menjadi satu.
2. **Split:** Pisah PDF menjadi halaman individu atau rentang halaman (range).
3. **Compress:** Kurangi ukuran file PDF (optimasi gambar internal).
4. **Convert Image:** Ekstrak halaman PDF ke JPG/PNG.

## 4. UI/UX Design
- **Home (Dashboard):** Grid view dengan 4 kartu menu utama untuk tiap fitur (Merge, Split, Compress, Image).
- **Tool View:**
  - Area drag-and-drop besar di tengah layar.
  - Tombol "Select Files" fallback.
  - Opsi konfigurasi spesifik muncul setelah file dipilih (contoh: form rentang halaman untuk Split).
  - Tombol "Proses".
  - Indikator loading/progress bar.

## 5. Data Flow & Memory Management
- **Zero-Copy Frontend:** Frontend tidak pernah memuat isi file PDF ke memory JS.
- **Path Passing:** Frontend hanya mengirim array path absolut file ke backend (Rust).
- **Native Processing:** Rust membaca stream file secara native, memproses via PDFium, dan menulis langsung ke disk (default: folder yang sama dengan file asli, ditambah suffix `-modified`).
- **Event Driven:** Rust mengirim event konfirmasi atau error kembali ke frontend setelah proses I/O selesai.
