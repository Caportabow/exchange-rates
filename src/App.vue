<script setup lang="ts">
import Header from "./components/Header.vue";
import Card from "./components/Card.vue";
import { invoke } from '@tauri-apps/api/core';
import { toast, type ToastOptions } from 'vue3-toastify';
import { ref } from 'vue';

const loaded = ref(false);
const loading = ref(false);
const TextInput = ref('');
const DateInput = ref();
const rates = ref();

async function searchCurrency() {
  if (!TextInput){ return; } // Prevent function from triggering when only date is changed

  loading.value = true;
  const loadingToast = toast("Processing... Please wait", {
    position: toast.POSITION.BOTTOM_RIGHT,
    theme: 'dark',
    type: 'loading',
    autoClose: false, // Don't auto-close until we update it
    closeOnClick: false, // Disable close button until finished
    toastStyle: 'border-radius: 10px; background-color: #1a202c;',
  } as unknown as ToastOptions);;

  try {
    const _rates = await invoke<{ from: String; to: String; rate: number; mtd: number; ytd: number; }[]>('parser', { targetCurrency: TextInput.value.toLowerCase(), date: DateInput.value });
    DateInput.value = ""
    TextInput.value = ""
    if (_rates[0].ytd == 0) {
      toast.update(loadingToast, {
          render: "Invalid currency code",
          type: "error",
          isLoading: false,
          autoClose: 1500,
      });
    } else {
      rates.value = _rates;
      toast.update(loadingToast, {
          render: "Successfully fetched rates from the server!",
          type: "success",
          isLoading: false,
          autoClose: 1500,
      });
      loaded.value = true
    }
  } catch (error) {
    toast.update(loadingToast, {
      render: "Something went wrong. Please try again.",
      type: "error",
      isLoading: false,
      autoClose: 1500,
    });
  }
  loading.value = false;
}
</script>

<template class="bg-gray-900 text-gray-200">
  <div class="flex flex-col h-screen bg-gray-900 text-gray-200" style="border-radius: 15px;">
    <Header />
    <div class="py-2 px-4">
      <section class="mb-4">
        <form class="text-center justify-between" @submit.prevent="searchCurrency">
          <!-- Currency Input -->
          <input
            v-model="TextInput"
            type="text" 
            placeholder="Enter a currency code (e.g., USD, EUR, UAH)" 
            required
            :disabled="loading"
            class="w-170.5 p-3 rounded-lg bg-gray-800 text-white placeholder-gray-400 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300 shadow-lg hover:shadow-xl mr-1"
          />

          <!-- Date Picker (without button) -->
          <input
            v-model="DateInput"
            @keydown.enter="searchCurrency"
            type="date"
            :disabled="loading"
            class="w-28 p-3 rounded-lg bg-gray-800 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300 shadow-lg hover:shadow-xl appearance-none"
          />
        </form>

      </section>
      <section>
          <div v-if="loaded && rates.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <!-- Spawn Cards -->  
              <div v-for="rate in rates" class="bg-gray-800 p-4 rounded-lg transition-all duration-500 shadow-xl hover:scale-101">
                <Card :rate="rate"/>
              </div>
          </div>
          <div v-else class="text-center">
            <p class="text-lg font-bold text-gray-400">Search for currency to see exchanges...</p>
          </div>
      </section>
    </div>
</div>
</template>

<!-- Styles -->
<style scoped>
  input[type="date"]::-webkit-calendar-picker-indicator {
    display: none;
    appearance: none;
  }
  .bg-gray-700 {
    background-color: #2b3544;
  }
  .bg-gray-800 {
    background-color: #1f2631;
  }
  .bg-gray-900 {
    background-color: #1a202c;
  }
  .text-gray-200 {
    color: #e5e7eb;
  }
  body {
    font-family: 'Roboto', sans-serif;
  }
</style>