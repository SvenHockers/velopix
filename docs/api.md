# Velopix API Reference
## Table of Contents

1. [Package Structure](#package-structure)

   * [`velopix.DetectorEvent`](#velopixdetectorevent)
   * [`velopix.hyperParameterFramework.solvers`](#velopixhyperparameterframeworksolvers)
   * [`velopix.ReconstructionAlgorithms`](#velopixreconstructionalgorithms)
   * [`velopix.Validator`](#velopixvalidator)
2. [Module Details](#module-details)

   * [DetectorEvent](#detectorevent)
   * [hyperParameterFramework.solvers](#hyperparameterframeworksolvers)
   * [ReconstructionAlgorithms](#reconstructionalgorithms)
   * [Validator](#validator)
3. [Examples](#examples)

---

## Package Structure

Below is a high-level view of the package layout:

```text
velopix/
├── DetectorEvent/
├── hyperParameterFramework/
│   └── solvers/
├── ReconstructionAlgorithms/
└── Validator/
```

## Module Details

### velopix.DetectorEvent

Module for managing detector hits, tracks, and modules within an event.

#### Classes

* **Hit**

  * **Attributes:**

    * `id: int` — Unique identifier of the hit.
    * `x: float` — X-coordinate of the hit.
    * `y: float` — Y-coordinate of the hit.
    * `z: float` — Z-coordinate of the hit.
    * `t: float` — Time of the hit.
    * `module_number: int` — Index of the module where the hit occurred.
    * `with_t: bool` — Whether time information is included.
  * **Constructor:**

    ```python
    Hit(
        x: float,
        y: float,
        z: float,
        hit_id: int,
        module: Optional[int] = None,
        t: Optional[float] = None,
        with_t: Optional[bool] = None
    ) -> None
    ```

* **Track**

  * **Attributes:**

    * `hits: List[Hit]` — Hits composing the track.
    * `missed_last_module: bool` — Whether the track missed the last module.
    * `missed_penultimate_module: bool` — Whether the track missed the penultimate module.
  * **Constructor:**

    ```python
    Track(hits: List[Hit]) -> None
    ```
  * **Methods:**

    * `add_hit(hit: Hit) -> None` — Add a hit to the track.

* **Module**

  * **Attributes:**

    * `module_number: int` — Module index.
    * `z: float` — Z-position of the module.
    * `hit_start_index: int` — Start index in the global hit list.
    * `hit_end_index: int` — End index in the global hit list.
    * `global_hits: List[Hit]` — Reference to the global list of hits.
  * **Constructor:**

    ```python
    Module(
        module_number: int,
        z: float,
        hit_start_index: int,
        hit_end_index: int,
        global_hits: List[Hit]
    ) -> None
    ```
  * **Methods:**

    * `hits() -> List[Hit]` — Retrieve hits in this module.

* **Event**

  * **Attributes:**

    * `description: str` — Textual description of the event.
    * `montecarlo: object` — Monte Carlo truth information.
    * `module_prefix_sum: List[int]` — Cumulative counts of hits per module.
    * `number_of_hits: int` — Total number of hits in the event.
    * `module_zs: List[List[float]]` — Z-positions grouped by module.
    * `hits: List[Hit]` — All hits in the event.
    * `modules: List[Module]` — Module objects representing detector layers.
  * **Constructor:**

    ```python
    Event(json_data: object) -> None
    ```

### velopix.hyperParameterFramework

Module grouping utilities for hyper-parameter optimization, validation metrics, pipeline orchestration, and associated types.

#### Submodules

* **solvers** — Hyper-parameter optimization algorithms (`GridSearch`, `Bayesian`, `ParticleSwarm`).
* **metrics** — Validation metrics computation (`EventMetricsCalculator`).
* **optimizers** — Base class for optimizers (`BaseOptimizer`).
* **pipeline** — Pipeline classes to run reconstruction with given parameters (`PipelineBase`, `TrackFollowingPipeline`, `GraphDFSPipeline`, `SearchByTripletTriePipeline`).
* **types** — Type aliases for configuration maps, events, and results.

---

#### Metrics (`velopix.hyperParameterFramework.metrics`)

* **EventMetricsCalculator**

  * **Constructor:**

    ```python
    EventMetricsCalculator(validation_results: ValidationResults) -> None
    ```

    * `validation_results` — Nested dict of per-event and per-track metrics.
  * **Attributes:**

    * `df_events: pd.DataFrame` — Flattened DataFrame of all event entries.
  * **Methods:**

    * `_create_events_dataframe() -> pd.DataFrame` — Build dataframe from validation results.
    * `compute_aggregations() -> pd.DataFrame` — Group by particle label, compute stats (mean, std, median, q25, q75, skew, kurtosis, IQR).
    * `flatten_aggregations(agg_df: pd.DataFrame) -> MetricsDict` — Convert multi-index DataFrame to flat metrics dict.
    * `compute_average_metric(metrics: MetricsDict, col: str, stat: str) -> float` — Average a specific metric/statistic.
    * `get_metric(metric: str = "clone_percentage", stat: str = "std") -> float` — Shortcut to compute a single average metric.

---

#### Optimizers (`velopix.hyperParameterFramework.optimizers`)

* **BaseOptimizer**

  * **Constructor:**

    ```python
    BaseOptimizer(
        objective: str = "min",
        auto_eval: dict[str, Any] = {"autoEval": False, "nested": True, "weights": []}
    ) -> None
    ```

    * `objective` — "min" or "max" to indicate optimization direction.
    * `auto_eval` — Dict controlling automatic evaluation after each `next` (keys: `autoEval`, `nested`, `weights`).
  * **Attributes:**

    * `objective: str` — Optimization direction.
    * `best_score: float` — Current best objective value.
    * `best_config: pMap` — Best parameter map found.
    * `auto_evaluate: bool` — Whether to automatically evaluate each step.
    * `nested: bool`, `weights: List[float]`, `score_history: List[float]`, `history: Dict[str, Any]`, `prev_config: pMap`, `run: ValidationResults` (when auto-eval enabled).
  * **Methods:**

    * `validate_config(config: pMap, schema: pMapType) -> bool` — Static config/schema validator.
    * `start(algorithm: ReconstructionAlgorithms) -> pMap` — Initialize and return first parameter map.
    * `next_pMap() -> pMap` — Alias for advancing one optimization step.
    * `add_run(results: ValidationResults) -> None` — Record results of an evaluation.
    * `get_optimised_pMap() -> pMap` — Retrieve best parameter map.
    * `get_run_data() -> ValidationResults` — Retrieve full run history.
    * `init() -> pMap`, `next() -> pMap`, `is_finished() -> bool` — Interface for algorithm implementations.
    * `objective_func(weights: Sequence[float], nested: bool = True) -> float | int` — Evaluate current parameters via validation pipeline.
    * `_evaluate_run(validationResult: ValidationResults, weight: List[float], nested: bool = False) -> None` — Internal scoring and logging.

---

#### Pipeline (`velopix.hyperParameterFramework.pipeline`)

* **PipelineBase**

  * **Constructor:**

    ```python
    PipelineBase(
        events: EventType,
        intra_node: bool,
        parameter_map: List[pMap] | None = None
    ) -> None
    ```

    * `events` — List of event dicts.
    * `intra_node` — Whether to compute nested (per-track) metrics.
    * `parameter_map` — Optional list of parameter maps to evaluate.
  * **Attributes:**

    * `name: ReconstructionAlgorithms` — Algorithm enum value.
    * `json_events: EventType`, `nested: bool`, `parameters: List[pMap] | None`, `events: List[Event]`.
    * `tracks: List[Track]` — Populated after running model.
    * `results: ValidationResults` — Populated when running validation.
  * **Methods:**

    * `model(pmap: pMap) -> ReconstructionAlgorithmsType` — Abstract factory for algorithm instance.
    * `run(overwrite: bool) -> None` — Execute reconstruction and validation; manages persistence.
    * `print_validation(parameters: pMap | None = None, verbose: bool = True) -> None` — Print validation summary.

* **TrackFollowingPipeline**, **GraphDFSPipeline**, **SearchByTripletTriePipeline**

  * Concrete implementations setting `name` and overriding `model(pmap)` to return the corresponding algorithm with parameters extracted from `pmap`.

---

#### Types (`velopix.hyperParameterFramework.types`)

* `MetricsDict` — `dict[str, int | float | bool]`
* `pMapType` — `dict[str, tuple[type, Any]]`
* `pMap` — `dict[str, int | float | bool]`
* `EventType` — `list[dict[str, Any]]`
* `ReconstructionAlgorithmsType` — `Union[TrackFollowing, GraphDFS, SearchByTripletTrie]`
* `ValidationResults`, `ValidationResultsNested` — Nested dict types for validation outputs.
* `ConfigType` — `dict[str, tuple[Union[type[float], type[int], type[bool]], Any]]`
* `boundType` — `dict[str, tuple[Union[int, float], Union[int, float]] | Any]`

### velopix.hyperParameterFramework.solvers

Module providing hyper-parameter optimization solvers following a common `BaseOptimizer` interface.

#### Classes

* **GridSearch**

  * **Constructor:**

    ```python
    GridSearch(
        resolution: int = 10,
        objective: Literal["min", "max"] = "min",
        nested: bool = True,
        weights: List[float] = [1.0, 1.0, 1.0, -10.0]
    ) -> None
    ```

    * `resolution` — Number of discrete points per hyperparameter axis.
    * `objective` — Objective direction: "min" or "max".
    * `nested` — Whether to perform nested grid search.
    * `weights` — Weight vector for multi-objective evaluation.
  * **Attributes:**

    * `_resolution: int` — Current grid resolution.
    * `_stopped: bool` — Flag indicating completion of search.
    * `_total_hypotheses: int` — Total number of configurations to evaluate.
    * `_last_config: Dict[str, Any]` — Most recent parameter configuration.
    * `_best_config: Dict[str, Any]` — Best configuration found so far.
    * `_best_score: float` — Best objective score observed.
  * **Methods:**

    * `init() -> Dict[str, Any]` — Initialize search and return the first configuration.
    * `next() -> Dict[str, Any]` — Advance to next configuration or return best if finished.
    * `is_finished() -> bool` — Check if search has exhausted all configurations.

* **Bayesian**

  * **Constructor:**

    ```python
    Bayesian(
        learning_rate: float,
        max_iterations: int = 100,
        target_score: float = 0.3,
        objective: Literal["min", "max"] = "min",
        nested: bool = True,
        weights: List[float] = [1.0, 1.0, 1.0, -10.0]
    ) -> None
    ```

    * `learning_rate` — Learning rate for Gaussian Process updates.
    * `max_iterations` — Maximum number of optimization iterations.
    * `target_score` — Early stopping threshold for objective.
    * `objective` — Objective direction: "min" or "max".
    * `nested` — Whether nested evaluation is enabled.
    * `weights` — Weight vector for multi-objective evaluation.
  * **Attributes:**

    * `current_iteration: int` — Iteration counter.
    * `X: List[List[Any]]` — Evaluated parameter vectors.
    * `Y: List[float]` — Corresponding objective scores.
    * `gpr: GaussianProcessRegressor` — Underlying GP model.
    * `_stopped: bool` — Flag indicating completion.
    * `_current_config: Dict[str, Any]` — Last sampled configuration.
  * **Methods:**

    * `init() -> Dict[str, Any]` — Initialize and return a random starting configuration.
    * `next() -> Dict[str, Any]` — Evaluate last config, update GP, and return next candidate.
    * `_predict_next() -> Dict[str, Any]` — Use acquisition (expected improvement) to select next point.
    * `_sample_random_point() -> Dict[str, Any]` — Sample a random point within hyperparameter bounds.
    * `is_finished() -> bool` — Check if `current_iteration >= max_iterations`.

* **ParticleSwarm**

  * **Constructor:**

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
    ) -> None
    ```

    * `swarm_size` — Number of particles in the swarm.
    * `inertia` — Inertia coefficient for velocity update.
    * `cognitive` — Cognitive (personal) coefficient.
    * `social` — Social (global) coefficient.
    * `max_iterations` — Maximum number of PSO iterations.
    * `objective` — Objective direction: "min" or "max".
    * `nested` — Whether nested evaluation is enabled.
    * `weights` — Weight vector for multi-objective evaluation.
  * **Attributes:**

    * `swarm_size: int`
    * `inertia: float`
    * `cognitive: float`
    * `social: float`
    * `max_iterations: int`
    * `current_iteration: int`
    * `nested: bool`
    * `weights: List[float]`
    * `target_score: float`
    * `swarm: List[Dict[str, Any]]` — Current particle positions.
    * `velocities: List[Dict[str, float]]` — Velocity vectors.
    * `pbest_positions: List[Dict[str, Any]]` — Personal best positions.
    * `pbest_scores: List[float]` — Personal best scores.
    * `gbest_position: Dict[str, Any]` — Global best position.
    * `gbest_score: float` — Global best score.
    * `current_particle_index: int` — Index of particle to evaluate next.
    * `_current_config: Dict[str, Any]` — Configuration currently under evaluation.
    * `_stopped: bool` — Flag indicating completion.
  * **Methods:**

    * `init() -> Dict[str, Any]` — Initialize swarm, velocities, personal bests; return first particle.
    * `next() -> Dict[str, Any]` — Evaluate current particle, update pbest/gbest, advance or update swarm; return next config.
    * `_update_velocities_positions() -> None` — Perform velocity and position updates after each full swarm evaluation.
    * `is_finished() -> bool` — Check if PSO has reached stopping criteria.

## velopix.ReconstructionAlgorithms

Module providing track reconstruction algorithms leveraging different graph and triplet-based approaches.

#### Classes

* **TrackFollowing**

  * **Constructor:**

    ```python
    TrackFollowing(
        max_slopes: Optional[Tuple[float, float]] = None,
        max_tolerance: Optional[Tuple[float, float]] = None,
        max_scatter: Optional[float] = None,
        min_track_length: Optional[int] = None,
        min_strong_track_length: Optional[int] = None
    ) -> None
    ```

    * `max_slopes` — Maximum allowed slope values as (dx/dz, dy/dz).
    * `max_tolerance` — Maximum tolerance for track bending as (tol\_x, tol\_y).
    * `max_scatter` — Maximum allowed scattering angle.
    * `min_track_length` — Minimum number of hits for a track.
    * `min_strong_track_length` — Minimum number of hits for a strong-quality track.
  * **Methods:**

    * `are_compatible(hit0: Hit, hit1: Hit) -> bool` — Check if two hits are slope-compatible.
    * `check_tolerance(hit0: Hit, hit1: Hit, hit2: Hit) -> bool` — Verify tolerance across three hits.
    * `solve(event: Event) -> List[Track]` — Reconstruct tracks for a single event.
    * `solve_batch(events: List[Event]) -> List[Track]` — Batch reconstruction over multiple events.

* **GraphDFS**

  * **Constructor:**

    ```python
    GraphDFS(
        max_slopes: Optional[Tuple[float, float]] = None,
        max_tolerance: Optional[Tuple[float, float]] = None,
        max_scatter: Optional[float] = None,
        minimum_root_weight: Optional[int] = None,
        weight_assignment_iterations: Optional[int] = None,
        allowed_skip_modules: Optional[int] = None,
        allow_cross_track: Optional[bool] = None,
        clone_ghost_killing: Optional[bool] = None
    ) -> None
    ```

    * `minimum_root_weight` — Minimum weight to consider a root node in graph.
    * `weight_assignment_iterations` — Iterations for assigning weights.
    * `allowed_skip_modules` — Number of modules permitted without hits.
    * `allow_cross_track` — Whether tracks may cross in projection.
    * `clone_ghost_killing` — Enable ghost track removal via cloning.
  * **Methods:**

    * `solve(event: Event) -> List[Track]`
    * `solve_batch(events: List[Event]) -> List[Track]`

* **SearchByTripletTrie**

  * **Constructor:**

    ```python
    SearchByTripletTrie(
        max_scatter: Optional[float] = None,
        min_strong_track_length: Optional[int] = None,
        allowed_missed_modules: Optional[int] = None
    ) -> None
    ```

    * `max_scatter` — Maximum scatter allowed for triplet chaining.
    * `min_strong_track_length` — Minimum hits for a strong track.
    * `allowed_missed_modules` — Number of consecutive modules a track may skip.
  * **Methods:**

    * `solve(event: Event) -> List[Track]`
    * `solve_batch(events: List[Event]) -> List[Track]`

### velopix.Validator

Module providing functions to validate reconstructed tracks against Monte Carlo truth and export metrics.

#### Functions

* `print_validation_summary(py_events: List[Dict[str, Any]], py_tracks: List[Track], verbose: bool) -> None`

  * Print a summary of validation metrics to stdout.
  * `py_events` — List of event dictionaries (Monte Carlo truth).
  * `py_tracks` — List of reconstructed `Track` objects.
  * `verbose` — If `True`, include detailed per-track output.

* `compute_tracking_efficiency(py_events: List[Dict[str, Any]], py_tracks: List[Track], particle_type: str) -> float`

  * Compute the fraction of true particle tracks successfully reconstructed.
  * `particle_type` — Type of particle to compute efficiency for (e.g. "muon").
  * **Returns:** Efficiency as a float between 0 and 1.

* `export_validation_to_json(py_events: List[Dict[str, Any]], py_tracks: List[Track], verbose: bool) -> Dict[str, Any]`

  * Generate a JSON-serializable dictionary of summary validation metrics.
  * **Returns:** Dictionary with overall metrics and optional detailed lists.

* `export_detailed_metrics_json(py_events: List[Dict[str, Any]], py_tracks: List[Track], verbose: bool) -> Dict[str, Any]`

  * Generate a JSON-serializable dictionary containing detailed per-track and per-event metrics.
  * **Returns:** Nested dictionary of detailed metrics by event and track.

## Examples

**Simple Track Reconstruction Example**
```python
from velopix.DetectorEvent import Event
from velopix.ReconstructionAlgorithms import TrackFollowing
from velopix.Validator import print_validation_summary

event = Event(json.loads("path_to_the_event.json"))

TF = TrackFolling((0.5, 0.5), (0.2, 0.2), 0.7)
reconstructed_tracks = TF.solve()

print_validation_summary(event, reconstructed_tracks)
```

**Simple Hyperparameter Optimisation Example**
```python
from velopix.DetectorEvent import Event
from velopix.hyperParameterFramework import TrackFollowingPipeline, solvers

events = []
    for i in range(num_events):
        filename = f"velo_event_{i}.json"
        filepath = os.path.join(directory, filename)

        try:
            with open(filepath, "r") as f:
                events.append(json.load(f))
            logger.info("Loaded file: %s", filename)
        except FileNotFoundError:
            logger.error("File not found: %s", filepath)
        except json.JSONDecodeError as e:
            logger.error("Invalid JSON in %s: %s", filepath, e)

optimiser = solvers.GridSearch(3, "min", True, [1.0, 1.0, 1.0, -10.0])
pipeline = TrackFollowingPipeline(events, True)

optimal_parameters = pipeline.optimise_parameters(
        optimiser,
        max_runs=1_000,
    )

print(optimal_parameters) # Outputs the best set of hyperparameters found
print(optimiser.history) # This will output hyperparameters, scores and metadata (direct validation results)
```
