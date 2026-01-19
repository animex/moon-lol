# moon-lol

## Project Introduction

`moon-lol` is a League of Legends remake project developed using the Bevy engine. Leveraging the high performance of Rust and Bevy's modern ECS architecture, this project implements the parsing and loading of LoL resource files (WAD, PROP, TEX, etc.) and has initially built the combat system, skill system, and UI interface.

![riven](docs/riven.gif)

## Quick Start

Please follow these steps to prepare resources and run the game examples:

### 1. Extract Resource Files

Before running the game, you need to extract resource files from your local League of Legends client.

```bash
cargo run --example extract
```

> **Note**: Before running, ensure that the `root_dir` path in `examples/extract.rs` points to your local League of Legends game directory (e.g., `D:\WeGameApps\League of Legends\Game`). The extracted resources will be stored in the `assets/data` directory. The extracted resource files are quite large (approx. 8GB) and contain about 47,000+ files.

### 2. Start Game

Once the resource extraction is complete, you can run the Riven skill test example to start the game:

```bash
cargo run --example riven --release
```

This example will load Riven's model, animations, and skill logic, allowing you to perform operational tests in the window.

## Acknowledgements and References

This project referenced or used the following open-source projects during development. We thank these community contributors for their work:

- **CDragon**: Provides detailed resource data references.
- **LeagueToolkit**: Resource parsing and processing tools.
- **LoL-NGRID-converter**: Technical implementation related to navigation grid conversion.

## Copyright Notice

**Please be sure to read the following statement:**

1. This project is for technical research and learning purposes only. The repository **does not contain** any native art assets from League of Legends.
2. The copyright for all related art assets and configuration structure code belongs to **Riot Games**.
3. Riot Games reserves the right to request the closure or termination of this project at any time.
