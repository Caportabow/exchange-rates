<script setup lang="ts">
const props = defineProps<{
    rate: {
        from: String;
        to: String;
        rate: number;
        mtd: number;
        ytd: number;
    },
}>();

function getArrowStyle(rate: { from: String; to: String; rate: number; mtd: number; ytd: number; }) {
    console.log(rate.rate)
    if (rate.rate == 0) {
        return "fa fa-arrows-h text-gray-500";
    }
    if (rate.rate > rate.mtd) {
        return "fa fa-arrow-up text-green-500";
    } else if (rate.rate < rate.mtd) {
        return "fa fa-arrow-down text-red-500";
    } else {
        return "fa fa-arrows-h text-gray-500";
    }
}

</script>

<template>
    <!-- Header: Exchange Rate -->
    <div class="flex justify-between items-center mb-1">
        <span class="text-lg font-semibold text-white">
            {{ props.rate.from.toUpperCase() }} → {{ props.rate.to.toUpperCase() }}
        </span>
        <span class="text-xl text-white">
            {{ !(props.rate.rate == 0) ? props.rate.rate.toFixed(2) : 'N/A' }}
        </span>
    </div>

    <!-- MTD & YTD Section -->
    <div class="flex justify-between items-center">
        <div class="flex items-center space-x-2">
            <span class="text-xs font-medium text-gray-300 bg-gray-700 px-2 py-1 rounded-lg">
                MTD: {{ props.rate.mtd ? props.rate.mtd.toFixed(5): 'N/A' }}
            </span>
            <span class="text-xs font-medium text-gray-300 bg-gray-700 px-2 py-1 rounded-lg">
                YTD: {{ props.rate.ytd.toFixed(5) }}
            </span>
        </div>
        <i :class="getArrowStyle(props.rate)" class="text-lg"></i>
    </div>
</template>