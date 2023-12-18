<template>
  <RouterView class="overflow-x-hidden" />
</template>

<script setup lang="ts">
const router = useRouter();

onMounted(() => {
  // check if has redirect query
  const windowSearch = new URLSearchParams(window.location.search);
  const redirect = windowSearch.get("redirect");

  console.info(`Redirect query: ${redirect}`);

  if (redirect) {
    console.info(`Redirecting to: ${redirect}`);
    router.push(decodeURIComponent(redirect));
  }
});
</script>
