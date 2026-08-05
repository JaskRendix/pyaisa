use crate::performance;
use pyo3::prelude::*;

#[pymodule]
pub fn performance_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction_bound!(drag_polar, m)?)?;
    m.add_function(wrap_pyfunction_bound!(thrust_lapse, m)?)?;
    m.add_function(wrap_pyfunction_bound!(drag_force, m)?)?;
    m.add_function(wrap_pyfunction_bound!(climb_rate, m)?)?;
    m.add_function(wrap_pyfunction_bound!(service_ceiling, m)?)?;
    Ok(())
}

#[pyfunction]
fn drag_polar(cd0: f64, k: f64, cl: f64) -> f64 {
    performance::drag_polar(cd0, k, cl)
}

#[pyfunction]
fn thrust_lapse(thrust_sl: f64, t: f64, p: f64) -> f64 {
    performance::thrust_lapse(thrust_sl, t, p)
}

#[pyfunction]
fn drag_force(q: f64, s: f64, cd: f64) -> f64 {
    performance::drag_force(q, s, cd)
}

#[pyfunction]
fn climb_rate(thrust: f64, drag: f64, v: f64, weight: f64) -> f64 {
    performance::climb_rate(thrust, drag, v, weight)
}

#[pyfunction]
fn service_ceiling(rc: f64) -> bool {
    performance::service_ceiling(rc)
}
