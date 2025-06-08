import { getLocalStorage, setLocalStorage } from '$lib/local-storage';

const black_theme: ThemeFeature = {
  name: 'black',
  background: '222.2 30% 8%',
  foreground: '210 40% 98%',
  muted: '217.2 20% 18%',
  muted_foreground: '215 18% 70%',
  popover: '222.2 30% 8%',
  popover_foreground: '210 40% 98%',
  card: '222.2 30% 9%',
  card_primary: '222.2 40% 12%',
  card_foreground: '210 40% 98%',
  border: '217.2 20% 22%',
  input: '217.2 20% 22%',
  primary: '210 40% 98%',
  primary_foreground: '222.2 30% 15%',
  secondary: '217.2 20% 22%',
  secondary_foreground: '210 40% 98%',
  accent: '217.2 20% 22%',
  accent_foreground: '210 40% 98%',
  destructive: '0 62.8% 35%',
  destructive_foreground: '210 40% 98%',
  ring: '212.7 26.8% 83.9%',
  radius: '0.5rem',
  sidebar_background: '240 5.9% 13%',
  sidebar_foreground: '240 4.8% 95.9%',
  sidebar_primary: '224.3 76.3% 48%',
  sidebar_primary_foreground: '0 0% 100%',
  sidebar_accent: '240 3.7% 18%',
  sidebar_accent_foreground: '240 4.8% 95.9%',
  sidebar_border: '240 3.7% 18%',
  sidebar_ring: '217.2 91.2% 59.8%',
};

export interface ThemeFeature {
  name: string;

  background: string;
  foreground: string;
  muted: string;
  muted_foreground: string;
  popover: string;
  popover_foreground: string;
  card: string;
  card_primary: string;
  card_foreground: string;
  border: string;
  input: string;
  primary: string;
  primary_foreground: string;
  secondary: string;
  secondary_foreground: string;
  accent: string;
  accent_foreground: string;
  destructive: string;
  destructive_foreground: string;
  ring: string;
  radius: string;
  sidebar_background: string;
  sidebar_foreground: string;
  sidebar_primary: string;
  sidebar_primary_foreground: string;
  sidebar_accent: string;
  sidebar_accent_foreground: string;
  sidebar_border: string;
  sidebar_ring: string;
}

let themes: ThemeFeature[] | undefined = $state();

function init() {
  const storedTheme = getLocalStorage('current_theme');
  if (storedTheme) {
    return themes?.find(theme => theme.name === storedTheme) || black_theme;
  }
  return black_theme;
}

let current_theme: ThemeFeature = $state(init());

export function set_current_theme(theme: ThemeFeature): void {
  current_theme = theme;
}

export async function get_themes(): Promise<ThemeFeature[]> {
  $effect(() => {
    document.documentElement.style.cssText = theme_to_style(current_theme);
    setLocalStorage('current_theme', JSON.stringify(current_theme.name));
  });

  if (themes) {
    return themes;
  }
  themes = builtin_get_themes();
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

function builtin_get_themes(): ThemeFeature[] {
  return [
    black_theme,
    {
      name: 'white',
      background: '0 0% 98%',
      foreground: '222.2 47.4% 11.2%',
      muted: '210 20% 94%',
      muted_foreground: '215.4 16.3% 46.9%',
      popover: '0 0% 99%',
      popover_foreground: '222.2 47.4% 11.2%',
      card: '0 0% 97%',
      card_primary: '210 40% 92%',
      card_foreground: '222.2 47.4% 11.2%',
      border: '210 20% 85%',
      input: '210 20% 90%',
      primary: '222.2 47.4% 11.2%',
      primary_foreground: '0 0% 100%',
      secondary: '210 20% 94%',
      secondary_foreground: '222.2 47.4% 11.2%',
      accent: '210 40% 92%',
      accent_foreground: '222.2 47.4% 11.2%',
      destructive: '0 72.2% 50.6%',
      destructive_foreground: '0 0% 100%',
      ring: '222.2 47.4% 60%',
      radius: '0.5rem',
      sidebar_background: '220 20% 96%',
      sidebar_foreground: '222.2 47.4% 11.2%',
      sidebar_primary: '222.2 84% 4.9%',
      sidebar_primary_foreground: '0 0% 100%',
      sidebar_accent: '220 14% 90%',
      sidebar_accent_foreground: '222.2 47.4% 11.2%',
      sidebar_border: '210 16% 85%',
      sidebar_ring: '217.2 91.2% 59.8%',
    },
    {
      name: 'tokyonight_night',
      background: '234 21% 9%',
      foreground: '228 69% 83%',
      muted: '233 25% 8%',
      muted_foreground: '226 28% 67%',
      popover: '232 20% 8%',
      popover_foreground: '228 69% 83%',
      card: '231 19% 10%',
      card_primary: '232 23% 12%',
      card_foreground: '228 69% 83%',
      border: '229 20% 37%',
      input: '229 18% 41%',
      primary: '221 82% 58%',
      primary_foreground: '234 21% 9%',
      secondary: '267 75% 63%',
      secondary_foreground: '234 21% 9%',
      accent: '83 49% 58%',
      accent_foreground: '234 21% 9%',
      destructive: '344 85% 59%',
      destructive_foreground: '228 69% 83%',
      ring: '221 98% 73%',
      radius: '0.5rem',
      sidebar_background: '232 20% 8%',
      sidebar_foreground: '228 69% 83%',
      sidebar_primary: '221 82% 58%',
      sidebar_primary_foreground: '234 21% 9%',
      sidebar_accent: '174 65% 44%',
      sidebar_accent_foreground: '234 21% 9%',
      sidebar_border: '229 20% 37%',
      sidebar_ring: '33 100% 55%',
    },
    {
      name: 'catppuccin_frappe',
      background: '228 19% 23%',
      foreground: '228 69% 87%',
      muted: '227 16% 14%',
      muted_foreground: '227 28% 69%',
      popover: '228 20% 20%',
      popover_foreground: '228 69% 87%',
      card: '228 17% 17%',
      card_primary: '228 19% 20%',
      card_foreground: '228 69% 87%',
      border: '228 11% 58%',
      input: '228 12% 63%',
      primary: '226 77% 74%',
      primary_foreground: '228 17% 17%',
      secondary: '239 65% 85%',
      secondary_foreground: '228 17% 17%',
      accent: '270 54% 75%',
      accent_foreground: '228 17% 17%',
      destructive: '359 70% 69%',
      destructive_foreground: '228 69% 87%',
      ring: '270 54% 75%',
      radius: '0.5rem',
      sidebar_background: '228 20% 20%',
      sidebar_foreground: '228 69% 87%',
      sidebar_primary: '226 77% 74%',
      sidebar_primary_foreground: '228 17% 17%',
      sidebar_accent: '188 44% 67%',
      sidebar_accent_foreground: '228 17% 17%',
      sidebar_border: '228 11% 58%',
      sidebar_ring: '16 69% 85%',
    },
  ];
}
