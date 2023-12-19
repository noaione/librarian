<template>
  <Teleport to="body">
    <div
      :class="`fixed bottom-0 left-1/2 mx-4 my-4 flex -translate-x-1/2 flex-col items-center`"
      data-librarian="toaster"
    >
      <TransitionGroup name="toast">
        <Toast
          v-for="toast in toasts.toasts"
          :id="toast.id"
          :key="toast.id"
          :color="typeToColor(toast.type)"
          :message="toast.message"
          :title="toast.title"
          :duration="toast.duration"
          :persist="toast.persist"
        />
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import useToast from "@/composables/use-toast";

const toasts = useToast();

function typeToColor(type: "success" | "error" | "warning" | "info"): string {
  switch (type) {
    case "success": {
      return "green";
    }
    case "error": {
      return "red";
    }
    case "warning": {
      return "yellow";
    }
    case "info": {
      return "blue";
    }
  }
}
</script>

<style scoped lang="postcss">
.toast-enter-active,
.toast-leave-active {
  transition: all 0.5s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(30px);
}
</style>
