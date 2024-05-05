[![Build](https://github.com/Nami-R2301/Wave-Engine-Rust/actions/workflows/build.yml/badge.svg)](https://github.com/Nami-R2301/Wave-Engine-Rust/actions/workflows/build.yml)

# Wave-Engine-Rust
Rust version of Wave-Engine project (C++). 

# Wave Engine

**Taken from https://github.com/Nami-R2301/Wave-Engine/blob/main/README.md**

## What is it ?

- Wave Engine is a cross-platform game engine library targeted at game devleoppers, ideal for creating and editing 2D and 3D games. Currently supporting OpenGL, Vulkan, and DirectX12.

  + **NOTE** : *For experimental features and builds, check out the corresponding branches for the specific API you are looking to work with (main containing the most current stable build).*

## Why use it ?

- Black box approach, allowing game developpers to work on **what** to create, rather than **how** to, without ever needing to communicate with internal functions or work with core algorithms.
- Simple and minimalistic design, suitable for small projects that don't necessarily require an overkill API for simple entities and scenes. 
- Lightweight and fast, without requiring high-end hardware to run it, especially when you compare it to other engines like Unreal and Unity
- Great for young aspiring indie companies looking to publish their upcoming game quickly and as effortlessly as possible.
- Awesome educational tool for new and aspiring game developpers who are looking for a lightweight engine to show them the ropes on game design.

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

### Building the library locally

Build the engine locally only for the current user (C:\Users\<user name>\Appdata\Local) (Requires admin privileges).

- Open Powershell as admin and enter the following

```
<Wave-Engine-Rust root directory>./install_local.bat
```

### Building the library globally

Alternatively, you can install our engine system-wide (C:\Program Files\) (Requires admin privileges).

- Open Powershell as admin and enter the following

```
<Wave-Engine-Rust root directory>./install.bat
```

#### Using Cargo

- Open Wave-Engine-Rust root dir with your favorite shell.

- Run the following:
```
cargo run --package wave_engine
```

### Linux (64 bit)

### Importing the library

Start by importing the repository in your project.

- Open a bash terminal and run the following

```
cd <Wherever you want to place the library>
git clone --recursive https://github.com/Nami-R2301/Wave-Engine-Rust.git
```

### Building the library locally

Build the engine locally only for the current user (/usr/local) (Requires admin privileges).

- Open a bash terminal and run the following

```
sudo chmod +x <Wave-Engine-Rust root directory>./install_local.sh
sudo <Wave-Engine-Rust root directory>./install_local.sh
```


### Building the library globally

Alternatively, you can install our engine system-wide (/opt/) (Requires admin privileges).

- Open a bash terminal and run the following

```
sudo chmod <Wave-Engine-Rust root directory>./install.sh
sudo <Wave-Engine-Rust root directory>./install.sh
```

### MacOS

- We currently do not support MacOS, however if enough requests are made for a MacOS port there will be a MacOS branch for MacOS developpement that will contain its release build and this README will be updated accordingly. Check back periodically to make sure you don't miss any updates on MacOS support if you are one of the requestees.

## How do I remove it ?

### Windows

- Open Powershell as admin and enter the following (May require admin shell if it has been installed globally on the machine)

```
<Wave-Engine-Rust root directory>./uninstall.bat
```

### Linux

- Open a bash terminal and run the following (May require `sudo` if it has been installed globally on the machine)
```
<Wave-Engine-Rust root directory>./uninstall.sh
```
