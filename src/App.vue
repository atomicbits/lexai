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
  <div class="flex min-h-screen bg-gray-100 dark:bg-gray-900">
    <!-- Sidebar -->
    <aside class="flex-none w-16 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700">
      <!-- Sidebar Header -->
      <div class="h-16 flex items-center justify-center border-b border-gray-200 dark:border-gray-700">
        <div class="text-blue-600 dark:text-blue-400 font-bold text-2xl">L</div>
      </div>
      
      <!-- Sidebar Actions -->
      <div class="py-4 space-y-4">
        <!-- Home -->
        <div class="flex justify-center">
          <button class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-gray-600 dark:text-gray-300">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
            </svg>
          </button>
        </div>
        
        <!-- Search -->
        <div class="flex justify-center">
          <button class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-gray-600 dark:text-gray-300">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
            </svg>
          </button>
        </div>
        
        <!-- Settings -->
        <div class="flex justify-center">
          <button class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-gray-600 dark:text-gray-300">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 0 1 0 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 0 1 0-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281Z" />
              <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
            </svg>
          </button>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 p-4">
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
            <img src="./assets/vue.svg" class="h-24 p-6 transition-all duration-750 hover:drop-shadow-[0_0_2em_#249b73]" alt="Vue logo" />
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
    </div>
  </div>
</template>

<style>
@import './style.css';

@layer base {
  :root {
    @apply font-['Inter,Avenir,Helvetica,Arial,sans-serif'] text-base leading-6 font-normal;
    @apply text-gray-900 bg-gray-100;
    @apply antialiased;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-text-size-adjust: 100%;
  }

  a {
    @apply font-medium text-blue-600 no-underline hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      @apply text-gray-100 bg-gray-900;
    }
  }
}
</style>