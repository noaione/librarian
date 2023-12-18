import { defineStore } from "pinia";
import type { InviteConfig } from "@/types/invites";
import useBackendFetch from "./use-backend-fetch";

const useInviteConfig = defineStore("librarian.invite-config", () => {
  const inviteConfig = ref<InviteConfig>();

  async function fetchInviteConfig() {
    try {
      const resp = await useBackendFetch<InviteConfig>("/invite/config");

      inviteConfig.value = resp;
    } catch (error) {
      console.error(error);

      throw error;
    }
  }

  return {
    inviteConfig,
    fetchInviteConfig,
  };
});

export default useInviteConfig;
