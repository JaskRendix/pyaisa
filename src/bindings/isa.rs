use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};

use crate::core::IsaCore;

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

#[pyclass]
pub struct Isa {
    core: IsaCore,
    params: Py<PyDict>,
}

// Rust-only accessor
impl Isa {
    pub fn core(&self) -> &IsaCore {
        &self.core
    }
}

#[pymethods]
impl Isa {
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

        // Optional custom layers
        let layers: Option<Bound<'_, PyDict>> = kwargs.and_then(|k| {
            k.get_item("layers")
                .ok()
                .flatten()
                .and_then(|obj| obj.downcast::<PyDict>().ok().cloned())
        });

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
            // ICAO Isa default layers
            (
                vec![0.0, 11000.0, 20000.0, 32000.0],
                vec![-0.0065, 0.0, 0.001],
            )
        };

        let core = IsaCore::new(r, g, hl, al, t0, p0, psize);

        let params = kwargs
            .map(|k| Py::from(k.to_owned()))
            .unwrap_or_else(|| PyDict::new_bound(py).unbind());

        Ok(Isa { core, params })
    }

    #[getter]
    fn params(&self, py: Python) -> Py<PyDict> {
        self.params.clone_ref(py)
    }

    /// Forward atmosphere lookup
    fn atm(&self, py: Python, h: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        // Scalar
        if let Ok(val) = h.extract::<f64>() {
            if let Some((t, p, rho)) = self.core.atm_scalar(val) {
                return Ok(PyTuple::new_bound(py, [t, p, rho]).unbind().into());
            }
            let warnings = py.import_bound("warnings")?;
            warnings.call_method1("warn", ("Altitude value outside range",))?;
            return Ok(PyTuple::new_bound(py, [f64::NAN, f64::NAN, f64::NAN])
                .unbind()
                .into());
        }

        // Vector
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

    /// Deviated atmosphere lookup
    fn atm_deviation(
        &self,
        py: Python,
        h: &Bound<'_, PyAny>,
        d_t: f64,
        dp: f64,
        drho: f64,
    ) -> PyResult<PyObject> {
        // Scalar
        if let Ok(val) = h.extract::<f64>() {
            if let Some((t, p, rho)) = self.core.atm_deviation_scalar(val, d_t, dp, drho) {
                return Ok(PyTuple::new_bound(py, [t, p, rho]).unbind().into());
            }
            let warnings = py.import_bound("warnings")?;
            warnings.call_method1("warn", ("Altitude value outside range",))?;
            return Ok(PyTuple::new_bound(py, [f64::NAN, f64::NAN, f64::NAN])
                .unbind()
                .into());
        }

        // Vector
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

    fn layer_at(&self, _py: Python, h: f64) -> Option<usize> {
        self.core.layer_at(h)
    }

    /// Isa pressure ratio δ = p / p0
    fn delta(&self, _py: Python, h: f64) -> Option<f64> {
        self.core.delta(h)
    }

    /// Isa temperature ratio θ = T / T0
    fn theta(&self, _py: Python, h: f64) -> Option<f64> {
        self.core.theta(h)
    }

    /// Isa density ratio σ = ρ / ρ0
    fn sigma(&self, _py: Python, h: f64) -> Option<f64> {
        self.core.sigma(h)
    }

    /// Tropopause altitude (geometric)
    fn tropopause(&self, _py: Python) -> Option<f64> {
        self.core.tropopause()
    }

    /// Static stability (Brunt–Väisälä frequency squared)
    fn static_stability(&self, _py: Python, h: f64) -> Option<f64> {
        self.core.static_stability(h)
    }

    /// Isa deviation ΔT, Δp, Δρ
    fn isa_deviation(&self, py: Python, h: f64) -> PyResult<PyObject> {
        if let Some((d_t, d_p, d_rho)) = self.core.isa_deviation(h) {
            Ok(PyTuple::new_bound(py, [d_t, d_p, d_rho]).unbind().into())
        } else {
            let warnings = py.import_bound("warnings")?;
            warnings.call_method1("warn", ("Altitude value outside range",))?;
            Ok(PyTuple::new_bound(py, [f64::NAN, f64::NAN, f64::NAN])
                .unbind()
                .into())
        }
    }
}
