use pyo3::prelude::*;

mod core;
mod flight;
mod icing;
mod layers;
mod math;
mod thermo;
mod wind;

mod bindings;

#[pymodule]
fn _core(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // ISA class
    m.add_class::<bindings::isa::ISA>()?;

    // Submodules of functions
    bindings::math::math_bindings(py, m)?;
    bindings::flight::flight_bindings(py, m)?;
    bindings::wind::wind_bindings(py, m)?;
    bindings::thermo::thermo_bindings(py, m)?;
    bindings::icing::icing_bindings(py, m)?;

    Ok(())
}
