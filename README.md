# Vue-convert &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Documentation]][docs.rs] [![vue_convert: rustc 1.40+]][Rust 1.40] [![License]][license] [![maintenance]][free] [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

[Build Status]: https://img.shields.io/github/workflow/status/mocsy/vue_convert/Rust/master
[actions]: https://github.com/mocsy/vue_convert/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/vue_convert.svg
[crates.io]: https://crates.io/crates/vue_convert
[vue_convert: rustc 1.40+]: https://img.shields.io/badge/vue_convert-rustc_1.40+-lightgray.svg
[Rust 1.40]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html
[Documentation]: https://docs.rs/vue_convert/badge.svg
[docs.rs]: https://docs.rs/vue_convert
[License: MIT OR Apache-2.0]: https://img.shields.io/crates/l/clippy.svg
[license]: #license
[maintenance]: https://img.shields.io/badge/maintenance-casual-blue
[free]: http://unhandledexpression.com/general/2018/11/27/foss-is-free-as-in-toilet.html

Designed for typescript only, support for es-whatevs syntax is a non-goal.

# usage
```bash
find ./ -name '*.html' | xargs vue_convert
```

## Roadmap
- [x] Generate a `vue single file component` from a `html` file
- [] Extract relevant `css` rules to `<style>` rather than using links
- [] Generate a `vue router` from a `sitemap.xml`
- [] Link `router` and `sfc`s 
- [] Integrate a web-scraper to turn any webpage into a vue spa auto-magically
- [] Add pwa support to generated vue spa for offline viewing prowess

*Contributors welcome*
