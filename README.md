# Maze Ascension: The Labyrinth of Echoes

A procedurally generated 3D maze game built with Rust and Bevy game engine.
Navigate through hexagonal maze levels that become progressively more
challenging as you ascend.
[Play on itch.io](https://kristoferssolo.itch.io/maze-ascension)

## Features

- Procedurally generated hexagonal mazes
- Multiple floor levels with increasing difficulty
- Smooth floor transitions and animations
- Power-up system (WIP)
- Custom hexagonal grid library implementation

## Installation

1. Clone the repository:

```bash
git clone https://github.com/kristoferssolo/maze-ascension.git
cd maze-ascension
```

2. Build and run:

```bash
just native-release
# or
cargo run --release --no-default-features
```

## License

This project is licensed under the GPLv3 License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- [Bevy Game Engine](https://bevyengine.org/)
- [Red Blob Games' Hexagonal Grids](https://www.redblobgames.com/grids/hexagons/) article for hexagonal grid mathematics
- [hexx](https://github.com/ManevilleF/hexx) for hexagonal grid inspiration
