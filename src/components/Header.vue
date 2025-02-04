<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

const emit = defineEmits();
const isPinned = ref(false);

async function closeApp() {
    await getCurrentWindow().close();
}

async function pinApp() {
    isPinned.value = !isPinned.value;
    await getCurrentWindow().setAlwaysOnTop(isPinned.value);
}
</script>

<template>
  <div class="p-2">
    <div class="flex justify-between items-center py-2 px-4 bg-gray-800 transition-all duration-300 shadow-md hover:shadow-lg rounded-lg select-none">
      <!-- Left Section: App Name + Logo -->
      <div class="drag-area flex items-center space-x-3 data-tauri-drag-region">
        <span class="text-gray-200 font-semibold text-lg">Exchange rates</span>
      </div>

      <!-- Right Section: Clear & Settings -->
      <div class="flex items-center" style="transform-origin: center;">
        <!-- Pin Button - Pulses on hover -->
        <button @click="pinApp" style="rotate: 30deg;" class="transition-all duration-500 rounded-md hover:scale-95 space-x-3" :class="isPinned ? 'pin-button-pressed': 'text-gray-400'">
          <i class="fa fa-thumb-tack text-inherit"></i>
        </button>

        <!-- Close Button - Pulses on hover -->
        <button @click="closeApp" class="transition-all duration-500 rounded-md text-gray-400 hover:scale-95">
          <i class="fa fa-times text-inherit"></i>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bg-gray-800 {
    background-color: #2b3544;
  }
.drag-area {
  -webkit-app-region: drag;
  flex-grow: 1;
}
/* Dark UI Theme */
.text-gray-200 {
  color: #e5e7eb;
}
.text-gray-400 {
  color: #cbd5e0;
}
.pin-button-pressed {
  color: #6b6b6b;
  scale: 0.95;
}
.hover\:text-gray-200:hover {
  color: #e5e7eb;
}
.transition-all {
  transition: all 0.3s ease;
}
.space-x-3 {
  margin-right: 0.75rem;
}
.rounded-md {
  border-radius: 0.375rem;
}
.shadow-md {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
</style>
