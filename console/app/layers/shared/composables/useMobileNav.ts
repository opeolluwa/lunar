export function useMobileNav() {
  const mobileNavOpen = useState<boolean>("mobile-nav-open", () => false);

  function openMobileNav() {
    mobileNavOpen.value = true;
  }

  function closeMobileNav() {
    mobileNavOpen.value = false;
  }

  function toggleMobileNav() {
    mobileNavOpen.value = !mobileNavOpen.value;
  }

  return {
    mobileNavOpen,
    openMobileNav,
    closeMobileNav,
    toggleMobileNav,
  };
}
