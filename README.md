<div align="center">

# Life Sim: Ecosystem

**English** · [Русский](README.ru.md)

### Grow. Hunt. Adapt the balance.

A small visual ecosystem · herbivores and predators · configurable rules

[![Build](https://github.com/IgorKramar/life_sim_ecosystem/actions/workflows/rust.yml/badge.svg)](https://github.com/IgorKramar/life_sim_ecosystem/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/IgorKramar/life_sim_ecosystem?style=flat-square&color=7aa2f7)](https://github.com/IgorKramar/life_sim_ecosystem/releases)
![Rust](https://img.shields.io/badge/Rust-e0af68?style=flat-square&logo=rust&logoColor=black)
![Macroquad](https://img.shields.io/badge/graphics-Macroquad-bb9af7?style=flat-square)
![TOML](https://img.shields.io/badge/config-TOML-9ece6a?style=flat-square)

[Run](#run) · [Controls](#controls) · [Configuration](#configuration) · [Genealogy](#genealogy-and-limits) · [Development](#development)

</div>

Watch plants grow, herbivores feed and predators hunt on a grid. Agents spend energy, age, reproduce and die; the on-screen counters show population, plants, average energy and generation. Version **0.0.3** is an experimental simulation, not a scientifically validated ecology model.

## Inside the world

| System | Behavior |
|:---|:---|
| Plants | Random growth provides energy for herbivores |
| Herbivores | Feed, find mates and migrate when crowded |
| Predators | Hunt herbivores and gain energy from kills |
| Agents | Track sex, age, energy, parents, generation and family root |
| Observation | Pause, restart, change speed and inspect generation counts |
| Balance | Edit world size, population, food, energy and reproduction in TOML |

## Run

Use a current stable Rust toolchain and a graphical desktop compatible with Macroquad. A minimum supported Rust version is not pinned in the manifest.

```sh
git clone https://github.com/IgorKramar/life_sim_ecosystem.git
cd life_sim_ecosystem
cargo run --release --locked
```

Run from the repository directory to use its [config.toml](config.toml). Without that file in the working directory, the application attempts to create a default one. A read or parse error falls back to defaults and prints a diagnostic.

[Published releases](https://github.com/IgorKramar/life_sim_ecosystem/releases) are available separately; check each release's assets for a suitable build.

## Controls

| Key | Action |
|:---|:---|
| `P` / `Space` | Pause or resume |
| `R` | Reset the world using the already loaded configuration |
| `G` | Show or hide generation counts |
| `=` / `+` | Speed up; default limit is about 100 ticks/second |
| `-` | Slow down; default limit is 2 ticks/second |
| `Q` / `Esc` | Quit |

Actual simulation throughput depends on rendering and hardware. The keyboard handler uses the `Equal` key code for acceleration.

## Configuration

Edit [config.toml](config.toml), then **exit and start the program again**. No recompilation is needed. `R` resets agents but does not reread the file; live configuration reload is not implemented.

| Section | Parameters |
|:---|:---|
| `world` | Grid width and height |
| `population` | Initial count, herbivore ratio and crowding threshold |
| `plants` | Growth attempts, growth amounts and energy cap |
| `herbivore`, `predator` | Initial/birth energy, age, movement and reproduction thresholds; hunting rewards/penalties |
| `common` | Migration, mate-search radius and reproduction costs |
| `genetics` | Relatedness-filter switches and genealogy settings |
| `timing` | Starting tick interval, limits and speed multipliers |
| `render` | Cell size, agent-size limits and plant transparency |

The committed configuration starts with a herbivore ratio of **0.65**. The fallback in `Config::default()` uses **0.95**; removing the file changes the initial balance.

<details>
<summary><strong>Three balance experiments</strong></summary>

Apply these values to the matching sections of the existing file; these fragments are not complete configurations.

**Hardcore — stronger predators**

```toml
[predator]
kill_reward = 200
hunt_fail_penalty = 1

[herbivore]
reproduce_min_energy = 100
```

**Garden — more plants and herbivores**

```toml
[plants]
growth_attempts = 100
max_energy = 200

[population]
herbivore_spawn_ratio = 0.99
```

**Big World — a larger grid**

```toml
[world]
width = 200
height = 150

[population]
init_population = 300
```

</details>

## Visual key

| Entity | Color | Shape |
|:---|:---|:---|
| Plant | Green; opacity reflects energy | Square |
| Male / female herbivore | Blue / light blue | Circle |
| Male / female predator | Red / dark red | Circle |

Agent size reflects energy, within the configured rendering limits.

## Genealogy and limits

Every agent records `parent1_id`, `parent2_id`, `generation` and `family_root`. Initial agents belong to generation zero. Press `G` to display the number of living agents in each generation.

Relatedness filtering is **heuristic**: it first compares family roots, then checks parents and shared parents. The grandparent option blocks agents with the same family root and a generation gap of at least two; it does not traverse a complete ancestry tree. Relationships across different roots can be missed. `max_genealogy_depth` is present in the configuration but is not used by the current check.

There is no save/load, CSV export, inherited mutation system or terrain model. These remain ideas, alongside live preset switching, population graphs, video recording and manual sandbox placement.

## Development

```sh
cargo check --locked
cargo clippy --locked -- -D warnings
cargo test --locked
cargo run                 # debug build
cargo run --release       # optimized build
cargo clean               # remove build artifacts
```

Before proposing changes, run the checks, exercise the release build and update configuration documentation when adding parameters. The existing workflow runs a build and tests.

| File | Responsibility |
|:---|:---|
| [src/main.rs](src/main.rs) | Event loop and controls |
| [src/config.rs](src/config.rs) | Configuration loading and defaults |
| [src/agent.rs](src/agent.rs) | Agents, reproduction and relatedness |
| [src/world.rs](src/world.rs) | World updates and population |
| [src/render.rs](src/render.rs) | Macroquad rendering |

Dependencies in [Cargo.toml](Cargo.toml): Macroquad `0.4.14`, rand `0.10.0`, Serde `1.0`, TOML `0.8`; exact resolved versions are in `Cargo.lock`.

## Version history

| Version | Focus |
|:---|:---|
| 0.0.3 | Genealogy fields, relatedness filtering and generation counters |
| 0.0.2 | TOML configuration, radius-based mate search and modular layout |
| 0.0.1 | Plants, herbivores, predators and time controls |

## License

The previous README declared MIT, but this checkout does not yet include the license text. A license file needs to be supplied before presenting a complete licensing statement.

Built with [Rust](https://www.rust-lang.org/) and [Macroquad](https://macroquad.rs/). Thanks to the Rust community for its help and examples.
