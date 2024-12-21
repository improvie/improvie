//! ref: https://github.com/skeletonlabs/skeleton/blob/dev/sites/skeleton.dev/src/lib/themes.ts

export interface Theme {
  file: string;
  name: string;
  icon: string;
}

export const themes: Theme[] = [
  {
    file: "skeleton",
    name: "Skeleton",
    icon: "💀",
  },
  {
    file: "wintry",
    name: "Wintry",
    icon: "🌨️",
  },
  {
    file: "modern",
    name: "Modern",
    icon: "🤖",
  },
  {
    file: "rocket",
    name: "Rocket",
    icon: "🚀",
  },
  {
    file: "seafoam",
    name: "Seafoam",
    icon: "🧜",
  },
  {
    file: "vintage",
    name: "Vintage",
    icon: "📺",
  },
  {
    file: "sahara",
    name: "Sahara",
    icon: "🏜",
  },
  {
    file: "hamlindigo",
    name: "Hamlindigo",
    icon: "👔",
  },
  {
    file: "gold-nouveau",
    name: "Gold Nouveau",
    icon: "💫",
  },
  {
    file: "crimson",
    name: "Crimson",
    icon: "⭕",
  },
];
