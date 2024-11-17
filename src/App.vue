<script setup lang="ts">

import SidePanel from './components/SidePanel.vue'
import MainPanel from './components/MainPanel.vue'

import { onMounted } from 'vue'
import { Menu, Submenu, MenuItem } from '@tauri-apps/api/menu'

onMounted(async () => {
  const macOS = navigator.userAgent.includes('Macintosh')
  let optionsAbout = {
    text: "About"
    // action: (id) => dispatch('itemClick', { id, text: t })
  }
  let optionsA = {
    id: "testA",
    text: "testA",
    action: ((id: string) => {id})
  }
  let optionsB = {
    idText: "",
    text: "testB"
    // action: (id) => dispatch('itemClick', { id, text: t })
  }
  let itemAbout = await MenuItem.new(optionsAbout)
  let itemA = await MenuItem.new(optionsA)
  let itemB = await MenuItem.new(optionsB)
  let submenuApp = await Submenu.new({
    text: 'app',
    items: [itemAbout]
  })
  let submenuFile = await Submenu.new({
    text: 'File',
    items: [itemA]
  })
  let submenuEdit = await Submenu.new({ 
    text: 'Edit',
    items: [itemB]
  })
  let menu = await Menu.new({
    items: [submenuApp, submenuFile, submenuEdit]
  })
  await (macOS ? menu.setAsAppMenu() : menu.setAsWindowMenu())
})
 

</script>

<template>
  <div class="flex min-h-screen bg-gray-100 dark:bg-gray-900">
    <SidePanel />
    <MainPanel />
  </div>
</template>

<style>
@import './style.css';
</style>
