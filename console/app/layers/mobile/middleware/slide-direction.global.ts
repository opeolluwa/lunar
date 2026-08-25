let isBack = false;

if (import.meta.client) {
  window.addEventListener("popstate", () => {
    isBack = true;
  });
}

export default defineNuxtRouteMiddleware((to, from) => {
  if (!from || from.matched.length === 0 || from.fullPath === to.fullPath) {
    return;
  }

  const name = isBack ? "slide-right" : "slide-left";
  isBack = false;

  to.meta.pageTransition = { name };
});
