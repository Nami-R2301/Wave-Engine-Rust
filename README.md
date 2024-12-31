[![Build](https://github.com/Nami-R2301/Wave-Engine-Rust/actions/workflows/build.yml/badge.svg)](https://github.com/Nami-R2301/Wave-Engine-Rust/actions/workflows/build.yml)

# Wave-Engine-Rust
Rust version of Wave-Engine project (C++). 

# Wave Engine

**Taken from https://github.com/Nami-R2301/Wave-Engine/blob/main/README.md**

## What is it ?

- Wave Engine is a cross-platform game engine library targeted at game developers, ideal for creating and editing 2D and 3D games. Currently supporting OpenGL, Vulkan, and DirectX12.

  + **NOTE** : *For experimental features and builds, check out the corresponding branches for the specific API you are looking to work with (main containing the most current stable build).*

## Why use it ?

- Black box approach, allowing game developers to work on **what** to create, rather than **how** to, without ever needing to communicate with internal functions or work with core algorithms.
- Simple and minimalistic design, suitable for small projects that don't necessarily require an overkill API for simple entities and scenes. 
- Lightweight and fast, without requiring high-end hardware to run it, especially when you compare it to other engines like Unreal and Unity
- Great for young aspiring indie companies looking to publish their upcoming game quickly and as effortlessly as possible.
- Awesome educational tool for new and aspiring game developers who are looking for a lightweight engine to show them the ropes on game design.

## How do I use it ?

### Requirements

- Rust version >= 1.5
- Vulkan SDK for Vulkan-dev branch.

### Windows (64 bit)

### Importing the library

Start by importing the repository in your project.

```
cd <Wherever you want to place the library>
git clone --recursive https://github.com/Nami-R2301/Wave-Engine-Rust.git
```

## Build and Run

### Using Cargo

- Open Wave-Engine-Rust root dir with your favorite shell.

- Run the following for Vulkan support:
```
cargo run --features "vulkan"
```

- Run the following for logging and profiling:
```
cargo run --features "debug"
```

- Run the following for everything:
```
cargo run --all-features
```

### Enjoy!

