# velopix

**Velopix** is a high-performance track reconstruction framework designed for processing collision data from the LHCb detector at CERN. It reconstructs particle trajectories by analyzing hit patterns recorded across multiple detector modules using a hybrid Rust-Python architecture that combines computational efficiency with algorithmic flexibility.

---

## Architecture Overview

Velopix implements a layered architecture with core computational components written in Rust and exposed through Python bindings via PyO3. The framework consists of four primary modules:

### Core Components

**Event Model (`event_model`)**: Defines the fundamental data structures for detector geometry and particle interactions. The `Event` class represents collision data across 52 detector modules, with `Hit` objects representing spatial coordinates (x, y, z) and optional temporal information. `Module` objects provide geometric boundaries and hit indexing, while `Track` objects represent reconstructed particle trajectories.

**Reconstruction Algorithms (`algorithms`)**: Implements three distinct algorithmic approaches for track reconstruction:

1. **Track Following**: A greedy forward-tracking algorithm that builds tracks by extending hit sequences based on slope compatibility criteria. For hits $h_0$ and $h_1$, compatibility is determined by:

   $$|x_1 - x_0| < \text{max\_slope}_x \times |z_1 - z_0|$$
   $$|y_1 - y_0| < \text{max\_slope}_y \times |z_1 - z_0|$$

2. **Graph DFS**: A graph-based approach that constructs segments between hit pairs and uses depth-first search with iterative weight assignment. Segments are weighted based on their connectivity, and tracks are extracted from high-weight paths through the graph structure.

3. **Search by Triplet Trie**: A pattern-matching algorithm that builds tracks from compatible triplets of hits. Uses a trie data structure to efficiently store and retrieve compatible hit combinations, with scatter validation:
   $$\text{scatter} = \frac{dx^2 + dy^2}{dz_{12}^2} < \text{max\_scatter}$$

**Validation Framework (`validator`)**: Provides comprehensive track quality assessment through Monte Carlo truth matching. Computes efficiency metrics, purity measures, and identifies clone and ghost tracks using configurable matching criteria.

**Hyperparameter Optimization (`hyperParameterFramework`)**: A Python-based optimization suite supporting multiple search strategies including Bayesian optimization, grid search, particle swarm optimization, and polynomial heuristic optimization (PolyHoot). Provides algorithm-specific pipeline implementations for systematic parameter tuning.

### Class Interactions

The framework follows a pipeline architecture where `Event` objects flow through reconstruction algorithms to produce `Track` collections. The `Validator` compares reconstructed tracks against Monte Carlo truth data, while the hyperparameter framework orchestrates algorithm execution and performance optimization.

```mermaid
classDiagram
    %% Core Data Model
    namespace DataModel {
        class Event {
            +Vec~Hit~ hits
            +Vec~Module~ modules
            +u32 event_id
            +get_hits_in_module(module_id) Vec~Hit~
            +get_module(module_id) Module
        }

        class Hit {
            +f64 x, y, z
            +Option~f64~ t
            +u32 module_id, hit_id
            +distance_to(other) f64
        }

        class Module {
            +u32 module_id
            +f64 x_min, x_max, y_min, y_max, z_position
            +contains_point(x, y) bool
        }

        class Track {
            +Vec~Hit~ hits
            +u32 track_id
            +f64 chi_squared
            +add_hit(hit)
            +get_slope_x() f64
            +get_slope_y() f64
        }
    }

    %% Reconstruction Algorithms
    namespace Algorithms {
        class ReconstructionAlgorithm {
            <<interface>>
            +reconstruct(event) Vec~Track~
        }

        class TrackFollowing {
            +f64 max_slope_x, max_slope_y
            +u32 min_hits_per_track
            +reconstruct(event) Vec~Track~
            -is_compatible(hit1, hit2) bool
        }

        class GraphDFS {
            +f64 max_distance
            +u32 max_iterations
            +reconstruct(event) Vec~Track~
            -build_graph(hits) Graph
            -assign_weights(graph)
        }

        class SearchByTripletTrie {
            +f64 max_scatter
            +u32 min_triplet_hits
            +reconstruct(event) Vec~Track~
            -build_trie(hits) Trie
            -validate_scatter(triplet) bool
        }
    }

    %% Validation Framework
    namespace Validation {
        class Validator {
            +f64 matching_threshold
            +validate(tracks, mc_truth) ValidationResult
            +calculate_efficiency(tracks, mc_truth) f64
            +calculate_purity(tracks, mc_truth) f64
        }

        class EfficiencyCalculator {
            +calculate_track_efficiency(tracks, mc_truth) f64
            +calculate_hit_efficiency(tracks, mc_truth) f64
            +identify_clones(tracks) Vec~Track~
            +identify_ghosts(tracks, mc_truth) Vec~Track~
        }

        class MonteCarloTruth {
            +Vec~MCParticle~ particles
            +get_reconstructable_tracks() Vec~MCParticle~
            +match_track(track) Option~MCParticle~
        }
    }

    %% Optimization Framework
    namespace Optimization {
        class PipelineBase {
            <<abstract>>
            +ReconstructionAlgorithm algorithm
            +run_algorithm(params) Vec~Track~
            +evaluate_performance(tracks) f64
        }

        class OptimizerBase {
            <<abstract>>
            +optimize(pipeline) OptimizationResult
        }

        class BayesianOptimizer {
            +u32 n_iterations
            +optimize(pipeline) OptimizationResult
        }

        class GridSearchOptimizer {
            +Vec~ParamGrid~ parameter_grids
            +optimize(pipeline) OptimizationResult
        }

        class ParticleSwarmOptimizer {
            +u32 n_particles, n_iterations
            +optimize(pipeline) OptimizationResult
        }

        class PolyHootOptimizer {
            +u32 polynomial_degree
            +optimize(pipeline) OptimizationResult
        }
    }

    %% Core Relationships
    Event "1" *-- "many" Hit : contains
    Event "1" *-- "many" Module : contains
    Hit "many" --o "1" Module : belongs_to
    Track "1" *-- "many" Hit : composed_of

    %% Algorithm Relationships
    ReconstructionAlgorithm <|.. TrackFollowing : implements
    ReconstructionAlgorithm <|.. GraphDFS : implements
    ReconstructionAlgorithm <|.. SearchByTripletTrie : implements

    TrackFollowing ..> Event : processes
    GraphDFS ..> Event : processes
    SearchByTripletTrie ..> Event : processes

    TrackFollowing ..> Track : produces
    GraphDFS ..> Track : produces
    SearchByTripletTrie ..> Track : produces

    %% Validation Relationships
    Validator ..> Track : validates
    Validator ..> MonteCarloTruth : uses
    EfficiencyCalculator ..> Track : analyzes
    EfficiencyCalculator ..> MonteCarloTruth : compares

    %% Optimization Relationships
    PipelineBase "1" *-- "1" ReconstructionAlgorithm : contains

    OptimizerBase <|-- BayesianOptimizer : inherits
    OptimizerBase <|-- GridSearchOptimizer : inherits
    OptimizerBase <|-- ParticleSwarmOptimizer : inherits
    OptimizerBase <|-- PolyHootOptimizer : inherits

    OptimizerBase ..> PipelineBase : optimizes
    PipelineBase ..> Validator : uses
```

---

## Features

- **High-Performance Core**: Rust implementation with parallel processing capabilities via Rayon
- **Multiple Reconstruction Strategies**: Three complementary algorithms optimized for different detector conditions
- **Comprehensive Validation**: Monte Carlo truth matching with detailed efficiency and purity metrics
- **Hyperparameter Optimization**: Automated parameter tuning with multiple optimization strategies
- **Flexible Data Model**: Support for both spatial (x,y,z) and temporal (t) hit information
- **Modular Architecture**: Extensible design supporting custom algorithm development

---

## Mathematical Foundations

The reconstruction algorithms employ geometric constraints and statistical methods:

- **Slope Compatibility**: Linear trajectory assumption with configurable tolerance bounds
- **Scatter Validation**: Chi-squared-like metric for track quality assessment
- **Weight Assignment**: Iterative graph weighting based on segment connectivity
- **Efficiency Calculation**: Ratio of correctly reconstructed to reconstructable tracks

---

## Use Cases

Velopix is designed for:

- High-energy physics researchers analyzing LHCb detector data
- Algorithm developers comparing reconstruction performance
- Research teams requiring optimized tracking pipelines
- Educational applications in particle physics data analysis

---

## Documentation

Full documentation is available online:

- [Installation Guide](https://github.com/SvenHockers/velopix/blob/main/docs/installation.md)
- [System Overview](https://github.com/SvenHockers/velopix/blob/main/docs/installation.md)
- [API Reference](https://github.com/SvenHockers/velopix/blob/main/docs/api.md)

---

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/SvenHockers/velopix/blob/main/LICENSE) file for details.
