use crate::math::d;

/// Temperature layer (linear lapse rate)
pub struct TLayer {
    hs: f64, // layer base altitude
    a: f64,  // lapse rate
    ts: f64, // temperature at layer base
}

impl TLayer {
    #[inline]
    pub fn new(hs: f64, a: f64, ts: f64) -> Self {
        Self { hs, a, ts }
    }

    /// Evaluate temperature at altitude h
    #[inline]
    pub fn eval(&self, h: f64) -> f64 {
        self.ts + self.a * (h - self.hs)
    }
}

/// Pressure layer (isothermal or gradient)
pub struct PLayer {
    r: f64,     // specific gas constant
    g: f64,     // gravity
    hs: f64,    // layer base altitude
    a: f64,     // lapse rate
    ts: f64,    // temperature at layer base
    ps: f64,    // pressure at layer base
    grad: bool, // true = isothermal layer (a ≈ 0)
}

impl PLayer {
    #[inline]
    pub fn new(r: f64, g: f64, hs: f64, a: f64, ts: f64, ps: f64) -> Self {
        // isothermal layer when lapse rate is effectively zero
        let grad = d(a) == 1.0;

        Self {
            r,
            g,
            hs,
            a,
            ts,
            ps,
            grad,
        }
    }

    /// Evaluate pressure at altitude h
    #[inline]
    pub fn eval(&self, h: f64) -> f64 {
        let dh = h - self.hs;

        if self.grad {
            // isothermal layer
            self.ps * f64::exp(-self.g * dh / (self.r * self.ts))
        } else {
            // gradient layer
            let ratio = 1.0 + self.a * dh / self.ts;
            self.ps * ratio.powf(-self.g / (self.r * self.a))
        }
    }
}
