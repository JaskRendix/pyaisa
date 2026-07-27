use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};

mod core;
mod flight;
mod icing;
mod layers;
mod math;
mod thermo;
mod wind;

use core::IsaCore;

#[pyclass]
pub struct ISA {
    core: IsaCore,
    params: Py<PyDict>,
}

fn get_f64(k: &Bound<'_, PyDict>, name: &str, default: f64) -> f64 {
    k.get_item(name)
        .ok()
        .flatten()
        .and_then(|v| v.extract::<f64>().ok())
        .unwrap_or(default)
}

fn get_usize(k: &Bound<'_, PyDict>, name: &str, default: usize) -> usize {
    k.get_item(name)
        .ok()
        .flatten()
        .and_then(|v| v.extract::<usize>().ok())
        .unwrap_or(default)
}

#[pymethods]
impl ISA {
    #[new]
    fn new(py: Python, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let r = kwargs
            .map(|k| get_f64(k, "R", 287.05287))
            .unwrap_or(287.05287);
        let g = kwargs.map(|k| get_f64(k, "g", 9.80665)).unwrap_or(9.80665);
        let t0 = kwargs.map(|k| get_f64(k, "T0", 288.15)).unwrap_or(288.15);
        let p0 = kwargs
            .map(|k| get_f64(k, "p0", 101325.0))
            .unwrap_or(101325.0);
        let psize = kwargs.map(|k| get_usize(k, "psize", 1000)).unwrap_or(1000);

        let layers: Option<Bound<'_, PyDict>> = if let Some(k) = kwargs {
            if let Some(obj) = k.get_item("layers").ok().flatten() {
                obj.downcast::<PyDict>().ok().map(|d| d.clone())
            } else {
                None
            }
        } else {
            None
        };

        let (hl, al) = if let Some(ld) = layers {
            let h_any = ld
                .get_item("h")?
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("layers['h'] missing"))?;
            let a_any = ld
                .get_item("a")?
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("layers['a'] missing"))?;

            let h_arr: &PyArray1<f64> = h_any.extract()?;
            let a_arr: &PyArray1<f64> = a_any.extract()?;

            (h_arr.to_vec()?, a_arr.to_vec()?)
        } else {
            (
                vec![0.0, 11000.0, 20000.0, 32000.0],
                vec![-0.0065, 0.0, 0.001],
            )
        };

        let core = IsaCore::new(r, g, hl, al, t0, p0, psize);

        let params = kwargs
            .map(|k| Py::from(k.to_owned()))
            .unwrap_or_else(|| PyDict::new_bound(py).unbind());

        Ok(ISA { core, params })
    }

    #[getter]
    fn params(&self, py: Python) -> Py<PyDict> {
        self.params.clone_ref(py)
    }

    fn atm(&self, py: Python, h: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        if let Ok(val) = h.extract::<f64>() {
            if let Some((t, p, rho)) = self.core.atm_scalar(val) {
                return Ok(PyTuple::new_bound(py, &[t, p, rho]).unbind().into());
            } else {
                let warnings = py.import_bound("warnings")?;
                warnings.call_method1("warn", ("Altitude value outside range",))?;
                return Ok(PyTuple::new_bound(py, &[f64::NAN, f64::NAN, f64::NAN])
                    .unbind()
                    .into());
            }
        }

        if let Ok(arr) = h.extract::<&PyArray1<f64>>() {
            let hs = unsafe { arr.as_slice()? };
            let (t, p, rho, error) = self.core.atm_vec(hs);

            if error {
                let warnings = py.import_bound("warnings")?;
                warnings.call_method1("warn", ("Altitude value outside range",))?;
            }

            return Ok(PyTuple::new_bound(
                py,
                &[
                    t.to_pyarray_bound(py).unbind(),
                    p.to_pyarray_bound(py).unbind(),
                    rho.to_pyarray_bound(py).unbind(),
                ],
            )
            .unbind()
            .into());
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "h must be float or 1D numpy array",
        ))
    }

    fn layer_at(&self, _py: Python, h: f64) -> Option<usize> {
        self.core.layer_at(h)
    }

    fn atm_deviation(
        &self,
        py: Python,
        h: &Bound<'_, PyAny>,
        d_t: f64,
        dp: f64,
        drho: f64,
    ) -> PyResult<PyObject> {
        if let Ok(val) = h.extract::<f64>() {
            if let Some((t, p, rho)) = self.core.atm_deviation_scalar(val, d_t, dp, drho) {
                return Ok(PyTuple::new_bound(py, &[t, p, rho]).unbind().into());
            } else {
                let warnings = py.import_bound("warnings")?;
                warnings.call_method1("warn", ("Altitude value outside range",))?;
                return Ok(PyTuple::new_bound(py, &[f64::NAN, f64::NAN, f64::NAN])
                    .unbind()
                    .into());
            }
        }

        if let Ok(arr) = h.extract::<&PyArray1<f64>>() {
            let hs = unsafe { arr.as_slice()? };
            let (t, p, rho, error) = self.core.atm_deviation_vec(hs, d_t, dp, drho);

            if error {
                let warnings = py.import_bound("warnings")?;
                warnings.call_method1("warn", ("Altitude value outside range",))?;
            }

            return Ok(PyTuple::new_bound(
                py,
                &[
                    t.to_pyarray_bound(py).unbind(),
                    p.to_pyarray_bound(py).unbind(),
                    rho.to_pyarray_bound(py).unbind(),
                ],
            )
            .unbind()
            .into());
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "h must be float or 1D numpy array",
        ))
    }
}

#[pyfunction]
fn speed_of_sound(t: f64) -> f64 {
    math::speed_of_sound(t)
}

#[pyfunction]
fn dynamic_pressure(rho: f64, v: f64) -> f64 {
    math::dynamic_pressure(rho, v)
}

#[pyfunction]
fn mach(v: f64, a: f64) -> f64 {
    math::mach(v, a)
}

#[pyfunction]
fn pressure_altitude(p: f64) -> f64 {
    math::pressure_altitude(p)
}

#[pyfunction]
fn density_altitude(p: f64, t: f64) -> f64 {
    math::density_altitude(p, t)
}

#[pyfunction]
fn saturation_vapor_pressure(t: f64) -> f64 {
    math::saturation_vapor_pressure(t)
}

#[pyfunction]
fn vapor_pressure(t: f64, rh: f64) -> f64 {
    math::vapor_pressure(t, rh)
}

#[pyfunction]
fn mixing_ratio(p: f64, e: f64) -> f64 {
    math::mixing_ratio(p, e)
}

#[pyfunction]
fn dew_point(e: f64) -> f64 {
    math::dew_point(e)
}

#[pyfunction]
fn virtual_temperature(t: f64, w: f64) -> f64 {
    math::virtual_temperature(t, w)
}

#[pyfunction]
fn wind_loglaw(z: f64, z_ref: f64, u_ref: f64, z0: f64) -> f64 {
    math::wind_loglaw(z, z_ref, u_ref, z0)
}

#[pyfunction]
fn geometric_to_geopotential(h: f64) -> f64 {
    math::geometric_to_geopotential(h)
}

#[pyfunction]
fn geopotential_to_geometric(h: f64) -> f64 {
    math::geopotential_to_geometric(h)
}

#[pyfunction]
fn moist_air_density(p: f64, t: f64, rh: f64) -> f64 {
    math::moist_air_density(p, t, rh)
}

#[pyfunction]
fn moist_speed_of_sound(isa: &ISA, h: f64, rh: f64) -> f64 {
    let (t, p, _rho) = isa
        .core
        .atm_scalar(h)
        .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
    math::moist_speed_of_sound(t, rh, p)
}

// --- wind models ---

#[pyfunction]
fn wind_power_law(z: f64, z_ref: f64, u_ref: f64, alpha: f64) -> f64 {
    wind::wind_power_law(z, z_ref, u_ref, alpha)
}

#[pyfunction]
fn wind_loglaw_displaced(z: f64, z_ref: f64, u_ref: f64, z0: f64, d: f64) -> f64 {
    wind::wind_loglaw_displaced(z, z_ref, u_ref, z0, d)
}

#[pyfunction]
fn wind_linear_shear(z: f64, z0: f64, z1: f64, u0: f64, u1: f64) -> f64 {
    wind::wind_linear_shear(z, z0, z1, u0, u1)
}

#[pyfunction]
fn wind_ekman(z: f64, u0: f64, v0: f64, z_ek: f64, angle_max_deg: f64) -> (f64, f64) {
    wind::wind_ekman(z, u0, v0, z_ek, angle_max_deg)
}

#[pyfunction]
fn gust(u_mean: f64, g_factor: f64) -> f64 {
    wind::gust(u_mean, g_factor)
}

// --- thermodynamic extensions ---

#[pyfunction]
fn potential_temperature(t: f64, p: f64) -> f64 {
    thermo::potential_temperature(t, p)
}

#[pyfunction]
fn moist_lapse_rate(t: f64, p: f64, rh: f64) -> f64 {
    thermo::moist_lapse_rate(t, p, rh)
}

#[pyfunction]
fn wet_bulb_temperature(t: f64, rh: f64) -> f64 {
    thermo::wet_bulb_temperature(t, rh)
}

// --- flight-level conversions ---

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

// --- icing conditions ---

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

#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ISA>()?;

    m.add_function(wrap_pyfunction!(speed_of_sound, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mach, m)?)?;
    m.add_function(wrap_pyfunction!(pressure_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(density_altitude, m)?)?;

    m.add_function(wrap_pyfunction!(saturation_vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mixing_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(dew_point, m)?)?;
    m.add_function(wrap_pyfunction!(virtual_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(wind_loglaw, m)?)?;

    m.add_function(wrap_pyfunction!(geometric_to_geopotential, m)?)?;
    m.add_function(wrap_pyfunction!(geopotential_to_geometric, m)?)?;

    m.add_function(wrap_pyfunction!(moist_air_density, m)?)?;
    m.add_function(wrap_pyfunction!(moist_speed_of_sound, m)?)?;

    m.add_function(wrap_pyfunction!(wind_power_law, m)?)?;
    m.add_function(wrap_pyfunction!(wind_loglaw_displaced, m)?)?;
    m.add_function(wrap_pyfunction!(wind_linear_shear, m)?)?;
    m.add_function(wrap_pyfunction!(wind_ekman, m)?)?;
    m.add_function(wrap_pyfunction!(gust, m)?)?;

    m.add_function(wrap_pyfunction!(potential_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(moist_lapse_rate, m)?)?;
    m.add_function(wrap_pyfunction!(wet_bulb_temperature, m)?)?;

    m.add_function(wrap_pyfunction!(altitude_to_fl, m)?)?;
    m.add_function(wrap_pyfunction!(fl_to_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(geometric_to_fl, m)?)?;
    m.add_function(wrap_pyfunction!(indicated_altitude, m)?)?;

    m.add_function(wrap_pyfunction!(lwc, m)?)?;
    m.add_function(wrap_pyfunction!(supercooled_fraction, m)?)?;
    m.add_function(wrap_pyfunction!(icing_severity, m)?)?;
    m.add_function(wrap_pyfunction!(freezing_fraction, m)?)?;

    Ok(())
}
