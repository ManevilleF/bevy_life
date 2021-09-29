# Bevy Game of life

[![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)

`bevy_life` is a generic plugin for any Conway's game of life implementation and rules.
From the classic 2D to [WireWorld] and 3D rules, the plugin is completely generic and dynamic.

## Examples

1. Classic 2D

    Run `cargo run --example 2d_classic --features auto-coloring --release` (the release feature flag enables optimizations)

    Press space to reload the board