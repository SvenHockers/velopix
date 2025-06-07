# System Overview

Velopix follows a modular design:

* **DetectorEvent**: Represents events, modules, hits, and particles.
* **ReconstructionAlgorithms**: Plug-and-play components implementing different reconstruction strategies.
* **Pipelines**: High-level interface for running algorithms on input data.
* **Validator**: Evaluate reconstruction performance and correctness.
* **Solvers**: Implementation of Hyperparameter reconstruction algorithm optimisation tools.

This architecture supports experimentation, testing, and integration into broader data workflows.

---
### **4. System Architecture Overview**  

The framework is structured into modular components to facilitate **performance optimization and algorithm development**. The following diagram illustrates the interaction between key modules. For a more detailed overview, see [this reference](./docs/abstractions_diagram.md).

```mermaid
classDiagram
class DetectorEvent {<<Rust Module>>
- Event
- Hit
- Module
- MCParticle
- Efficiency} 
class ReconstructionAlgorithms {<<Rust Module>>
- Track Following
- Graph DFS
- Search By Triplet Trie} 
class Validator {<<Rust Module>>
- print_validation_summary()
- compute_tracking_efficiency()
- export_validation_to_json()
- export_detailed_metrics_json()
}
class hyperParameterFramework - Module {<<Rust Module>>
- PipelineBase
- OptimiserBase
- EventMetricsCalculator
- ReconstructionAlgorithms} 
class hyperParameterFramework - Pipeline {<<Python Wrapper>>
 - Calls optimized Rust algorithms
} 
class hyperParameterFramework.solvers {<<Optimizer Implementation>>
    + GridSearch
    + Bayesian
    + ParticleSwarm
    + PolyHoot
    + __init__()
    + start()
    + is_finished()
    + objective_func()}

DetectorEvent --> ReconstructionAlgorithms : interacts with
DetectorEvent --> hyperParameterFramework - Module : uses
DetectorEvent --> hyperParameterFramework - Pipeline : creates events for
hyperParameterFramework - Module <-- hyperParameterFramework - Pipeline : uses 
hyperParameterFramework - Module <-- Validator : uses

ReconstructionAlgorithms --> hyperParameterFramework - Module : uses metrics from
ReconstructionAlgorithms *-- hyperParameterFramework - Pipeline : creates pipelines for

hyperParameterFramework - Module <|.. hyperParameterFramework.solvers : Implements Optimizer
hyperParameterFramework - Pipeline <.. hyperParameterFramework.solvers : Selects Reconstruction Algorithm
```
---