use pyaisa_core::core::IsaCore;
use pyaisa_core::flight::*;
use pyaisa_core::layers::*;
use pyaisa_core::thermo::*;

fn build_test_isa() -> IsaCore {
    let r = 287.05287;
    let g = 9.80665;

    let hl = vec![0.0, 11000.0, 20000.0];
    let al = vec![-0.0065, 0.0];

    let t0 = 288.15;
    let p0 = 101325.0;

    IsaCore::new(r, g, hl, al, t0, p0, 500)
}

#[test]
fn tlayer_linear() {
    let t = TLayer::new(0.0_f64, -0.0065_f64, 288.15_f64);
    let t1000 = t.eval(1000.0_f64);
    assert!((t1000 - (288.15_f64 - 6.5_f64)).abs() < 1e-6_f64);
}

#[test]
fn player_isothermal() {
    let p = PLayer::new(
        287.05287_f64,
        9.80665_f64,
        0.0_f64,
        0.0_f64,
        288.15_f64,
        101325.0_f64,
    );
    let p1000 = p.eval(1000.0_f64);

    let expected =
        101325.0_f64 * f64::exp(-9.80665_f64 * 1000.0_f64 / (287.05287_f64 * 288.15_f64));
    assert!((p1000 - expected).abs() < 1e-6_f64);
}

#[test]
fn player_gradient() {
    let p = PLayer::new(
        287.05287_f64,
        9.80665_f64,
        0.0_f64,
        -0.0065_f64,
        288.15_f64,
        101325.0_f64,
    );
    let p5000 = p.eval(5000.0_f64);

    let ratio: f64 = 1.0_f64 + (-0.0065_f64) * 5000.0_f64 / 288.15_f64;
    let expected = 101325.0_f64 * ratio.powf(-9.80665_f64 / (287.05287_f64 * -0.0065_f64));

    assert!((p5000 - expected).abs() < 1e-6_f64);
}

#[test]
fn isa_sea_level() {
    let isa = build_test_isa();
    let (t, p, rho) = isa.atm_scalar(0.0_f64).unwrap();

    assert!((t - 288.15_f64).abs() < 1e-6_f64);
    assert!((p - 101325.0_f64).abs() < 1e-6_f64);
    assert!((rho - 1.225_f64).abs() < 1e-3_f64);
}

#[test]
fn isa_tropopause_temp() {
    let isa = build_test_isa();
    let (t, _, _) = isa.atm_scalar(11000.0_f64).unwrap();
    assert!((t - 216.65_f64).abs() < 0.5_f64);
}

#[test]
fn isa_vector_consistency() {
    let isa = build_test_isa();
    let h: Vec<f64> = vec![0.0_f64, 5000.0_f64, 10000.0_f64];

    let (tv, pv, rv, err) = isa.atm_vec(&h);
    assert!(!err);

    for (i, &hi) in h.iter().enumerate() {
        let (ts, ps, rs) = isa.atm_scalar(hi).unwrap();
        assert!((tv[i] - ts).abs() < 1e-12_f64);
        assert!((pv[i] - ps).abs() < 1e-12_f64);
        assert!((rv[i] - rs).abs() < 1e-12_f64);
    }
}

#[test]
fn geopotential_round_trip() {
    let h: f64 = 8000.0_f64;
    let g = geometric_to_geopotential(h);
    let h2 = geopotential_to_geometric(g);

    assert!((h2 - h).abs() < 1e-6_f64);
}

#[test]
fn moist_air_density_lower_than_dry() {
    let isa = build_test_isa();
    let (t, p, rho_dry) = isa.atm_scalar(2000.0_f64).unwrap();

    let rho_moist = moist_air_density(p, t, 0.8_f64);

    assert!(rho_moist < rho_dry);
}

#[test]
fn isa_ratios_at_sea_level() {
    let isa = build_test_isa();

    assert!((isa.delta(0.0_f64).unwrap() - 1.0_f64).abs() < 1e-12_f64);
    assert!((isa.theta(0.0_f64).unwrap() - 1.0_f64).abs() < 1e-12_f64);
    assert!((isa.sigma(0.0_f64).unwrap() - 1.0_f64).abs() < 1e-12_f64);
}

#[test]
fn tropopause_detection() {
    let isa = build_test_isa();
    let tp = isa.tropopause().unwrap();
    assert!((tp - 11000.0_f64).abs() < 1e-6_f64);
}

#[test]
fn static_stability_positive() {
    let isa = build_test_isa();
    let n2 = isa.static_stability(5000.0_f64).unwrap();
    assert!(n2 > 0.0_f64);
}

#[test]
fn isa_deviation_zero_at_sea_level() {
    let isa = build_test_isa();
    let (dt, dp, drho) = isa.isa_deviation(0.0_f64).unwrap();

    assert!(dt.abs() < 1e-12_f64);
    assert!(dp.abs() < 1e-12_f64);
    assert!(drho.abs() < 1e-12_f64);
}
