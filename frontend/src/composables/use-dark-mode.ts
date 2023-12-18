import { useLocalStorage } from "@vueuse/core";

function changeThemeHTML(isDark: boolean) {
  if (isDark) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}

export default function useDarkMode() {
  const darkMode = useLocalStorage("librarian.theme", () => {
    const darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;

    return darkMode ? "dark" : "light";
  });

  const isDarkMode = computed({
    get() {
      return darkMode.value === "dark";
    },
    set(value) {
      darkMode.value = value ? "dark" : "light";
      changeThemeHTML(value);
    },
  });

  return isDarkMode;
}
