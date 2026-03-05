# rust_common

A collection of **commonly used algorithms and utilities implemented in Rust**, designed for **high-performance, memory-safe, and reusable applications** in embedded systems, autonomous platforms, and high-reliability software.

This repository serves as a **lightweight algorithm toolbox** for projects involving:

- Embedded systems
- Autonomous vehicles (UAV/USV)
- Robotics and control systems
- Data processing pipelines
- Performance-critical software

The goal is to provide **clean, efficient, and well-tested implementations** of fundamental algorithms frequently used in engineering applications.

---

## Key Features

- Memory-safe implementations using Rust’s ownership model
- Efficient algorithms suitable for real-time and embedded environments
- Minimal dependencies
- Modular and reusable components
- Unit tested implementations

---

## Implemented Algorithms

### Common Types and functions used for Interpolation
- 1D LookUp Table interpolation
- 2D LookUp Table interpolation
-  Possible errors handling for interpolation
-  
#### Interpolation alghorithm used:
- Interpolation method: Linear Point-Slop
- Extrapolation method: Clip
- Index Search method: Binary Search

### Value Algorithms
- Timers (delay)
- Hysterisis 
- On Change 
- check range

### Utility Functions
- Possible Errors handling
- state machine

---

## Project Structure
rust_common
│
├── Cargo.toml
│── build.rs
│── LICENSE
├── README.md
├── src
│   ├── lib.rs
│   │── error.rs
│   │── state.rs
│   ├── algo
│   │   ├── mod.rs
│   │   
│   ├── interp
│   │   ├── mod.rs
│   │   ├── common.rs
│   │   └── lookup_1d.rs
│   │   └── lookup_2d.rs
│   │
│   ├── values
│   │   ├── mod.rs
│   │   ├── delay.rs
│   │   └── hystresis.rs
│   │   └── on_change.rs
│   │   └── range.rs
│   │
│   ├── io
