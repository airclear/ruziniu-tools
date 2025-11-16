<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// Batch URL Converter
const urls = ref("");
const concurrency = ref(10);
const progress = ref({ total: 0, converted: 0 });
const results = ref([]);
const isConverting = ref(false);

let unlistenProgress;
let unlistenResult;

onMounted(async () => {
  try {
    unlistenProgress = await listen("conversion-progress", (event) => {
      progress.value = event.payload;
      if (progress.value.converted === progress.value.total) {
        isConverting.value = false;
      }
    });

    unlistenResult = await listen("conversion-result", (event) => {
      results.value.push(event.payload);
    });
  } catch (error) {
    console.log("Event listeners setup:", error);
  }
});

onUnmounted(() => {
  if (unlistenProgress) {
    unlistenProgress();
  }
  if (unlistenResult) {
    unlistenResult();
  }
});

async function startConversion() {
  const urlList = urls.value.split("\n").filter(u => u.trim() !== "");
  if (urlList.length === 0) {
    return;
  }

  isConverting.value = true;
  progress.value = { total: urlList.length, converted: 0 };
  results.value = [];

  try {
    await invoke("batch_short2long_url", {
      urls: urlList,
      concurrency: concurrency.value,
    });
  } catch (error) {
    console.log("Conversion error:", error);
    isConverting.value = false;
    alert("Error: This feature is only available in the Tauri app, not in browser mode.");
  }
}

function downloadCSV() {
  const headers = ["短链", "长链"];
  const csvContent = [
    headers.join(","),
    ...results.value.map(r => `"${r.short_url}","${r.long_url}"`)
  ].join("\n");

  const blob = new Blob([`\uFEFF${csvContent}`], { type: "text/csv;charset=utf-8;" });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = "converted_urls.csv";
  link.click();
  URL.revokeObjectURL(link.href);
}

</script>

<template>
  <main class="min-h-screen bg-gray-50 dark:bg-gray-900 py-8 px-4 sm:px-6 lg:px-8">
    <div class="max-w-4xl mx-auto">
      <!-- URL Converter Section -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 mb-8">
        <h1 class="text-3xl font-bold text-center text-gray-900 dark:text-gray-100 mb-8">
          Batch Short URL to Long URL Converter
        </h1>

        <textarea
          v-model="urls"
          placeholder="Enter one short URL per line"
          rows="10"
          class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg
                 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                 placeholder-gray-500 dark:placeholder-gray-400
                 focus:ring-2 focus:ring-blue-500 focus:border-blue-500
                 outline-none transition-all duration-200 resize-none"
        ></textarea>

        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 mt-6">
          <div class="flex items-center gap-2">
            <label for="concurrency" class="text-gray-700 dark:text-gray-300 font-medium">
              Concurrency:
            </label>
            <input
              id="concurrency"
              type="number"
              v-model.number="concurrency"
              min="1"
              class="w-20 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg
                     bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                     focus:ring-2 focus:ring-blue-500 focus:border-blue-500
                     outline-none transition-all duration-200"
            />
          </div>
          <button
            @click="startConversion"
            :disabled="isConverting"
            class="px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400
                   text-white rounded-lg font-medium transition-all duration-200
                   focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2
                   disabled:cursor-not-allowed disabled:opacity-50"
          >
            {{ isConverting ? 'Converting...' : 'Start' }}
          </button>
        </div>

        <!-- Progress Bar -->
        <div v-if="isConverting" class="mt-6">
          <div class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-6 relative">
            <div
              class="bg-gradient-to-r from-blue-500 to-green-500 h-full rounded-full transition-all duration-300"
              :style="{ width: `${(progress.converted / progress.total) * 100}%` }"
            ></div>
            <span class="absolute inset-0 flex items-center justify-center text-sm font-medium text-gray-700 dark:text-gray-200">
              {{ progress.converted }} / {{ progress.total }}
            </span>
          </div>
        </div>

        <button
          @click="downloadCSV"
          :disabled="isConverting || results.length === 0"
          class="mt-6 px-6 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400
                 text-white rounded-lg font-medium transition-all duration-200
                 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2
                 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Download CSV
        </button>
      </div>
    </div>
  </main>
</template>

<!-- All styles have been migrated to Tailwind CSS utility classes -->
