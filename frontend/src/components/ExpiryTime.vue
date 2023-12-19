<template>
  <div class="font-variable text-sm variation-weight-light">
    <time v-if="expiresAt !== undefined">Expires: {{ expireText }}</time>
    <span v-else>Expires: Never</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  expiresAt?: number;
}>();

const expireText = computed(() => {
  if (props.expiresAt === undefined) {
    return "Never";
  }

  const expiresAt = new Date(props.expiresAt * 1000);

  return expiresAt.toLocaleString();
});
</script>
