# engram

A small GPUI-based component library — a Zed-flavored UI toolkit built on `gpui` from the (unreleased) Zed source.

`engram` is a Cargo workspace with three crates that downstream apps consume through the umbrella `engram` crate:

```rust
use engram::prelude::*;
```

## Workspace

| crate | role |
|---|---|
| `engram` | umbrella facade — re-exports `engram_theme` and `engram_ui` |
| `engram-theme` | theme tokens (`Color`, `Spacing`, `Radius`, `TextSize`) + `ActiveTheme` global |
| `engram-ui` | component primitives + shared traits + embedded SVG assets |

## Quick start

```rust
fn main() {
    Application::new()
        .with_assets(engram_ui::Assets)
        .run(|cx| {
            engram_theme::init(cx);
            engram_ui::init(cx);
            // ... open your window
        });
}
```

The two `init` calls are not optional — `engram_theme::init` installs the default dark theme as a GPUI global, and `engram_ui::init` registers the `TextField` keybindings. To use icons, the asset source must be wired through `Application::with_assets`.

The canonical way to eyeball every component is the showcase example:

```bash
cargo run --example showcase -p engram
```

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Third-party content

- **Icons** — `crates/engram-ui/assets/icons/` ships ~140 SVG icons sourced from [Lucide](https://lucide.dev), licensed under [ISC](crates/engram-ui/assets/icons/LICENSES). One derivative (`star_filled.svg`) is Lucide's `star` modified to use `fill="currentColor"`.
- **`crates/engram-ui/src/components/text_field.rs`** — derived from `crates/gpui/examples/input.rs` in [zed-industries/zed](https://github.com/zed-industries/zed) (Apache-2.0). The file header carries the derivation notice required by Apache-2.0 §4(b).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in engram by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
