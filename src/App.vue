<script setup lang="ts">
import Header from "./components/Header.vue";
import { invoke } from '@tauri-apps/api/core';
import { toast, type ToastOptions } from 'vue3-toastify';
import { ref } from 'vue';

const loaded = ref(false);
const userInput = ref('');
const rates = ref();

async function searchCurrency() {
  const loadingToast = toast("Processing... Please wait", {
    position: toast.POSITION.BOTTOM_RIGHT,
    theme: 'dark',
    type: 'loading',
    autoClose: false, // Don't auto-close until we update it
    closeOnClick: false, // Disable close button until finished
    toastStyle: 'border-radius: 10px; background-color: #1a202c;',
  } as unknown as ToastOptions);;

  try {
    const _rates = await invoke<{ from: String; to: String; rate: number; mtd: number; ytd: number; }[]>('parser', { targetCurrency: userInput.value });
    userInput.value = ""
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
}

function getArrowStyle(rate: { from: String; to: String; rate: number; mtd: number; ytd: number; }) {
  if (rate.rate > rate.mtd) {
    return "fa fa-arrow-up text-green-500";
  } else if (rate.rate < rate.mtd) {
    return "fa fa-arrow-down text-red-500";
  } else {
    return "fa fa-arrows-h text-gray-500";
  }
}
</script>

<template class="bg-gray-900 text-gray-200">
  <div class="flex flex-col h-screen bg-gray-900 text-gray-200" style="border-radius: 15px;">
    <Header />
    <div class="py-2 px-4">
      <section class="mb-4">
        <form @submit.prevent="searchCurrency">
          <input
            v-model="userInput"
            type="text" 
            placeholder="Enter a currency code (e.g., USD, EUR, UAH)" 
            required
            class="w-full p-3 rounded-lg bg-gray-800 text-white placeholder-gray-400 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300 shadow-lg hover:shadow-xl"
          />
        </form>
      </section>
      <section>
          <div v-if="loaded && rates.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">

              <div v-for="rate in rates" class="bg-gray-800 p-4 rounded-lg transition-all duration-500 shadow-xl hover:scale-101">
                <!-- Header: Exchange Rate -->
                <div class="flex justify-between items-center mb-1">
                    <span class="text-lg font-semibold text-white">
                        {{ rate.from.toUpperCase() }} → {{ rate.to.toUpperCase() }}
                    </span>
                    <span class="text-xl text-white">
                        {{ rate.rate.toFixed(2) }}
                    </span>
                </div>

                <!-- MTD & YTD Section -->
                <div class="flex justify-between items-center">
                    <div class="flex items-center space-x-2">
                        <span class="text-xs font-medium text-gray-300 bg-gray-700 px-2 py-1 rounded-lg">
                            MTD: {{ rate.mtd ? rate.mtd.toFixed(5): 'N/A' }}
                        </span>
                        <span class="text-xs font-medium text-gray-300 bg-gray-700 px-2 py-1 rounded-lg">
                            YTD: {{ rate.ytd.toFixed(5) }}
                        </span>
                    </div>
                    <i :class="getArrowStyle(rate)" class="text-lg"></i>
                </div>
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