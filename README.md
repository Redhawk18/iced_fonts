# iced_fonts
Include fonts via feature flags in your Iced project!

[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

## Usage

Include `iced_fonts` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
iced_fonts = { version = "0.1.0", features = [...] }
```

## Versioning

| `iced` version | `iced_fonts` version |
| -------------- | ---------------------|
| 0.13.x         | 0.1.0, 0.1.1         |

## Fonts
Fonts can all be enabled with feature flags. All the fonts can be enabled with the `full` feature flag. 

> **_NOTE:_**  By enabling a feature flag you add the full size of the font file(s) to your binary.

The Following are a link to the source followed by their feature flag name.
* [Bootstrap](https://icons.getbootstrap.com) `bootstrap`

* [Nerd Fonts (Symbols Only)](https://www.nerdfonts.com/) `nerd`

# Contributing
If you would like to request a new font, please open an issue with the font's name and where to download the ttf font from. If you would like a faster merge run the [mamba-bronze](https://github.com/Redhawk18/mamba-bronze) script yourself and pr the result.
