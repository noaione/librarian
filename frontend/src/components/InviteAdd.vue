<template>
  <div class="mb-4 flex flex-col gap-4 rounded-md px-2 py-2 dark:bg-gray-800 lg:flex-row">
    <div class="flex flex-col">
      <label class="font-variable text-lg variation-weight-semibold">Libraries</label>
      <div class="flex flex-row items-center">
        <input
          type="checkbox"
          class="form-checkbox mr-2 rounded-md"
          name="library"
          :checked="selectedLibraries.includes('all')"
          @click="selectedLibraries = selectedLibraries.includes('all') ? [] : ['all']"
        />
        <label>All</label>
      </div>
      <div v-for="library in computedLibraries" :key="library.value" class="flex flex-row items-center">
        <input
          type="checkbox"
          class="form-checkbox mr-2 rounded-md disabled:opacity-80"
          name="library"
          :checked="library.checked || selectedLibraries.includes('all')"
          :disabled="selectedLibraries.includes('all')"
          @click="addToLibrary(library.value)"
        />
        <label>{{ library.label }}</label>
      </div>
    </div>

    <div class="flex flex-col">
      <label class="font-variable text-lg variation-weight-semibold">Allowed Labels</label>
    </div>
    <div class="flex flex-col">
      <label class="font-variable text-lg variation-weight-semibold">Excluded Labels</label>
    </div>
  </div>
  <button
    class="font-variable mb-4 flex flex-row items-center justify-center border-2 border-green-500 bg-transparent px-2 py-2 text-sm text-green-500 transition variation-weight-[550] hover:bg-green-600 hover:text-white"
    @click="emit('add', { libraries: selectedLibraries, labels: selectedLabels, excludeLabels: selectedExcludeLabels })"
  >
    <span class="text-center">Add</span>
  </button>
</template>

<script setup lang="ts">
import useInviteConfig from "@/composables/use-invite-config";

interface AddEmit {
  libraries: string[];
  labels: string[];
  excludeLabels: string[];
}

const emit = defineEmits<{
  (e: "add", data: AddEmit): void;
}>();

const inviteConfig = useInviteConfig();
const selectedLibraries = ref<string[]>(["all"]);
const selectedLabels = ref<string[]>([]);
const selectedExcludeLabels = ref<string[]>([]);

const computedLibraries = computed(() => {
  return (
    inviteConfig.inviteConfig?.libraries
      .filter((library) => !library.unavailable)
      .map((library) => ({
        label: library.name,
        value: library.id,
        checked: selectedLibraries.value.includes(library.id),
      })) ?? []
  );
});

function addToLibrary(libraryId: string) {
  if (selectedLibraries.value.includes(libraryId)) {
    selectedLibraries.value = selectedLibraries.value.filter((id) => id !== libraryId);
  } else {
    selectedLibraries.value.push(libraryId);
  }
}
</script>
