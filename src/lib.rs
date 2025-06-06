use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod algorithms;
mod event_model;
mod validator;

// Bring in your structs and functions:
use crate::event_model::hit::Hit;
use crate::event_model::track::Track;
use crate::event_model::module::Module;
use crate::event_model::event::Event;
use crate::algorithms::track_following::TrackFollowing;
use crate::algorithms::graph_dfs::GraphDFS;
use crate::algorithms::search_by_triplet_trie::SearchByTripletTrie;
use crate::validator::validator::{
    print_validation_summary,
    compute_tracking_efficiency,
    export_validation_to_json,
    export_detailed_metrics_json,
};

#[pymodule]
#[pyo3(name = "velopix")]
fn internal_velopix(py: Python, m: &PyModule) -> PyResult<()> {
    // 1) Register ReconstructionAlgorithms submodule
    let algo_mod = PyModule::new(py, "ReconstructionAlgorithms")?;
    algo_mod.add_class::<TrackFollowing>()?;
    algo_mod.add_class::<GraphDFS>()?;
    algo_mod.add_class::<SearchByTripletTrie>()?;
    m.add_submodule(algo_mod)?;

    // 2) Register Validator submodule
    let val_mod = PyModule::new(py, "Validator")?;
    val_mod.add_function(wrap_pyfunction!(print_validation_summary, val_mod)?)?;
    val_mod.add_function(wrap_pyfunction!(compute_tracking_efficiency, val_mod)?)?;
    val_mod.add_function(wrap_pyfunction!(export_validation_to_json, val_mod)?)?;
    val_mod.add_function(wrap_pyfunction!(export_detailed_metrics_json, val_mod)?)?;
    m.add_submodule(val_mod)?;

    // 3) Register DetectorEvent submodule
    let det_mod = PyModule::new(py, "DetectorEvent")?;
    det_mod.add_class::<Hit>()?;
    det_mod.add_class::<Track>()?;
    det_mod.add_class::<Module>()?;
    det_mod.add_class::<Event>()?;
    m.add_submodule(det_mod)?;

    Ok(())
}