# smitters - [Smith chart](https://en.wikipedia.org/wiki/Smith_chart) math and drawing

Rust library for [Smith chart](https://en.wikipedia.org/wiki/Smith_chart) (Volpert-Smith chart) math and rendering.

Core math is UI-agnostic. Includes ready-to-use [egui](https://github.com/emilk/egui) integration for interactive charts.

## Web demo

[https://n4n5.dev/smitters/](https://n4n5.dev/smitters/) thanks to [@Its-Just-Nans](https://www.github.com/Its-Just-Nans)

## Examples

```sh
cargo run --example hello
# or
cargo run --example show
```

## Web version

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked

# Dev server with hot reload
trunk serve --example show

# Production build
trunk build --example show --release --public-url ./
```

![Screenshot_2023-09-21_02-59-22](https://github.com/enomado/smitters/assets/707007/4993223b-3f5e-4ec3-9723-84d56a9b5de1)

## Features

- Convert between impedance (R + jX) and Smith chart coordinates
- Constant resistance and constant reactance circle math
- Arc rendering via Bezier curves (using [kurbo](https://github.com/linebender/kurbo))
- Two grid styles: simple and fancy (detailed multi-patch grid)
- Interactive hover: mouse position to impedance readout
- Optional egui integration behind `egui-integration` feature (enabled by default)

## Quick start

```toml
[dependencies]
smitters = "0.2"
```

Run the interactive demo:

```sh
cargo run --example demo
```

The demo lets you:
- Switch between simple and fancy grid styles
- Hover over the chart to see R and X values in real time
- Place an impedance point by entering R and X values

Other examples:

```sh
cargo run --example hello   # basic chart with grid toggle
cargo run --example show    # hover-based impedance display
```

## Usage

### Math only (no GUI dependency)

```rust
use smitters::math::{resistance_to_xy, xy_to_resistance, ResActive, ResReactive};

// Impedance to chart coordinates
let pos = resistance_to_xy(ResActive(1.0), ResReactive(0.5), 100.0);

// Chart coordinates to impedance
let (r, x) = xy_to_resistance(&pos);
```

### With egui

```rust
use smitters::{chart_fancy::draw_smith_fancy, context::Context};

// Inside your egui update:
let ctx = Context::get_context(ui);
draw_smith_fancy(&ctx);
```

## Architecture

| Module | Description |
|--------|-------------|
| `math` | Core types (`ResActive`, `ResReactive`, `Pos2`, `Circle`) and coordinate transforms |
| `trigonometry` | Circle intersection and chord angle calculations |
| `drawing_primitives` | Arc segment computation for constant-R and constant-X lines |
| `egui` | Bezier curve rendering with egui painter |
| `context` | Drawing context that wraps egui painter with Smith chart utilities |
| `chart_simple` | Simple grid renderer |
| `chart_fancy` | Detailed multi-resolution grid renderer |

## License

MIT
