//! The logo of Dr. Mukherjee's Smithery, for its websites. Each constant is
//! one file of this repository, embedded when a site is compiled, so every
//! site shows the current logo without a copy of its own.
//!
//! A site publishes the files it uses and links them by `url_path()`:
//!
//! ```ignore
//! Site::new(…).static_files(&[smithery_logos::MARK, smithery_logos::MARK_DARK])
//! // in the markup:
//! img src=(smithery_logos::MARK.url_path()) alt="";
//! ```
//!
//! `…` is for light backgrounds, `…_DARK` for dark ones. The files are
//! served under `/assets/img/smithery-logos/`, each name carrying a hash of
//! its content (`mark-3f9a1c2b.svg`): browsers keep them for a year, and a
//! new logo gets a new name.

use setzer::StaticFile;

/// The anvil with its sparks, square, transparent.
pub const MARK: StaticFile = setzer::static_file!(
    "light/app_icon_light_transparent.svg" => "assets/img/smithery-logos/mark.svg"
);
pub const MARK_DARK: StaticFile = setzer::static_file!(
    "dark/app_icon_transparent.svg" => "assets/img/smithery-logos/mark-dark.svg"
);

/// Anvil, "Dr. Mukherjee's Smithery" and "EST. MMXXIV · SWITZERLAND" in a row, transparent.
pub const LOGO: StaticFile = setzer::static_file!(
    "light/logo_horizontal_light_transparent.svg" => "assets/img/smithery-logos/logo.svg"
);
pub const LOGO_DARK: StaticFile = setzer::static_file!(
    "dark/logo_horizontal_transparent.svg" => "assets/img/smithery-logos/logo-dark.svg"
);

/// The anvil on a light disc, for browsers in light mode.
pub const FAVICON: StaticFile = setzer::static_file!(
    "light/favicon_light.svg" => "assets/img/smithery-logos/favicon.svg"
);
/// The anvil on a dark disc, for browsers in dark mode (the frame's `favicon_dark`).
pub const FAVICON_DARK: StaticFile = setzer::static_file!(
    "dark/favicon.svg" => "assets/img/smithery-logos/favicon-dark.svg"
);

/// The anvil on a dark square for the iPhone home screen; Setzer draws it as
/// the 180×180 PNG iOS wants. Link it as the frame's `apple_touch_icon`.
pub const APPLE_TOUCH_ICON: StaticFile = setzer::static_file!(
    "dark/apple-touch-icon.svg" => "assets/img/smithery-logos/apple-touch-icon.svg"
);

/// 1200×630, the dark logo, for link previews (`og:image`).
pub const OG_IMAGE: StaticFile =
    setzer::static_file!("dark/og_image.jpg" => "assets/img/smithery-logos/og.jpg");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_file_is_served_once() {
        let all = [
            MARK,
            MARK_DARK,
            LOGO,
            LOGO_DARK,
            FAVICON,
            FAVICON_DARK,
            APPLE_TOUCH_ICON,
            OG_IMAGE,
        ];
        let mut paths: Vec<&str> = all.iter().map(|f| f.path).collect();
        paths.sort();
        paths.dedup();
        assert_eq!(paths.len(), all.len());
        assert!(all.iter().all(|f| !f.bytes.is_empty()));
    }
}
