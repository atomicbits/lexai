<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="flex-1 p-4">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold mb-8">Welcome to Tauri + Vue</h1>

      <div class="flex justify-center mb-8">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#747bff]" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#24c8db]" alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="../assets/vue.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#249b73]" alt="Vue logo" />
        </a>
      </div>
      
      <p class="mb-8">Click on the Tauri, Vite, and Vue logos to learn more.</p>

      <form class="flex justify-center mb-4" @submit.prevent="greet">
        <input 
          id="greet-input" 
          v-model="name" 
          placeholder="Enter a name..." 
          class="mr-2 rounded-lg border border-transparent px-4 py-2 text-base font-medium text-gray-900 bg-white shadow-md transition-colors dark:text-white dark:bg-gray-800"
        />
        <button 
          type="submit"
          class="rounded-lg border border-transparent px-4 py-2 text-base font-medium text-gray-900 bg-white shadow-md transition-colors hover:border-blue-600 active:border-blue-600 active:bg-gray-100 cursor-pointer dark:text-white dark:bg-gray-800 dark:active:bg-gray-700"
        >
          Greet
        </button>
      </form>
      
      <p>{{ greetMsg }}</p>
    </div>
  </main>
</template>