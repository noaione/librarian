<template>
  <login-form v-if="!auth.isLoggedIn" />
  <main v-if="auth.isLoggedIn" class="flex w-full flex-col py-4">
    <div class="mx-4 flex flex-row items-center justify-between">
      <h1 class="font-variable text-2xl variation-weight-bold">Administration</h1>
      <div class="flex flex-row items-center gap-2">
        <router-link to="/">
          <i-mdi-home class="h-8 w-8" />
        </router-link>
        <dark-toggle class="h-8 w-8" />
      </div>
    </div>
    <hr class="mx-4 my-4 border-gray-600 opacity-70 dark:border-gray-400" />
    <div class="mx-4 flex flex-col">
      <div class="mb-2 flex flex-row items-center justify-between">
        <h2 class="font-variable text-xl variation-weight-[550]">
          Invites
          <span v-if="currentInvites !== undefined">[{{ currentInvites.length }}]</span>
        </h2>
        <button
          v-if="!addMode"
          class="font-variable flex flex-row items-center border-2 border-green-500 bg-transparent px-2 py-1 text-sm text-green-500 transition variation-weight-[550] hover:bg-green-600 hover:text-white"
          @click="addMode = true"
        >
          <i-mdi-plus class="mr-1 h-6 w-6" />
          Create
        </button>
      </div>
      <invite-add v-if="addMode && !loading" @add="createInvite" />
      <div v-if="currentInvites && currentInvites.length > 0" class="flex flex-col gap-2">
        <div
          v-for="(invite, idx) in currentInvites"
          :key="invite.token"
          class="flex flex-row items-center justify-between py-2"
        >
          <div class="flex flex-row items-center gap-1">
            <span class="font-variable text-sm variation-weight-black">[{{ idx + 1 }}]</span>
            <span class="font-variable text-sm variation-weight-[550]">{{ invite.token }}</span>
          </div>
          <div class="flex flex-row gap-2">
            <button
              class="font-variable flex flex-row items-center border-2 border-cyan-500 bg-transparent px-2 py-1 text-sm text-cyan-500 transition variation-weight-[550] hover:bg-cyan-600 hover:text-white"
              @click="shareInviteUrl(invite.token)"
            >
              Share
            </button>
            <button
              class="font-variable flex flex-row items-center border-2 border-red-500 bg-transparent px-2 py-1 text-sm text-red-500 transition variation-weight-[550] hover:bg-red-600 hover:text-white"
              @click="deleteInvite(invite.token)"
            >
              Revoke
            </button>
          </div>
        </div>
      </div>
      <div v-else-if="currentInvites && currentInvites.length === 0" class="flex flex-col gap-2">
        <span class="font-variable text-sm variation-weight-[550]">No invites found.</span>
      </div>
      <div v-else-if="loading" class="flex flex-row items-center justify-center gap-2 text-center">
        <i-mdi-loading class="h-8 w-8 animate-spin" />
        <span>Loading...</span>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import useAuth from "@/composables/use-auth";
import useBackend from "@/composables/use-backend";
import useBackendFetch, { makeUrl } from "@/composables/use-backend-fetch";
import useInviteConfig from "@/composables/use-invite-config";
import useToast from "@/composables/use-toast";
import type { Invite } from "@/types/invites";

const auth = useAuth();
const addMode = ref(false);
const configInvite = useInviteConfig();
const toasts = useToast();
const currentInvites = ref<Invite[]>();

const {
  fetch: inviteFetch,
  reload,
  loading,
} = useBackend<Invite[]>(
  "/invite",
  {
    method: "GET",
  },
  {
    immediate: false,
  }
);

async function deleteInvite(token: string) {
  const tokenHeader = new Headers();

  tokenHeader.append("Authorization", `Bearer ${auth.token}`);

  try {
    const results = await fetch(makeUrl(`/invite/${token}`), {
      method: "DELETE",
      headers: tokenHeader,
    });

    const json = await results.json();

    if (json.ok) {
      currentInvites.value = currentInvites.value?.filter((invite) => invite.token !== token);

      toasts.toast({
        title: "Invite revoked",
        message: `The invite has been revoked for: ${token}`,
        type: "success",
      });
    } else {
      toasts.toast({
        title: "Failed to revoke invite",
        message: `Failed to revoke invite for: ${token}`,
        type: "error",
      });
    }
  } catch (error) {
    console.error(error);

    toasts.toast({
      title: "Unknown error",
      message: "An unknown error occurred, please check console.",
      type: "error",
    });
  }
}

async function createInvite(data: { libraries: string[]; labels: string[]; excludeLabels: string[] }) {
  const allLibrary = data.libraries.includes("all") || data.libraries.length === 0;

  addMode.value = false;

  const jsonData = {
    labelsAllow: data.labels,
    labelsExclude: data.excludeLabels,
    sharedLibraries: {
      all: allLibrary,
      libraryIds: allLibrary ? [] : data.libraries,
    },
  };

  const results = await useBackendFetch<Invite>("/invite", {
    method: "POST",
    body: JSON.stringify(jsonData),
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (results) {
    currentInvites.value?.push(results);

    toasts.toast({
      title: "Invite created",
      message: `The invite has been created for: ${results.token}`,
      type: "success",
    });
  }
}

function shareInviteUrl(token: string) {
  const currentHost = window.location.host;

  const targetUrl = `${currentHost}/invite?token=${token}`;

  navigator.clipboard
    .writeText(targetUrl)
    .then(() => {
      toasts.toast({
        message: "Copied to clipboard",
        duration: 1500,
      });
    })
    .catch(() => {
      toasts.toast({
        message: "Failed to copy to clipboard",
        type: "error",
      });
    });
}

async function fetchInviteConfigs() {
  if (!configInvite.inviteConfig?.libraries) {
    await configInvite.fetchInviteConfig();
  }
}

function fetchData() {
  inviteFetch()
    .then((invites) => {
      currentInvites.value = invites;

      fetchInviteConfigs();
    })
    .catch(() => {
      auth.logout();
    });
}

onMounted(() => {
  if (!auth.isLoggedIn) {
    return;
  }

  auth
    .test()
    .then(() => {
      // Do fetch
      nextTick(() => {
        fetchData();
      });
    })
    .catch(() => {
      auth.logout();
    });
});

watch(
  () => auth.isLoggedIn,
  async (logged) => {
    if (logged) {
      await nextTick();

      const [reloadPromise, _] = await Promise.all([reload(), fetchInviteConfigs()]);

      if (reloadPromise) {
        currentInvites.value = reloadPromise;
      }
    }
  }
);
</script>
