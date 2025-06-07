# velopix

**Velopix** is a high-performance track reconstruction framework designed for processing collision data from the LHCb detector at CERN. It reconstructs particle trajectories by analyzing hit patterns recorded across multiple detector modules.

---

## Overview

The package provides a complete reconstruction pipeline along with multiple algorithms and tools for validation and analysis. It is intended for use in high-energy physics data workflows, particularly for experiments that involve tracking particles across layered detector systems.

---

## Features

- Track reconstruction from raw detector hit data
- Multiple algorithmic approaches:
  - Track-Following
  - Graph-based Depth-First Search (DFS)
  - Merged-Triplet Tree Search
- Efficient event parsing and data representation
- Built-in validation metrics:
  - Hit efficiency
  - Track purity
  - Clone and ghost detection
- Modular and extensible design for algorithm development and optimization

---

## Use Cases

Velopix is designed for:

- Physicists analyzing detector data from the LHCb experiment
- Developers comparing or building custom reconstruction algorithms
- Research teams building end-to-end pipelines for particle tracking

---

## Documentation

Full documentation is available online:

- [Installation Guide](https://github.com/SvenHockers/velopix/docs/installation)
- [System Overview](https://github.com/SvenHockers/velopix/docs/system_overview)
- [API Reference](https://github.com/SvenHockers/velopix/docs/api)

---

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/SvenHockers/velopix/blob/main/LICENSE) file for details.
