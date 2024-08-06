# iced_fonts
Include fonts via feature flags in your Iced project!

[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

## Usage

Include `iced_fonts` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
iced_fonts = { git = "https://github.com/Redhawk18/iced_fonts", features = [...] }
```

## Versioning

| `iced` version | `iced_fonts` version |
| -------------- | ---------------------|
| 0.13.0-dev     | master               |

## Fonts
Fonts can all be enabled with feature flags. All the fonts can be enabled with the `full` feature flag. 

> **_NOTE:_**  By enabling a feature flag you add the full size of the font file(s) to your binary.

The Following are a link to the source followed by their feature flag name.
* [Bootstrap](https://icons.getbootstrap.com) `bootstrap`

* [Nerd Fonts (Symbols Only)](https://www.nerdfonts.com/) `nerd`

