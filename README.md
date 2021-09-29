# Bevy Game of life

[![workflow](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_life/actions/workflows/rust.yml)

`bevy_life` is a generic plugin for [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton).
From the classic 2D [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to [WireWorld](https://en.wikipedia.org/wiki/Wireworld) and 3D rules, the plugin is completely generic and dynamic.

See:
 - [Game of life variations](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/gameOfLife2.html)
 - [Wireworld implementation](https://www.quinapalus.com/wi-index.html)


## Examples

### Classic 2D

Run `cargo run --example 2d_classic --features auto-coloring --release` (the release feature flag enables optimizations)

Press space to reload the board

### Cyclic 2D

Run `cargo run --example 2d_cyclic --features auto-coloring --release` (the release feature flag enables optimizations)

Press space to reload the board