use crate::flight;
use pyo3::prelude::*;

#[pymodule]
pub fn flight_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(altitude_to_fl, m)?)?;
    m.add_function(wrap_pyfunction!(fl_to_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(geometric_to_fl, m)?)?;
    m.add_function(wrap_pyfunction!(indicated_altitude, m)?)?;
    Ok(())
}

#[pyfunction]
fn altitude_to_fl(h_p: f64) -> f64 {
    flight::altitude_to_fl(h_p)
}
#[pyfunction]
fn fl_to_altitude(fl: f64) -> f64 {
    flight::fl_to_altitude(fl)
}
#[pyfunction]
fn geometric_to_fl(h: f64, p: f64) -> f64 {
    flight::geometric_to_fl(h, p)
}
#[pyfunction]
fn indicated_altitude(h: f64, p: f64, qnh: f64) -> f64 {
    flight::indicated_altitude(h, p, qnh)
}
