import { getLocalStorageOrDefault, setLocalStorage } from '$lib/local-storage';
import default_theme from '$static/default_theme.json';
import { invoke } from '@tauri-apps/api/core';

export interface ThemeFeature {
  name: string;

  [key: string]: string;
}

let themes: ThemeFeature[] | undefined = $state();

let current_theme: ThemeFeature = $state(JSON.parse(getLocalStorageOrDefault('current_theme', JSON.stringify(default_theme))));

export function set_current_theme(theme: ThemeFeature): void {
  current_theme = theme;
}

export async function get_themes(): Promise<ThemeFeature[]> {
  $effect(() => {
    document.documentElement.style.cssText = theme_to_style(current_theme);
    setLocalStorage('current_theme', JSON.stringify(current_theme));
  });

  if (themes) {
    return themes;
  }
  themes = await invoke<ThemeFeature[]>('get_themes');
  const new_current_theme = themes.find(theme => theme.name === current_theme.name);
  if (new_current_theme) {
    current_theme = new_current_theme;
  }
  return themes;
}

export function theme_to_style(theme: ThemeFeature): string {
  return Object.entries(theme)
    .map(([key, value]) => {
      const replaced_key = key.replace(/_/g, '-');
      return `--${replaced_key}: ${value}`;
    })
    .join(';');
}
