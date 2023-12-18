import { defineStore } from "pinia";

const useAuth = defineStore(
  "librarian.auth",
  () => {
    const token = ref<string>();

    const isLoggedIn = computed(() => !!token.value);

    async function test() {
      try {
        const resp = await fetch("/api/auth/test", {
          headers: {
            Authorization: `Bearer ${token.value}`,
          },
        });
        const data = await resp.json();

        if (data.ok) {
          token.value = data.token;
        } else {
          throw new Error(data.error);
        }
      } catch (error) {
        console.error(error);

        throw error;
      }
    }

    async function login(loginToken: string) {
      // test with api
      try {
        const resp = await fetch("/api/auth/login", {
          method: "POST",
          body: JSON.stringify({ token: loginToken }),
          headers: {
            "Content-Type": "application/json",
          },
        });
        const data = await resp.json();

        if (data.ok) {
          token.value = loginToken;
        } else {
          throw new Error(data.error);
        }
      } catch (error) {
        console.error(error);

        throw error;
      }
    }

    function logout() {
      token.value = undefined;
    }

    return {
      token,
      isLoggedIn,
      login,
      logout,
      test,
    };
  },
  {
    persist: {
      key: "librarian.auth",
      storage: localStorage,
    },
  }
);

export default useAuth;
