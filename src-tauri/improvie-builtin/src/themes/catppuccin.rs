use improvie_plugin::theme::ThemeFeature;

pub fn catppuccin_frappe_theme() -> ThemeFeature<'static> {
    ThemeFeature {
        name: "catppuccin_frappe",
        background: "228 19% 23%",             // base (#303446)
        foreground: "228 69% 87%",             // text (#c6d0f5)
        muted: "227 16% 14%",                  // crust (#232634)
        muted_foreground: "227 28% 69%",       // subtext0 (#a5adce)
        popover: "228 20% 20%",                // mantle (#292c3c)
        popover_foreground: "228 69% 87%",     // text
        card: "228 17% 17%",                   // surface0 (#414559)
        card_primary: "228 19% 20%",           // surface1 (#51576d)
        card_foreground: "228 69% 87%",        // text
        border: "228 11% 58%",                 // overlay0 (#737994)
        input: "228 12% 63%",                  // overlay1 (#838ba7)
        primary: "226 77% 74%",                // blue (#8caaee)
        primary_foreground: "228 17% 17%",     // surface0
        secondary: "239 65% 85%",              // lavender (#babbf1)
        secondary_foreground: "228 17% 17%",   // surface0
        accent: "270 54% 75%",                 // mauve (#ca9ee6)
        accent_foreground: "228 17% 17%",      // surface0
        destructive: "359 70% 69%",            // red (#e78284)
        destructive_foreground: "228 69% 87%", // text
        ring: "270 54% 75%",                   // mauve（リング強調にも合う）
        radius: "0.5rem",
        sidebar_background: "228 20% 20%",         // mantle
        sidebar_foreground: "228 69% 87%",         // text
        sidebar_primary: "226 77% 74%",            // blue
        sidebar_primary_foreground: "228 17% 17%", // surface0
        sidebar_accent: "188 44% 67%",             // teal (#81c8be)
        sidebar_accent_foreground: "228 17% 17%",  // surface0
        sidebar_border: "228 11% 58%",             // overlay0
        sidebar_ring: "16 69% 85%",                // rosewater (#f2d5cf)
    }
}
