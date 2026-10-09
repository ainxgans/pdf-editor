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