use pyaisa_core::wind::{
    gust, wind_ekman, wind_linear_shear, wind_loglaw, wind_loglaw_displaced, wind_power_law,
};

const EPS: f64 = 1e-9;

#[test]
fn power_law_basic() {
    let u = wind_power_law(20.0, 10.0, 5.0, 0.2);
    let expected = 5.0 * (20.0_f64 / 10.0_f64).powf(0.2);
    assert!((u - expected).abs() < EPS);
}

#[test]
fn power_law_zero_height() {
    assert_eq!(wind_power_law(0.0, 10.0, 5.0, 0.2), 0.0);
    assert_eq!(wind_power_law(20.0, 0.0, 5.0, 0.2), 0.0);
}

#[test]
fn power_law_monotonic() {
    let u1 = wind_power_law(10.0, 10.0, 5.0, 0.2);
    let u2 = wind_power_law(20.0, 10.0, 5.0, 0.2);
    assert!(u2 > u1);
}

#[test]
fn loglaw_basic() {
    let u = wind_loglaw(20.0, 10.0, 5.0, 0.1);
    let expected = 5.0 * (20.0_f64 / 0.1_f64).ln() / (10.0_f64 / 0.1_f64).ln();
    assert!((u - expected).abs() < EPS);
}

#[test]
fn loglaw_invalid_inputs() {
    assert_eq!(wind_loglaw(0.05, 10.0, 5.0, 0.1), 0.0);
    assert_eq!(wind_loglaw(20.0, 0.05, 5.0, 0.1), 0.0);
    assert_eq!(wind_loglaw(20.0, 10.0, 5.0, -1.0), 0.0);
}

#[test]
fn loglaw_monotonic() {
    let u1 = wind_loglaw(5.0, 10.0, 5.0, 0.1);
    let u2 = wind_loglaw(20.0, 10.0, 5.0, 0.1);
    assert!(u2 > u1);
}

#[test]
fn loglaw_displaced_basic() {
    let u = wind_loglaw_displaced(20.0, 10.0, 5.0, 0.1, 2.0);
    let num = ((20.0_f64 - 2.0_f64) / 0.1_f64).ln();
    let den = ((10.0_f64 - 2.0_f64) / 0.1_f64).ln();
    let expected = 5.0 * num / den;
    assert!((u - expected).abs() < EPS);
}

#[test]
fn loglaw_displaced_invalid() {
    assert_eq!(wind_loglaw_displaced(2.0, 10.0, 5.0, 0.1, 2.0), 0.0);
    assert_eq!(wind_loglaw_displaced(20.0, 2.0, 5.0, 0.1, 2.0), 0.0);
}

#[test]
fn loglaw_displaced_monotonic() {
    let u1 = wind_loglaw_displaced(5.0, 10.0, 5.0, 0.1, 2.0);
    let u2 = wind_loglaw_displaced(20.0, 10.0, 5.0, 0.1, 2.0);
    assert!(u2 > u1);
}

#[test]
fn linear_shear_basic() {
    let u = wind_linear_shear(5.0, 0.0, 10.0, 2.0, 8.0);
    let expected = 2.0 + (5.0 / 10.0) * (8.0 - 2.0);
    assert!((u - expected).abs() < EPS);
}

#[test]
fn linear_shear_below_range() {
    assert_eq!(wind_linear_shear(-1.0, 0.0, 10.0, 2.0, 8.0), 2.0);
}

#[test]
fn linear_shear_above_range() {
    assert_eq!(wind_linear_shear(20.0, 0.0, 10.0, 2.0, 8.0), 8.0);
}

#[test]
fn linear_shear_monotonic() {
    let u1 = wind_linear_shear(2.0, 0.0, 10.0, 2.0, 8.0);
    let u2 = wind_linear_shear(8.0, 0.0, 10.0, 2.0, 8.0);
    assert!(u2 > u1);
}

#[test]
fn ekman_zero_height() {
    let (u, v) = wind_ekman(0.0, 5.0, 0.0, 100.0, 45.0);
    assert!((u - 5.0).abs() < EPS);
    assert!((v - 0.0).abs() < EPS);
}

#[test]
fn ekman_rotation_half_height() {
    let (u, v) = wind_ekman(50.0, 5.0, 0.0, 100.0, 90.0);
    let speed = (u * u + v * v).sqrt();
    assert!((speed - 5.0).abs() < EPS);
}

#[test]
fn ekman_full_rotation() {
    let (u, v) = wind_ekman(200.0, 5.0, 0.0, 100.0, 90.0);
    let dir = v.atan2(u);
    let expected_dir = 90f64.to_radians();
    assert!((dir - expected_dir).abs() < 0.1);
}

#[test]
fn gust_basic() {
    assert_eq!(gust(10.0, 0.2), 12.0);
}

#[test]
fn gust_zero_factor() {
    assert_eq!(gust(10.0, 0.0), 10.0);
}

#[test]
fn gust_negative_factor() {
    assert_eq!(gust(10.0, -0.5), 5.0);
}

#[test]
fn gust_monotonic() {
    assert!(gust(10.0, 0.3) > gust(10.0, 0.1));
}
