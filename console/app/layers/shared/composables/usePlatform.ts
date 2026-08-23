export function usePlatform() {
  const platformName = useState<string>("platform", () => "web");

  if (import.meta.client && window.__TAURI_INTERNALS__) {
    import("@tauri-apps/plugin-os").then(({ platform }) => {
      platformName.value = platform();
    });
  }

  const isIos = computed(() => platformName.value === "ios");

  const isAndroid = computed(() => platformName.value === "android");

  const isMobile = computed(() => isIos.value || isAndroid.value);

  const isDesktop = computed(() =>
    ["macos", "windows", "linux"].includes(platformName.value),
  );

  const isWeb = computed(() => platformName.value === "web");

  const framework7Theme = computed(() => (isIos.value ? "ios" : "material"));

  return {
    platformName,
    isIos,
    isAndroid,
    isMobile,
    isWeb,
    isDesktop,
    framework7Theme,
  };
}
