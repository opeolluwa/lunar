export function useMobileSearch() {
  const { searchQuery, clearSearch } = useAppSearch();
  const isActive = useState<boolean>("mobileSearchActive", () => false);

  function openSearch() {
    isActive.value = true;
  }

  function closeSearch() {
    isActive.value = false;
    clearSearch();
  }

  return { isActive, searchQuery, openSearch, closeSearch };
}
