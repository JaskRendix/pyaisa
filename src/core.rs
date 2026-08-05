use crate::flight::geopotential_to_geometric;
use crate::layers::{PLayer, TLayer};
use crate::thermo::moist_air_density;
use rayon::prelude::*;

/// Core Isa engine (Rust side)
pub struct IsaCore {
    r: f64,
    _g: f64, // unused → prefix with underscore
    hl: Vec<f64>,
    _al: Vec<f64>, // unused
    _t0: f64,      // unused
    _p0: f64,      // unused
    layers: usize,
    parallel_size: usize,
    tl: Vec<TLayer>,
    pl: Vec<PLayer>,
}

impl IsaCore {
    /// Build Isa layer structure
    pub fn new(
        r: f64,
        g: f64,
        hl: Vec<f64>,
        al: Vec<f64>,
        t0: f64,
        p0: f64,
        parallel_size: usize,
    ) -> Self {
        assert_eq!(
            hl.len(),
            al.len() + 1,
            "hl must have one more element than al"
        );

        let layers = al.len();

        let mut tl = Vec::with_capacity(layers);
        let mut pl = Vec::with_capacity(layers);

        tl.push(TLayer::new(hl[0], al[0], t0));
        pl.push(PLayer::new(r, g, hl[0], al[0], t0, p0));

        for i in 1..layers {
            let ts = tl[i - 1].eval(hl[i]);
            let ps = pl[i - 1].eval(hl[i]);

            tl.push(TLayer::new(hl[i], al[i], ts));
            pl.push(PLayer::new(r, g, hl[i], al[i], ts, ps));
        }

        Self {
            r,
            _g: g,
            hl,
            _al: al,
            _t0: t0,
            _p0: p0,
            layers,
            parallel_size,
            tl,
            pl,
        }
    }

    #[inline]
    fn rho(&self, t: f64, p: f64) -> f64 {
        p / (self.r * t)
    }

    /// Optimized layer selector using binary search
    #[inline]
    fn select(&self, h: f64) -> usize {
        let idx = self.hl.partition_point(|&x| x <= h);
        idx.saturating_sub(1).min(self.layers - 1)
    }

    pub fn layer_at(&self, h: f64) -> Option<usize> {
        if h < self.hl[0] || h > self.hl[self.layers] {
            return None;
        }
        Some(self.select(h))
    }

    pub fn atm_scalar(&self, h: f64) -> Option<(f64, f64, f64)> {
        if h < self.hl[0] || h > self.hl[self.layers] {
            return None;
        }

        let l = self.select(h);
        let t = self.tl[l].eval(h);
        let p = self.pl[l].eval(h);
        let rho = self.rho(t, p);

        Some((t, p, rho))
    }

    pub fn atm_vec(&self, h: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, bool) {
        let parallel = h.len() > self.parallel_size;

        if parallel {
            let results: Vec<(f64, f64, f64, bool)> = h
                .par_iter()
                .map(|&h_i| match self.atm_scalar(h_i) {
                    Some((t, p, rho)) => (t, p, rho, false),
                    None => (f64::NAN, f64::NAN, f64::NAN, true),
                })
                .collect();

            let mut t = Vec::with_capacity(h.len());
            let mut p = Vec::with_capacity(h.len());
            let mut rho = Vec::with_capacity(h.len());
            let mut error = false;

            for (ti, pi, rhoi, err) in results {
                t.push(ti);
                p.push(pi);
                rho.push(rhoi);
                error |= err;
            }

            (t, p, rho, error)
        } else {
            let mut t = Vec::with_capacity(h.len());
            let mut p = Vec::with_capacity(h.len());
            let mut rho = Vec::with_capacity(h.len());
            let mut error = false;

            for &h_i in h.iter() {
                match self.atm_scalar(h_i) {
                    Some((ti, pi, rhoi)) => {
                        t.push(ti);
                        p.push(pi);
                        rho.push(rhoi);
                    }
                    None => {
                        t.push(f64::NAN);
                        p.push(f64::NAN);
                        rho.push(f64::NAN);
                        error = true;
                    }
                }
            }

            (t, p, rho, error)
        }
    }

    #[allow(dead_code)]
    pub fn atm_geopotential_scalar(&self, h_geo: f64) -> Option<(f64, f64, f64)> {
        let h = geopotential_to_geometric(h_geo);
        self.atm_scalar(h)
    }

    #[allow(dead_code)]
    pub fn atm_geopotential_vec(&self, h_geo: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, bool) {
        let h: Vec<f64> = h_geo
            .iter()
            .map(|&h_i| geopotential_to_geometric(h_i))
            .collect();
        self.atm_vec(&h)
    }

    #[allow(dead_code)]
    pub fn atm_moist_scalar(&self, h: f64, rh: f64) -> Option<(f64, f64, f64)> {
        let (t, p, _) = self.atm_scalar(h)?;
        let rho_m = moist_air_density(p, t, rh);
        Some((t, p, rho_m))
    }

    #[allow(dead_code)]
    pub fn atm_moist_vec(&self, h: &[f64], rh: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>, bool) {
        let (t, p, _, error) = self.atm_vec(h);

        let rho_m: Vec<f64> = t
            .iter()
            .zip(p.iter())
            .map(|(&t_i, &p_i)| moist_air_density(p_i, t_i, rh))
            .collect();

        (t, p, rho_m, error)
    }

    pub fn atm_deviation_scalar(
        &self,
        h: f64,
        d_t: f64,
        dp: f64,
        drho: f64,
    ) -> Option<(f64, f64, f64)> {
        let (mut t, mut p, mut rho) = self.atm_scalar(h)?;
        t += d_t;
        p += dp;
        rho += drho;
        Some((t, p, rho))
    }

    pub fn atm_deviation_vec(
        &self,
        h: &[f64],
        d_t: f64,
        dp: f64,
        drho: f64,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>, bool) {
        let (mut t, mut p, mut rho, err) = self.atm_vec(h);

        for i in 0..t.len() {
            t[i] += d_t;
            p[i] += dp;
            rho[i] += drho;
        }

        (t, p, rho, err)
    }

    /// Isa pressure ratio δ = p / p0
    pub fn delta(&self, h: f64) -> Option<f64> {
        let (_, p, _) = self.atm_scalar(h)?;
        Some(p / self._p0)
    }

    /// Isa temperature ratio θ = T / T0
    pub fn theta(&self, h: f64) -> Option<f64> {
        let (t, _, _) = self.atm_scalar(h)?;
        Some(t / self._t0)
    }

    /// Isa density ratio σ = ρ / ρ0
    pub fn sigma(&self, h: f64) -> Option<f64> {
        let (_, _, rho) = self.atm_scalar(h)?;
        let rho0 = self._p0 / (self.r * self._t0);
        Some(rho / rho0)
    }

    /// Tropopause detection: first layer with zero lapse rate
    pub fn tropopause(&self) -> Option<f64> {
        for (i, layer) in self.tl.iter().enumerate() {
            if layer.lapse_rate() == 0.0 {
                return Some(self.hl[i]);
            }
        }
        None
    }

    /// Static stability (Brunt–Väisälä frequency squared)
    /// N² = (g / T) * (Γ_d - Γ)
    pub fn static_stability(&self, h: f64) -> Option<f64> {
        let (t, _, _) = self.atm_scalar(h)?;
        let idx = self.select(h);
        let lapse = self.tl[idx].lapse_rate(); // Γ
        let gamma_d = 0.00980665; // dry lapse rate [K/m]
        let g = self._g;
        Some((g / t) * (gamma_d - lapse))
    }

    /// Isa deviation reporting: ΔT, Δp, Δρ from standard Isa
    pub fn isa_deviation(&self, h: f64) -> Option<(f64, f64, f64)> {
        let (t, p, rho) = self.atm_scalar(h)?;
        let t_std = self.tl[0].eval(h);
        let p_std = self.pl[0].eval(h);
        let rho_std = self.rho(t_std, p_std);
        Some((t - t_std, p - p_std, rho - rho_std))
    }
}
