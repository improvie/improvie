import { getLocalStorage, setLocalStorage } from '$lib/local-storage';

let current_theme: string = $state('dark');

export function init_theme() {
  const storedTheme = getLocalStorage('current_theme');
  const newTheme = storedTheme || 'dark';
  document.documentElement.setAttribute('data-theme', newTheme);
  document.documentElement.classList.add(newTheme);
  current_theme = newTheme;
}

export function get_current_theme(): string {
  return current_theme;
}

export function set_current_theme(theme: string): void {
  if (current_theme === theme)
    return;

  document.documentElement.setAttribute('data-theme', theme);
  document.documentElement.classList.remove(current_theme);
  document.documentElement.classList.add(theme);
  current_theme = theme;

  setLocalStorage('current_theme', theme);
}
