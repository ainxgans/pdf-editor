<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { Card, Spinner, Alert, Button } from "flowbite-svelte";
  import { LayersSolid, FilePdfSolid, TrashBinSolid, ArrowsRepeatOutline, ArrowLeftOutline, ArrowRightOutline } from "flowbite-svelte-icons";

  type PageInfo = { id: string, file_path: string, page_index: number, thumbnail: string, rotation: number };
  
  let pages: PageInfo[] = [];
  let status = "";
  let loading = false;
  let error = false;
  let mode = "menu";

  async function handleSelectFiles() {
    status = ""; error = false;
    try {
      const selected = await open({
        multiple: true,
        filters: [{ name: "PDF", extensions: ["pdf"] }]
      });
      
      if (!selected || selected.length === 0) return;
      
      loading = true;
      mode = "editor";
      status = "Memuat halaman dan membuat thumbnail...";
      
      const result: any[] = await invoke("load_pdf_pages", { paths: selected });
      const newPages = result.map(p => ({ ...p, rotation: 0 }));
      pages = [...pages, ...newPages];
      
      status = "";
    } catch (e) {
      error = true;
      status = "Gagal memuat PDF: " + e;
    } finally {
      loading = false;
    }
  }

  function movePage(index: number, dir: number) {
    if (index + dir < 0 || index + dir >= pages.length) return;
    const p = pages[index];
    pages[index] = pages[index + dir];
    pages[index + dir] = p;
    pages = [...pages];
  }

  function removePage(index: number) {
    pages.splice(index, 1);
    pages = [...pages];
  }

  function rotatePage(index: number) {
    pages[index].rotation = (pages[index].rotation + 90) % 360;
    pages = [...pages];
  }

  async function executeMerge() {
    if (pages.length === 0) return;
    
    try {
      const outPath = await save({
        filters: [{ name: "PDF", extensions: ["pdf"] }],
        defaultPath: "hasil_merge_editor.pdf"
      });
      
      if (!outPath) return;

      loading = true;
      status = "Memproses penggabungan PDF...";
      const instructions = pages.map(p => ({
        file_path: p.file_path,
        page_index: p.page_index,
        rotation: p.rotation
      }));

      const res = await invoke("process_pdf", { instructions, outPath });
      
      status = `Sukses! Tersimpan di:\n${res}`;
      error = false;
      mode = "menu";
      pages = [];
    } catch (e) {
      error = true;
      status = "Gagal memproses: " + e;
    } finally {
      loading = false;
    }
  }
</script>

<main class="min-h-screen bg-gray-50 dark:bg-gray-900 p-8 flex flex-col items-center">
  <h1 class="text-3xl font-extrabold mb-8 text-gray-900 dark:text-white">PDF Tools (Offline)</h1>
  
  {#if mode === "menu"}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 w-full max-w-3xl">
      <button on:click={handleSelectFiles} class="text-left focus:outline-none">
        <Card class="hover:shadow-xl transition-all h-full bg-white dark:bg-gray-800 border-none shadow-md">
          <div class="flex flex-col items-center p-6 text-center">
            <LayersSolid class="w-16 h-16 text-blue-600 dark:text-blue-400 mb-4" />
            <h5 class="text-2xl font-bold text-gray-900 dark:text-white">Merge & Edit PDF</h5>
            <p class="text-gray-500 dark:text-gray-400 mt-2">Gabung, hapus, putar, dan urutkan halaman dari beberapa file PDF.</p>
          </div>
        </Card>
      </button>
      
      <button class="text-left focus:outline-none opacity-50 cursor-not-allowed">
        <Card class="h-full bg-white dark:bg-gray-800 border-none shadow-md">
          <div class="flex flex-col items-center p-6 text-center">
            <FilePdfSolid class="w-16 h-16 text-gray-400 mb-4" />
            <h5 class="text-2xl font-bold text-gray-400">Fitur Lainnya</h5>
            <p class="text-gray-400 mt-2">Segera hadir.</p>
          </div>
        </Card>
      </button>
    </div>
  {:else}
    <div class="w-full max-w-6xl">
      <div class="flex flex-wrap justify-between items-center mb-6 gap-4">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">Editor Halaman ({pages.length} Halaman)</h2>
        <div class="flex flex-wrap gap-3">
          <Button color="alternative" on:click={() => { mode = 'menu'; pages = []; }}>Batal</Button>
          <Button color="light" on:click={handleSelectFiles}>+ Tambah PDF</Button>
          <Button color="blue" on:click={executeMerge} disabled={pages.length === 0}>Eksekusi & Simpan</Button>
        </div>
      </div>
      
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        {#each pages as page, i (page.id)}
          <Card padding="none" class="overflow-hidden bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 flex flex-col shadow-sm hover:shadow-md transition-shadow">
            <div class="p-2 bg-gray-100 dark:bg-gray-700 flex justify-between items-center">
              <span class="text-xs font-semibold text-gray-600 dark:text-gray-300 truncate mr-2" title={page.file_path.split('/').pop()}>
                Hal {page.page_index + 1}
              </span>
              <button on:click={() => removePage(i)} class="text-red-500 hover:text-red-400 focus:outline-none">
                <TrashBinSolid class="w-4 h-4" />
              </button>
            </div>
            
            <div class="flex-grow flex items-center justify-center p-4 bg-gray-50 dark:bg-gray-900 overflow-hidden h-48">
              <img src={page.thumbnail} alt="Thumbnail" class="max-h-full max-w-full shadow-sm border border-gray-200 dark:border-gray-700" style={`transform: rotate(${page.rotation}deg); transition: transform 0.2s;`} />
            </div>
            
            <div class="p-2 border-t border-gray-200 dark:border-gray-600 flex justify-center space-x-4 bg-white dark:bg-gray-800">
              <button on:click={() => movePage(i, -1)} disabled={i === 0} class="p-1 text-gray-500 hover:text-blue-600 dark:hover:text-blue-400 disabled:opacity-30 focus:outline-none">
                <ArrowLeftOutline class="w-5 h-5" />
              </button>
              <button on:click={() => rotatePage(i)} class="p-1 text-gray-500 hover:text-green-600 dark:hover:text-green-400 focus:outline-none">
                <ArrowsRepeatOutline class="w-5 h-5" />
              </button>
              <button on:click={() => movePage(i, 1)} disabled={i === pages.length - 1} class="p-1 text-gray-500 hover:text-blue-600 dark:hover:text-blue-400 disabled:opacity-30 focus:outline-none">
                <ArrowRightOutline class="w-5 h-5" />
              </button>
            </div>
          </Card>
        {/each}
      </div>
    </div>
  {/if}

  <div class="mt-8 w-full max-w-3xl">
    {#if loading}
      <div class="flex justify-center items-center space-x-3 text-blue-600 bg-blue-50 dark:bg-gray-800 dark:text-blue-400 p-4 rounded-lg shadow-sm border border-blue-100 dark:border-gray-700">
        <Spinner /> <span class="font-medium">{status || 'Memproses...'}</span>
      </div>
    {:else if status}
      <Alert color={error ? 'red' : 'green'} class="whitespace-pre-wrap text-left shadow-lg dark:bg-gray-800 border dark:border-gray-700">
        <span class="font-bold">{error ? 'Gagal!' : 'Berhasil!'}</span> {status}
      </Alert>
    {/if}
  </div>
</main>
