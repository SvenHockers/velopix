# Velopix API Reference

This document provides comprehensive API documentation for the Velopix track reconstruction framework. The API is organized into four main modules that provide data structures, reconstruction algorithms, validation tools, and optimization capabilities.

## Table of Contents

1. [Package Structure](#package-structure)
2. [Core Data Structures](#core-data-structures)
   - [DetectorEvent Module](#detectorevent-module)
3. [Reconstruction Algorithms](#reconstruction-algorithms)
   - [ReconstructionAlgorithms Module](#reconstructionalgorithms-module)
4. [Validation Framework](#validation-framework)
   - [Validator Module](#validator-module)
5. [Optimization Suite](#optimization-suite)
   - [HyperParameter Framework](#hyperparameter-framework)
6. [Usage Examples](#usage-examples)

---

## Package Structure

The Velopix package follows a hierarchical structure with Rust core components exposed through Python bindings:

```text
velopix/
├── DetectorEvent/              # Core data structures (Rust)
│   ├── Event
│   ├── Hit
│   ├── Module
│   └── Track
├── ReconstructionAlgorithms/   # Track reconstruction (Rust)
│   ├── TrackFollowing
│   ├── GraphDFS
│   └── SearchByTripletTrie
├── Validator/                  # Performance validation (Rust)
│   ├── print_validation_summary()
│   ├── compute_tracking_efficiency()
│   ├── export_validation_to_json()
│   └── export_detailed_metrics_json()
└── hyperParameterFramework/    # Optimization suite (Python)
    ├── solvers/
    │   ├── GridSearch
    │   ├── Bayesian
    │   ├── ParticleSwarm
    │   └── PolyHoot
    ├── PipelineBase
    ├── EventMetricsCalculator
    └── BaseOptimizer
```

## Core Data Structures

### DetectorEvent Module

The DetectorEvent module provides the fundamental data structures for representing detector geometry and particle interactions. All classes are implemented in Rust for optimal performance and exposed through Python bindings.

#### Hit Class

Represents a single particle interaction with the detector at a specific spatial and temporal coordinate.

**Attributes:**
- `id: int` — Unique identifier for the hit within the event
- `x: float` — X-coordinate in detector coordinate system (mm)
- `y: float` — Y-coordinate in detector coordinate system (mm)  
- `z: float` — Z-coordinate along beam axis (mm)
- `t: float` — Time of interaction (ns), optional
- `module_number: int` — Index of detector module containing this hit
- `with_t: bool` — Flag indicating whether temporal information is available

**Constructor:**
```python
Hit(
    x: float,
    y: float, 
    z: float,
    hit_id: int,
    module: Optional[int] = None,
    t: Optional[float] = None,
    with_t: Optional[bool] = None
) -> Hit
```

**Methods:**
- `__repr__() -> str` — String representation showing coordinates and module information

#### Track Class

Represents a reconstructed particle trajectory as a sequence of hits across detector modules.

**Attributes:**
- `hits: List[Hit]` — Ordered sequence of hits forming the track
- `missed_last_module: bool` — Flag indicating if track missed the final detector module
- `missed_penultimate_module: bool` — Flag indicating if track missed the second-to-last module
- `missed_modules: int` — Total count of missed modules (internal use)

**Constructor:**
```python
Track(hits: List[Hit]) -> Track
```

**Methods:**
- `add_hit(hit: Hit) -> None` — Append a hit to the track sequence
- `__repr__() -> str` — String representation showing hit count and hit details

#### Module Class

Represents a detector module with geometric boundaries and hit indexing for efficient access.

**Attributes:**
- `module_number: int` — Sequential module identifier (0-51 for LHCb VeloPix)
- `z: float` — Z-position of module along beam axis (mm)
- `hit_start_index: int` — Starting index in global hit array
- `hit_end_index: int` — Ending index in global hit array
- `global_hits: List[Hit]` — Reference to complete event hit collection

**Constructor:**
```python
Module(
    module_number: int,
    z: float,
    hit_start_index: int,
    hit_end_index: int,
    global_hits: List[Hit]
) -> Module
```

**Methods:**
- `hits() -> List[Hit]` — Extract hits belonging to this module
- `__repr__() -> str` — Detailed module information including hit count

#### Event Class

Encapsulates a complete collision event with hits organized across detector modules.

**Attributes:**
- `description: str` — Human-readable event description
- `montecarlo: PyObject` — Monte Carlo truth data for validation
- `module_prefix_sum: List[int]` — Cumulative hit counts per module for indexing
- `number_of_hits: int` — Total hit count across all modules
- `module_zs: List[List[float]]` — Unique Z-positions grouped by module
- `hits: List[Hit]` — Complete collection of event hits
- `modules: List[Module]` — Detector module objects with geometric information

**Constructor:**
```python
Event(json_data: Dict[str, Any]) -> Event
```

The constructor expects a dictionary containing:
- `description: str` — Event description
- `montecarlo: Any` — Monte Carlo truth information
- `module_prefix_sum: List[int]` — Hit count prefixes
- `x: List[float]` — Hit X-coordinates
- `y: List[float]` — Hit Y-coordinates  
- `z: List[float]` — Hit Z-coordinates
- `t: List[float]` — Hit times (optional)

## Reconstruction Algorithms

### ReconstructionAlgorithms Module

The ReconstructionAlgorithms module provides three distinct approaches to particle track reconstruction, each optimized for different detector conditions and track topologies.

#### TrackFollowing Class

A greedy forward-tracking algorithm that builds tracks by extending hit sequences based on geometric compatibility criteria.

**Mathematical Foundation:**
The algorithm uses slope compatibility checks between consecutive hits:
```
|x₁ - x₀| < max_slope_x × |z₁ - z₀|
|y₁ - y₀| < max_slope_y × |z₁ - z₀|
```

**Constructor:**
```python
TrackFollowing(
    max_slopes: Optional[Tuple[float, float]] = (0.7, 0.7),
    max_tolerance: Optional[Tuple[float, float]] = (0.4, 0.4),
    max_scatter: Optional[float] = 0.4,
    min_track_length: Optional[int] = 3,
    min_strong_track_length: Optional[int] = 4
) -> TrackFollowing
```

**Parameters:**
- `max_slopes` — Maximum allowed slope values (dx/dz, dy/dz)
- `max_tolerance` — Tolerance bounds for track extrapolation (mm)
- `max_scatter` — Maximum scattering parameter for three-hit validation
- `min_track_length` — Minimum hits required for track acceptance
- `min_strong_track_length` — Minimum hits for high-quality track classification

**Methods:**
- `are_compatible(hit0: Hit, hit1: Hit) -> bool` — Test slope compatibility between two hits
- `check_tolerance(hit0: Hit, hit1: Hit, hit2: Hit) -> bool` — Validate three-hit alignment within tolerance
- `solve(event: Event) -> List[Track]` — Reconstruct tracks for a single event
- `solve_batch(events: List[Event]) -> List[List[Track]]` — Batch processing for multiple events

#### GraphDFS Class

A graph-based reconstruction algorithm that constructs hit segments and uses depth-first search with iterative weight assignment.

**Algorithm Overview:**
1. Create segments between compatible hit pairs
2. Build connectivity graph between segments
3. Assign weights based on segment connectivity
4. Extract tracks from high-weight paths using DFS

**Constructor:**
```python
GraphDFS(
    max_slopes: Optional[Tuple[float, float]] = (0.7, 0.7),
    max_tolerance: Optional[Tuple[float, float]] = (0.4, 0.4),
    max_scatter: Optional[float] = 0.4,
    minimum_root_weight: Optional[int] = 1,
    weight_assignment_iterations: Optional[int] = 2,
    allowed_skip_modules: Optional[int] = 1,
    allow_cross_track: Optional[bool] = True,
    clone_ghost_killing: Optional[bool] = True
) -> GraphDFS
```

**Parameters:**
- `minimum_root_weight` — Minimum weight threshold for root segment selection
- `weight_assignment_iterations` — Number of iterations for weight propagation
- `allowed_skip_modules` — Maximum modules a track may skip
- `allow_cross_track` — Enable tracks crossing in detector projection
- `clone_ghost_killing` — Enable removal of duplicate and spurious tracks

**Methods:**
- `solve(event: Event) -> List[Track]` — Reconstruct tracks using graph-based approach
- `solve_batch(events: List[Event]) -> List[List[Track]]` — Batch processing capability

#### SearchByTripletTrie Class

A pattern-matching algorithm that builds tracks from compatible triplets of hits using trie data structures for efficient storage and retrieval.

**Mathematical Foundation:**
Uses scatter validation for triplet compatibility:
```
scatter = (dx² + dy²) / dz²₁₂ < max_scatter
```

**Constructor:**
```python
SearchByTripletTrie(
    max_scatter: Optional[float] = 0.1,
    min_strong_track_length: Optional[int] = 4,
    allowed_missed_modules: Optional[int] = 2
) -> SearchByTripletTrie
```

**Parameters:**
- `max_scatter` — Maximum scatter parameter for triplet validation
- `min_strong_track_length` — Minimum hits for strong track classification
- `allowed_missed_modules` — Maximum consecutive modules a track may miss

**Methods:**
- `solve(event: Event) -> List[Track]` — Reconstruct tracks using triplet-based approach
- `solve_batch(events: List[Event]) -> List[List[Track]]` — Batch processing for multiple events

## Validation Framework

### Validator Module

The Validator module provides comprehensive track quality assessment through Monte Carlo truth matching and performance metrics calculation.

#### Functions

**print_validation_summary**
```python
print_validation_summary(
    py_events: List[Dict[str, Any]], 
    py_tracks: List[Track], 
    verbose: bool = True
) -> None
```
Prints detailed validation metrics to stdout including efficiency, purity, and clone/ghost statistics.

**compute_tracking_efficiency**
```python
compute_tracking_efficiency(
    py_events: List[Dict[str, Any]], 
    py_tracks: List[Track], 
    particle_type: str = "all"
) -> float
```
Computes reconstruction efficiency as the ratio of successfully reconstructed to reconstructable tracks.

**export_validation_to_json**
```python
export_validation_to_json(
    py_events: List[Dict[str, Any]], 
    py_tracks: List[Track], 
    verbose: bool = True
) -> Dict[str, Any]
```
Generates JSON-serializable validation metrics for programmatic analysis.

**export_detailed_metrics_json**
```python
export_detailed_metrics_json(
    py_events: List[Dict[str, Any]], 
    py_tracks: List[Track], 
    verbose: bool = True
) -> Dict[str, Any]
```
Produces detailed per-track and per-event metrics in JSON format.

## Optimization Suite

### HyperParameter Framework

The hyperParameterFramework provides a comprehensive optimization suite for systematic parameter tuning and algorithm comparison.

#### EventMetricsCalculator Class

Processes validation results to compute statistical metrics and performance indicators.

**Constructor:**
```python
EventMetricsCalculator(validation_results: ValidationResults) -> EventMetricsCalculator
```

**Methods:**
- `compute_aggregations() -> pd.DataFrame` — Statistical analysis grouped by particle type
- `get_metric(metric: str, stat: str) -> float` — Extract specific performance metrics
- `flatten_aggregations(agg_df: pd.DataFrame) -> Dict[str, float]` — Convert to flat metric dictionary

#### BaseOptimizer Class

Abstract base class for hyperparameter optimization algorithms.

**Constructor:**
```python
BaseOptimizer(
    objective: str = "min",
    auto_eval: Dict[str, Any] = {"autoEval": False, "nested": True, "weights": []}
) -> BaseOptimizer
```

**Methods:**
- `start(algorithm: ReconstructionAlgorithms) -> Dict[str, Any]` — Initialize optimization
- `next() -> Dict[str, Any]` — Generate next parameter configuration
- `is_finished() -> bool` — Check optimization completion status
- `objective_func(weights: List[float], nested: bool) -> float` — Evaluate parameter configuration

#### Pipeline Classes

**PipelineBase** — Abstract pipeline for algorithm execution and validation
**TrackFollowingPipeline** — Pipeline for TrackFollowing algorithm
**GraphDFSPipeline** — Pipeline for GraphDFS algorithm  
**SearchByTripletTriePipeline** — Pipeline for SearchByTripletTrie algorithm

#### Optimization Solvers

**GridSearch** — Systematic grid-based parameter space exploration
**Bayesian** — Gaussian Process-based Bayesian optimization
**ParticleSwarm** — Particle Swarm Optimization for global search
**PolyHoot** — Monte Carlo Tree Search-based polynomial heuristic optimization

### Optimization Solvers

The solvers submodule provides concrete implementations of optimization algorithms for hyperparameter tuning.

#### GridSearch Class

Systematic exploration of the parameter space using a regular grid of points.

**Constructor:**
```python
GridSearch(
    resolution: int = 10,
    objective: Literal["min", "max"] = "min",
    nested: bool = True,
    weights: List[float] = [1.0, 1.0, 1.0, -10.0]
) -> GridSearch
```

**Parameters:**
- `resolution` — Number of discrete points per parameter dimension
- `objective` — Optimization direction ("min" for minimization, "max" for maximization)
- `nested` — Enable nested evaluation with per-track metrics
- `weights` — Multi-objective weighting vector for metric combination

**Methods:**
- `init() -> Dict[str, Any]` — Initialize grid search and return first configuration
- `next() -> Dict[str, Any]` — Advance to next grid point or return best if complete
- `is_finished() -> bool` — Check if all grid points have been evaluated

#### Bayesian Class

Gaussian Process-based Bayesian optimization using expected improvement acquisition.

**Constructor:**
```python
Bayesian(
    learning_rate: float,
    max_iterations: int = 100,
    target_score: float = 0.3,
    objective: Literal["min", "max"] = "min",
    nested: bool = True,
    weights: List[float] = [1.0, 1.0, 1.0, -10.0]
) -> Bayesian
```

**Parameters:**
- `learning_rate` — GP kernel learning rate for hyperparameter updates
- `max_iterations` — Maximum optimization iterations before termination
- `target_score` — Early stopping threshold for objective function
- `objective` — Optimization direction
- `nested` — Enable detailed per-track evaluation
- `weights` — Multi-objective weighting scheme

**Methods:**
- `init() -> Dict[str, Any]` — Initialize with random starting point
- `next() -> Dict[str, Any]` — Use acquisition function to select next evaluation point
- `is_finished() -> bool` — Check termination criteria (iterations or target reached)

#### ParticleSwarm Class

Particle Swarm Optimization for global parameter space exploration.

**Constructor:**
```python
ParticleSwarm(
    swarm_size: int = 20,
    inertia: float = 0.5,
    cognitive: float = 1.5,
    social: float = 1.5,
    max_iterations: int = 100,
    objective: Literal["min", "max"] = "min",
    nested: bool = False,
    weights: List[float] = [1.0, 1.0, 1.0, -10.0]
) -> ParticleSwarm
```

**Parameters:**
- `swarm_size` — Number of particles in the optimization swarm
- `inertia` — Inertia coefficient for velocity updates (w)
- `cognitive` — Personal best influence coefficient (c₁)
- `social` — Global best influence coefficient (c₂)
- `max_iterations` — Maximum PSO iterations
- `objective` — Optimization direction
- `nested` — Enable nested metric evaluation
- `weights` — Multi-objective weighting vector

**Velocity Update Equation:**
```
v(t+1) = w·v(t) + c₁·r₁·(pbest - x(t)) + c₂·r₂·(gbest - x(t))
```

**Methods:**
- `init() -> Dict[str, Any]` — Initialize swarm with random positions and velocities
- `next() -> Dict[str, Any]` — Evaluate particle and update swarm state
- `is_finished() -> bool` — Check convergence or iteration limit

#### PolyHoot Class

Monte Carlo Tree Search-based polynomial heuristic optimization algorithm.

**Constructor:**
```python
PolyHoot(
    max_iterations: int = 100,
    objective: Literal["min", "max"] = "min",
    nested: bool = True,
    weights: List[float] = [1.0, 1.0, 1.0, -10.0]
) -> PolyHoot
```

**Parameters:**
- `max_iterations` — Maximum MCTS iterations for tree expansion
- `objective` — Optimization direction
- `nested` — Enable detailed evaluation metrics
- `weights` — Multi-objective weighting scheme

**Algorithm Overview:**
1. Build binary decision tree over parameter space
2. Use UCT (Upper Confidence bounds applied to Trees) for node selection
3. Expand promising regions through recursive splitting
4. Backpropagate rewards to update node values

**Methods:**
- `init() -> Dict[str, Any]` — Initialize MCTS tree with root node
- `next() -> Dict[str, Any]` — Perform UCT selection, expansion, and backpropagation
- `is_finished() -> bool` — Check iteration limit and perform final backpropagation

## Usage Examples

### Basic Track Reconstruction

**Single Event Processing:**
```python
import json
from velopix.DetectorEvent import Event
from velopix.ReconstructionAlgorithms import TrackFollowing
from velopix.Validator import print_validation_summary

# Load event data
with open("event_data.json", "r") as f:
    event_data = json.load(f)

# Create event object
event = Event(event_data)

# Initialize algorithm with custom parameters
algorithm = TrackFollowing(
    max_slopes=(0.5, 0.5),
    max_tolerance=(0.2, 0.2),
    max_scatter=0.3,
    min_track_length=3,
    min_strong_track_length=4
)

# Reconstruct tracks
tracks = algorithm.solve(event)

# Validate results
print_validation_summary([event_data], tracks, verbose=True)
```

**Batch Processing:**
```python
from velopix.ReconstructionAlgorithms import GraphDFS
import json
import os

# Load multiple events
events = []
event_data = []
for i in range(10):
    filename = f"event_{i}.json"
    with open(filename, "r") as f:
        data = json.load(f)
        event_data.append(data)
        events.append(Event(data))

# Initialize algorithm
algorithm = GraphDFS(
    minimum_root_weight=2,
    weight_assignment_iterations=3,
    clone_ghost_killing=True
)

# Batch reconstruction
all_tracks = algorithm.solve_batch(events)

# Process results
for i, tracks in enumerate(all_tracks):
    print(f"Event {i}: {len(tracks)} tracks reconstructed")
```

### Algorithm Comparison

```python
from velopix.ReconstructionAlgorithms import TrackFollowing, GraphDFS, SearchByTripletTrie
from velopix.Validator import compute_tracking_efficiency

# Initialize algorithms with default parameters
tf_algo = TrackFollowing()
gd_algo = GraphDFS()
st_algo = SearchByTripletTrie()

algorithms = [
    ("TrackFollowing", tf_algo),
    ("GraphDFS", gd_algo),
    ("SearchByTripletTrie", st_algo)
]

# Compare performance on test events
for name, algorithm in algorithms:
    tracks = algorithm.solve(event)
    efficiency = compute_tracking_efficiency([event_data], tracks)
    print(f"{name}: {efficiency:.3f} efficiency")
```

### Hyperparameter Optimization

**Grid Search Optimization:**
```python
from velopix.hyperParameterFramework import TrackFollowingPipeline
from velopix.hyperParameterFramework.solvers import GridSearch
import json

# Load training events
events = []
for i in range(50):
    with open(f"training_event_{i}.json", "r") as f:
        events.append(json.load(f))

# Initialize optimizer
optimizer = GridSearch(
    resolution=5,
    objective="min",
    nested=True,
    weights=[1.0, 1.0, 1.0, -10.0]  # [efficiency, purity, clone_rate, ghost_penalty]
)

# Create pipeline
pipeline = TrackFollowingPipeline(events, intra_node=True)

# Run optimization
best_config = None
while not optimizer.is_finished():
    config = optimizer.next()
    
    # Run pipeline with current configuration
    algorithm = pipeline.model(config)
    tracks = []
    for event in pipeline.events:
        tracks.extend(algorithm.solve(event))
    
    # Evaluate performance
    pipeline.tracks = tracks
    pipeline.run(overwrite=True)
    
    # Update optimizer
    optimizer.add_run(pipeline.results)
    
    if optimizer.best_config:
        best_config = optimizer.best_config

print(f"Best configuration: {best_config}")
print(f"Best score: {optimizer.best_score}")
```

**Bayesian Optimization:**
```python
from velopix.hyperParameterFramework.solvers import Bayesian

# Initialize Bayesian optimizer
optimizer = Bayesian(
    learning_rate=0.1,
    max_iterations=100,
    target_score=0.95,
    objective="max"
)

# Optimization loop
pipeline = GraphDFSPipeline(events, intra_node=True)
iteration = 0

while not optimizer.is_finished():
    config = optimizer.next()
    
    # Evaluate configuration
    algorithm = pipeline.model(config)
    all_tracks = algorithm.solve_batch(pipeline.events)
    
    # Flatten tracks for validation
    tracks = [track for event_tracks in all_tracks for track in event_tracks]
    pipeline.tracks = tracks
    pipeline.run(overwrite=True)
    
    # Update optimizer
    optimizer.add_run(pipeline.results)
    
    iteration += 1
    print(f"Iteration {iteration}: Score = {optimizer.best_score:.4f}")

print(f"Optimization completed in {iteration} iterations")
print(f"Final configuration: {optimizer.get_optimised_pMap()}")
```

### Advanced Validation and Metrics

```python
from velopix.Validator import export_detailed_metrics_json
from velopix.hyperParameterFramework import EventMetricsCalculator
import pandas as pd

# Reconstruct tracks
algorithm = SearchByTripletTrie(max_scatter=0.05)
tracks = algorithm.solve(event)

# Export detailed metrics
detailed_metrics = export_detailed_metrics_json([event_data], tracks, verbose=True)

# Analyze with EventMetricsCalculator
calculator = EventMetricsCalculator(detailed_metrics)
aggregations = calculator.compute_aggregations()

# Extract specific metrics
efficiency_mean = calculator.get_metric("efficiency", "mean")
purity_std = calculator.get_metric("purity", "std")
clone_rate = calculator.get_metric("clone_percentage", "median")

print(f"Efficiency: {efficiency_mean:.3f} ± {calculator.get_metric('efficiency', 'std'):.3f}")
print(f"Purity: {calculator.get_metric('purity', 'mean'):.3f} ± {purity_std:.3f}")
print(f"Clone rate: {clone_rate:.3f}%")

# Convert to DataFrame for further analysis
df = calculator.df_events
print(df.describe())
```

### Custom Pipeline Implementation

```python
from velopix.hyperParameterFramework import PipelineBase
from velopix.hyperParameterFramework._velopixTypes import ReconstructionAlgorithms

class CustomPipeline(PipelineBase):
    name = ReconstructionAlgorithms.TRACK_FOLLOWING
    
    def model(self, pmap):
        # Custom parameter mapping
        return TrackFollowing(
            max_slopes=(pmap.get('slope_x', 0.7), pmap.get('slope_y', 0.7)),
            max_tolerance=(pmap.get('tol_x', 0.4), pmap.get('tol_y', 0.4)),
            max_scatter=pmap.get('scatter', 0.4),
            min_track_length=int(pmap.get('min_length', 3)),
            min_strong_track_length=int(pmap.get('strong_length', 4))
        )

# Use custom pipeline
custom_pipeline = CustomPipeline(events, intra_node=True)
custom_config = {
    'slope_x': 0.6, 'slope_y': 0.6,
    'tol_x': 0.3, 'tol_y': 0.3,
    'scatter': 0.35,
    'min_length': 3, 'strong_length': 5
}

algorithm = custom_pipeline.model(custom_config)
tracks = algorithm.solve_batch(custom_pipeline.events)
```
