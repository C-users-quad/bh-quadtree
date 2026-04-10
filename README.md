# N-Body Simulation w/ Barnes–Hut

A high-performance N-body simulation exploring how algorithmic and systems-level optimizations enable real-time modeling of large-scale gravitational systems.

## Overview
Simulates **tens of thousands of particles** interacting via gravity in real time.

<p align="center">
  <img src="assets/spiral-galaxy.gif" width="500">
  <br>
  <em>Fig 1: 100k particles orbiting a central mass, forming a spiral structure</em>
</p>

---

## How does it work?
- Partitions space using a **quadtree** to efficiently organize particles
- Uses the **Barnes–Hut approximation** to reduce the cost of force calculations
  - Distant particle clusters are approximated as a single mass
- Uses a **Velocity Verlet**-style integrator to maintain numerical stability over simpler methods such as Euler integration.
- Renders particles and a density heatmap using **glium** (Rust graphics library)

---

## Optimizations & Performance
- Replaced naive **O(N²)** force computation with **O(N log N)** via Barnes–Hut
- Eliminated function call overhead by replacing recursion with an **explicit stack-based traversal**
- Designed **cache-friendly data structures (32-byte structs)** to minimize cache misses
- Improved memory locality by storing quadtree nodes in a **contiguous arena array** to avoid heap fragmentation.
- Parallelized force calculations and rendering prep using **rayon**
  - Used **chunked parallelism** for force calculations to reduce scheduling overhead and improve cache efficiency.
- Reduced GPU overhead with **instanced rendering**, minimizing per-particle data transfer

## What I Learned
- Tradeoffs between simulation accuracy and computational efficiency
- How memory layout and cache behavior impact real-world performance
- Techniques for scaling physics simulations to large particle counts
