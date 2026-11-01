# Dr. Mukherjee's Smithery: the logos

An anvil with binary sparks, the wordmark in Space Grotesk, "EST. MMXXIV · SWITZERLAND".

```
light/   for light backgrounds: horizontal and stacked logo, mark, app icon, favicon, splash
dark/    the same for dark backgrounds, plus og_image.jpg (link previews)
round/   seal variants for social media and larger uses, round and square
overview.png
```

Every SVG has a PNG next to it, rendered at 4×. The text is outlines, so no font is needed to
show the logo.

## On the websites

The repository is also a Rust crate. The Smithery's websites (built with
[Setzer](https://github.com/patmuk/setzer)) depend on it and embed the files when they are
compiled, so a change here reaches every site with `cargo update -p smithery-logos`:

```toml
smithery-logos = { git = "https://github.com/dr-mukherjees-smithery/smithery-logos" }
```

```rust
Site::new(…).static_files(&[smithery_logos::MARK, smithery_logos::MARK_DARK])
// in the markup:
img src=(smithery_logos::MARK.url_path()) alt="";
```

`src/lib.rs` lists the files a website can use (mark, logo, favicon, Apple touch icon, link
preview), each served under `/assets/img/smithery-logos/` with a hash of its content in the name,
so browsers can keep it for a year and still see a new logo at once.

## Copyright

© 2024–2026 Dr. Mukherjee's Smithery, Switzerland. All rights reserved. The logo is public so the
websites can build it in; it is not licensed for any other use. Space Grotesk is © The Space
Grotesk Project Authors, under the SIL Open Font License.
