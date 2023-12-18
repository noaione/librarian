<template>
  <main class="mx-auto my-auto flex h-screen flex-col items-center justify-center">
    <i-mdi-login class="mb-2 h-12 w-12" />
    <div class="font-variable variation-weight-bold text-xl">Login</div>
    <hr class="server-width my-4 border-gray-600 opacity-70 dark:border-gray-400" />
    <div class="server-width mb-2 flex flex-col justify-start">
      <label for="token-form" class="mb-2 text-sm">Token</label>
      <input
        id="token-form"
        ref="inputRef"
        v-model="tokenCode"
        class="form-input w-full transition disabled:cursor-not-allowed disabled:border-opacity-50 disabled:bg-gray-100 dark:bg-gray-800 disabled:dark:bg-gray-900"
        :disabled="submitting"
        @keypress="interceptEnter"
      />
    </div>
    <div ref="errorRef" class="server-width flex flex-col justify-start gap-1">
      <div v-for="(error, idx) in errorMessages" :key="idx" class="text-red-400">{{ error }}</div>
    </div>
    <div class="server-width mt-2 flex flex-col justify-start">
      <button
        class="rounded-md bg-blue-600 py-2 text-white transition hover:bg-blue-500 disabled:cursor-not-allowed disabled:bg-blue-700 disabled:hover:bg-blue-700"
        :disabled="submitting || errorMessages.length > 0"
        :class="{
          'animate-pulse': submitting,
        }"
        @click="performLogin"
      >
        Login
      </button>
    </div>
    <hr class="server-width my-4 border-gray-600 opacity-70 dark:border-gray-400" />
    <div class="mt-2 flex flex-row gap-2">
      <router-link to="/" class="transition hover:opacity-70 dark:hover:opacity-80">
        <i-mdi-home class="h-8 w-8" />
      </router-link>
      <dark-toggle />
    </div>
  </main>
</template>

<script setup lang="ts">
import autoAnimate from "@formkit/auto-animate";
import useAuth from "@/composables/use-auth";

const auth = useAuth();
const inputRef = ref<HTMLInputElement>();
const tokenCode = ref();
const submitting = ref(false);
const errorRef = ref();
const errorMessages = ref(["Token is required."]);

function performLogin() {
  submitting.value = true;
  inputRef.value?.blur();

  auth
    .login(tokenCode.value)
    .then(() => {
      submitting.value = false;
    })
    .catch((error) => {
      submitting.value = false;

      if (error instanceof Error) {
        addError(error.message);
      }
    });
}

function interceptEnter(event: KeyboardEvent) {
  if (event.key === "Enter") {
    event.preventDefault();

    if (tokenCode.value) {
      performLogin();
    }
  }
}

function addError(message: string) {
  // check if error already exists
  if (errorMessages.value.includes(message)) {
    return;
  }

  errorMessages.value.push(message);
}

function removeError(message: string) {
  const idx = errorMessages.value.indexOf(message);

  if (idx === -1) {
    return;
  }

  errorMessages.value.splice(idx, 1);
}

function hasError(message: string) {
  return errorMessages.value.includes(message);
}

onMounted(() => {
  autoAnimate(errorRef.value);
});

watch(
  () => tokenCode.value,
  (newToken) => {
    if (!hasError("Token is required.") && errorMessages.value.length > 0) {
      // empty the error messages
      errorMessages.value = [];
    }

    if (newToken) {
      removeError("Token is required.");
    } else {
      addError("Token is required.");
    }
  }
);
</script>

<style scoped lang="postcss">
.server-width {
  @apply w-[80%] md:w-[60%] lg:w-[40%];
}
</style>
