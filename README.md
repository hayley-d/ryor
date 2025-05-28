# ryor
Ryor OS is an ambitious from-scratch operating system written entirely in Rust, designed to explore and push the boundaries of low-level systems programming, memory safety, and kernel architecture. The project serves as both a deep technical challenge and an educational platform, providing a robust foundation to understand how modern operating systems work beneath the surface.

Built without relying on the Rust standard library or external operating system components, Ryor implements its own bootloader, custom memory management system (including paging and heap allocation), and low-level interrupt handling. The kernel establishes a clean execution environment on bare metal x86_64 architecture, making it fully bootable on both real hardware and QEMU.

Beyond the basics, Ryor is designed as an evolving playground for experimenting with advanced systems topics, including cooperative multitasking, kernel-space scheduling, file system design, and eventually user-mode process support. Each component is carefully crafted to maximize clarity, performance, and extensibility, making the project a standout example of Rust’s potential in the systems programming domain.

As the project roadmap grows, Ryor aims to become not just a learning tool, but a lightweight, modular kernel framework that can inspire further experimentation in microkernel designs, device driver development, and even OS security research. Every line of code is written with the intent to understand why it works ensuring that this project is as much about intellectual exploration as it is about technical accomplishment.

---

## About

Ryor was born from a desire to deeply understand how operating systems work under the hood. Rather than relying on existing tools or frameworks, this project builds each component piece by piece, focusing on:

- Low-level **bootloading** using the x86_64 architecture.
- Custom **memory management** with page tables and heap allocation.
- Direct **interrupt handling** and programmable interrupt controllers.
- Bare-metal **multitasking** and system call handling.

This is not just an experiment, it’s a fully working OS foundation designed to grow into a testbed for systems experiments, kernel extensions, and educational exploration.

---

## Features

- **Custom Kernel** written entirely in Rust (no `std`, no external OS).
- **Bootable on real hardware or QEMU** via GRUB and a custom bootloader.  
- **Memory Paging & Heap Allocation** with dynamic memory support.  
- **Interrupt Handling** with custom interrupt descriptor tables (IDT).  
- **Basic Multitasking** with context switching and cooperative scheduling.  
- **Custom Shell (Planned)** for interacting with system services.  
- **File System Support (Planned)** with basic read/write operations.  
- **Userland Support (Planned)** to run user-mode applications safely.

The feature roadmap is actively tracked in the [Project Board](https://github.com/hayley-d/ryor/projects).

---

## Getting Started

### Prerequisites

- Rust nightly (`rustup install nightly`)
- `cargo-xbuild` and `bootimage` (`cargo install cargo-xbuild bootimage`)
- QEMU or a physical machine for testing

### How to Compile
```bash
# Linux
cargo +nightly build --target target.json
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

### How to Create Bootable Disk Image
```bash
cargo bootimage
```
