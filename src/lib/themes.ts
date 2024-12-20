//! ref: https://github.com/skeletonlabs/skeleton/blob/dev/sites/skeleton.dev/src/lib/themes.ts

export interface Theme {
  file: string;
  name: string;
}

export const themes: Theme[] = [
  {
    file: "skeleton",
    name: "💀 Skeleton",
  },
  {
    file: "wintry",
    name: "🌨️ Wintry",
  },
  {
    file: "modern",
    name: "🤖 Modern",
  },
  {
    file: "rocket",
    name: "🚀 Rocket",
  },
  {
    file: "seafoam",
    name: "🧜‍♀️ Seafoam",
  },
  {
    file: "vintage",
    name: "📺 Vintage",
  },
  {
    file: "sahara",
    name: "🏜️ Sahara",
  },
  {
    file: "hamlindigo",
    name: "👔 Hamlindigo",
  },
  {
    file: "gold-nouveau",
    name: "💫 Gold Nouveau",
  },
  {
    file: "crimson",
    name: "⭕ Crimson",
  },
];
