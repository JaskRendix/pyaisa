use crate::layers::{PLayer, TLayer};
use crate::math::{geopotential_to_geometric, moist_air_density};
use rayon::prelude::*;

/// Core ISA engine (Rust side)
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
    /// Build ISA layer structure
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
}
