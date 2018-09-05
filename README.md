# Luminite
[![Build Status](https://ci.caelum.ml/buildStatus/icon?job=Luminite)](https://ci.caelum.ml/job/Luminite/)
A 3D maze-based resource struggle game written in Rust.

Currently not even remotely working; this is an experimental project.

Graphics are done using gfx-hal.

The game is described better [here](design/README.md).

## Building
Luminite can be compiled by running
```bash
cargo build --release --features={backend}
```
where `backend` is your graphics backend of choice.

- Vulkan ("vulkan") - Linux/Windows
- DirectX 12 ("dx12") - Windows 10
- OpenGL ("gl") - Any platform
- Metal ("metal") - Apple

You can then run the binary under `target/releases`.

If you don't want to compile Luminite yourself,
precompiled binaries are available
in GitHub Releases. Download the binary for your platform
and run it.

## License
All source code and other assets are licensed under Apache 2.0.
