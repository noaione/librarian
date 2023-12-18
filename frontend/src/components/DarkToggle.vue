<template>
  <button
    :class="`h-8 w-8 cursor-pointer transition hover:opacity-70 dark:hover:opacity-80 ${$props.class ?? ''}`"
    @click="darkMode = !darkMode"
  >
    <i-mdi-lightbulb-outline v-if="darkMode" class="h-full w-full" />
    <i-mdi-lightbulb-on v-else class="h-full w-full" />
  </button>
</template>

<script setup lang="ts">
import useDarkMode from "@/composables/use-dark-mode";

defineProps<{
  class?: string;
}>();

const darkMode = useDarkMode();
</script>
