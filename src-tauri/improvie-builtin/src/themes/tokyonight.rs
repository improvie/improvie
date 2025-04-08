use improvie_plugin::theme::ThemeFeature;

pub fn tokyonight_night_theme() -> ThemeFeature<'static> {
    ThemeFeature {
        name: "tokyonight_night",
        background: "234 21% 9%",              // bg: #1a1b26
        foreground: "228 69% 83%",             // fg: #c0caf5
        muted: "233 25% 8%",                   // bg_dark1: #0c0e14
        muted_foreground: "226 28% 67%",       // fg_dark: #a9b1d6
        popover: "232 20% 8%",                 // bg_dark: #16161e
        popover_foreground: "228 69% 83%",     // fg
        card: "231 19% 10%",                   // bg_highlight: #292e42
        card_primary: "232 23% 12%",           // terminal_black: #414868
        card_foreground: "228 69% 83%",        // fg
        border: "229 20% 37%",                 // fg_gutter: #3b4261
        input: "229 18% 41%",                  // comment: #565f89
        primary: "221 82% 58%",                // blue: #7aa2f7 (黒を加え、落ち着けた青)
        primary_foreground: "234 21% 9%",      // background
        secondary: "267 75% 63%",              // magenta: #bb9af7 (ピンクを少し抑えたトーン)
        secondary_foreground: "234 21% 9%",    // background
        accent: "83 49% 58%",                  // green: #9ece6a (青緑を抑えて落ち着けた緑)
        accent_foreground: "234 21% 9%",       // background
        destructive: "344 85% 59%",            // red: #f7768e (赤を少し暗く)
        destructive_foreground: "228 69% 83%", // fg
        ring: "221 98% 73%",                   // blue5: #89ddff (少し落ち着けた青)
        radius: "0.5rem",
        sidebar_background: "232 20% 8%",         // bg_dark
        sidebar_foreground: "228 69% 83%",        // fg
        sidebar_primary: "221 82% 58%",           // blue
        sidebar_primary_foreground: "234 21% 9%", // background
        sidebar_accent: "174 65% 44%",            // teal: #1abc9c (青緑を少し抑えたトーン)
        sidebar_accent_foreground: "234 21% 9%",  // background
        sidebar_border: "229 20% 37%",            // fg_gutter
        sidebar_ring: "33 100% 55%",              // orange: #ff9e64 (少し抑えたオレンジ)
    }
}
