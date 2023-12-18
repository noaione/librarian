<template>
  <login-form v-if="!auth.isLoggedIn" />
  <main v-if="auth.isLoggedIn" class="flex w-full flex-col py-4">
    <h1 class="font-variable variation-weight-bold mx-4 text-2xl">Administration</h1>
    <hr class="mx-4 my-4 border-gray-600 opacity-70 dark:border-gray-400" />
  </main>
</template>

<script setup lang="ts">
import useAuth from "@/composables/use-auth";

const auth = useAuth();
</script>
