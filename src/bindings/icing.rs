use crate::icing;
use pyo3::prelude::*;

#[pymodule]
pub fn icing_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lwc, m)?)?;
    m.add_function(wrap_pyfunction!(supercooled_fraction, m)?)?;
    m.add_function(wrap_pyfunction!(icing_severity, m)?)?;
    m.add_function(wrap_pyfunction!(freezing_fraction, m)?)?;
    Ok(())
}

#[pyfunction]
fn lwc(t: f64, rh: f64) -> f64 {
    icing::lwc(t, rh)
}
#[pyfunction]
fn supercooled_fraction(t: f64) -> f64 {
    icing::supercooled_fraction(t)
}
#[pyfunction]
fn icing_severity(t: f64, rh: f64) -> f64 {
    icing::icing_severity(t, rh)
}
#[pyfunction]
fn freezing_fraction(t: f64) -> f64 {
    icing::freezing_fraction(t)
}
