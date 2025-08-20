# Installation Guide

## System Requirements

- **Python**: 3.9 or higher
- **Operating System**: Linux, macOS, or Windows
- **Memory**: Minimum 4GB RAM (8GB recommended for large datasets)
- **Dependencies**: NumPy, Pandas, SciPy, scikit-learn

## Installing from PyPI

The simplest installation method for end users:

```bash
pip install velopix
```

> [!NOTE]  
> Due to the limited availabillity of macOS VMs at Github (and the fact that I'm unwilling to pay) macOS builds might not be up-to-date. </br>
> Therefor on macOS I would suggest installing `velopix` from source.

## Installing from Source

For developers or users requiring the latest features:

```bash
git clone https://github.com/SvenHockers/velopix.git
cd velopix
pip install maturin
python -m maturin develop --release
```

### Development Environment Setup

For development work, use a virtual environment to isolate dependencies:

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements-dev.txt
python -m maturin develop --release
```

## Verifying Installation

Test your installation with:

```python
import velopix
from velopix.DetectorEvent import Event, Hit, Track, Module
from velopix.ReconstructionAlgorithms import TrackFollowing
print("Velopix installed successfully!")
```

---

# Developer Guide

## Architecture Overview

Velopix follows a hybrid Rust-Python architecture with four core modules:

### Repository Structure

```
src/
├── algorithms/          # Rust reconstruction algorithms
│   ├── track_following.rs
│   ├── graph_dfs.rs
│   └── search_by_triplet_trie.rs
├── event_model/         # Core data structures
│   ├── event.rs
│   ├── hit.rs
│   ├── module.rs
│   └── track.rs
├── validator/           # Performance validation
│   ├── validator.rs
│   ├── efficiency.rs
│   └── mc_particles.rs
└── velopix/            # Python interface
    └── hyperParameterFramework/  # Optimization suite
        ├── solvers/
        ├── _pipeline.py
        └── _optimizers.py
```