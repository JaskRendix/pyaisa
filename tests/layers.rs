use pyaisa_core::layers::{PLayer, TLayer};

const EPS: f64 = 1e-9;

//
// ─────────────────────────────────────────────────────────────
// TLayer TESTS
// ─────────────────────────────────────────────────────────────
//

#[test]
fn tlayer_constant_lapse_rate() {
    let t = TLayer::new(0.0, -0.0065, 288.15);
    let t1000 = t.eval(1000.0);
    assert!((t1000 - (288.15 - 6.5)).abs() < EPS);
}

#[test]
fn tlayer_zero_lapse_rate_isothermal() {
    let t = TLayer::new(0.0, 0.0, 250.0);
    assert!((t.eval(5000.0) - 250.0).abs() < EPS);
}

#[test]
fn tlayer_negative_altitude() {
    let t = TLayer::new(0.0, -0.0065, 288.15);
    let t_neg = t.eval(-500.0);
    assert!(t_neg > 288.15);
}

#[test]
fn tlayer_monotonicity() {
    let t = TLayer::new(0.0, -0.0065, 288.15);
    assert!(t.eval(2000.0) < t.eval(1000.0));
}

#[test]
fn tlayer_lapse_rate_correct() {
    let t = TLayer::new(0.0, -0.0065, 288.15);
    assert!((t.lapse_rate() + 0.0065).abs() < EPS);
}

//
// ─────────────────────────────────────────────────────────────
// PLayer TESTS — ISOTHERMAL
// ─────────────────────────────────────────────────────────────
//

#[test]
fn player_isothermal_basic() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let p1000 = p.eval(1000.0);

    let expected = 101325.0 * f64::exp(-9.81 * 1000.0 / (287.0 * 288.15));
    assert!((p1000 - expected).abs() < EPS);
}

#[test]
fn player_isothermal_monotonic() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    assert!(p.eval(2000.0) < p.eval(1000.0));
}

#[test]
fn player_isothermal_inverse() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let p1500 = p.eval(1500.0);
    let h2 = p.altitude_from_pressure(p1500);
    assert!((h2 - 1500.0).abs() < 1e-6);
}

//
// ─────────────────────────────────────────────────────────────
// PLayer TESTS — GRADIENT
// ─────────────────────────────────────────────────────────────
//

#[test]
fn player_gradient_basic() {
    let p = PLayer::new(287.0, 9.81, 0.0, -0.0065, 288.15, 101325.0);
    let p5000 = p.eval(5000.0);

    let ratio: f64 = 1.0 + (-0.0065_f64) * 5000.0_f64 / 288.15_f64;
    let expected = 101325.0 * ratio.powf(-9.81 / (287.0 * -0.0065));

    assert!((p5000 - expected).abs() < EPS);
}

#[test]
fn player_gradient_monotonic() {
    let p = PLayer::new(287.0, 9.81, 0.0, -0.0065, 288.15, 101325.0);
    assert!(p.eval(2000.0) < p.eval(1000.0));
}

#[test]
fn player_gradient_inverse() {
    let p = PLayer::new(287.0, 9.81, 0.0, -0.0065, 288.15, 101325.0);
    let p3000 = p.eval(3000.0);
    let h2 = p.altitude_from_pressure(p3000);
    assert!((h2 - 3000.0).abs() < 1e-6);
}

//
// ─────────────────────────────────────────────────────────────
// PLayer TESTS — EDGE CASES
// ─────────────────────────────────────────────────────────────
//

#[test]
fn player_extreme_low_pressure() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let pa = p.eval(50000.0);
    assert!((pa - 265.0).abs() < 50.0);
}

#[test]
fn player_extreme_high_pressure() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let pa = p.eval(-5000.0);
    assert!(pa > 101325.0);
}

#[test]
fn player_pressure_gradient_correct_sign() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let rho = 1.225;
    let dpdh = p.pressure_gradient(rho);
    assert!(dpdh < 0.0);
}

#[test]
fn player_pressure_gradient_magnitude() {
    let p = PLayer::new(287.0, 9.81, 0.0, 0.0, 288.15, 101325.0);
    let rho = 1.225;
    let dpdh = p.pressure_gradient(rho);
    assert!((dpdh + 1.225 * 9.81).abs() < EPS);
}
