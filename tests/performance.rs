use pyaisa_core::performance::*;

#[test]
fn test_drag_polar_nominal() {
    let cd = drag_polar(0.02, 0.04, 0.5);
    assert!(cd > 0.02);
}

#[test]
fn test_drag_polar_zero_lift() {
    let cd = drag_polar(0.02, 0.04, 0.0);
    assert!((cd - 0.02).abs() < 1e-12);
}

#[test]
fn test_drag_polar_negative_lift() {
    let cd = drag_polar(0.02, 0.04, -0.5);
    assert!(cd > 0.02);
}

#[test]
fn test_drag_polar_large_lift() {
    let cd = drag_polar(0.02, 0.04, 2.0);
    assert!(cd > 0.02);
}

#[test]
fn test_drag_polar_zero_k() {
    let cd = drag_polar(0.02, 0.0, 1.0);
    assert!((cd - 0.02).abs() < 1e-12);
}

#[test]
fn test_thrust_lapse_sea_level() {
    let t = thrust_lapse(10000.0, 288.15, 101325.0);
    assert!(t > 9000.0 && t < 11000.0);
}

#[test]
fn test_thrust_lapse_high_altitude() {
    let t = thrust_lapse(10000.0, 223.15, 25000.0);
    assert!(t < 5000.0);
}

#[test]
fn test_thrust_lapse_zero_thrust() {
    let t = thrust_lapse(0.0, 288.15, 101325.0);
    assert_eq!(t, 0.0);
}

#[test]
fn test_drag_force_nominal() {
    let d = drag_force(500.0, 20.0, 0.02);
    assert!((d - 200.0).abs() < 1e-12);
}

#[test]
fn test_drag_force_zero_q() {
    let d = drag_force(0.0, 20.0, 0.02);
    assert_eq!(d, 0.0);
}

#[test]
fn test_drag_force_zero_area() {
    let d = drag_force(500.0, 0.0, 0.02);
    assert_eq!(d, 0.0);
}

#[test]
fn test_drag_force_negative_cd() {
    let d = drag_force(500.0, 20.0, -0.02);
    assert!(d < 0.0);
}

#[test]
fn test_climb_rate_positive() {
    let rc = climb_rate(5000.0, 3000.0, 100.0, 60000.0);
    assert!(rc > 0.0);
}

#[test]
fn test_climb_rate_zero() {
    let rc = climb_rate(3000.0, 3000.0, 100.0, 60000.0);
    assert!((rc - 0.0).abs() < 1e-12);
}

#[test]
fn test_climb_rate_negative() {
    let rc = climb_rate(2000.0, 3000.0, 100.0, 60000.0);
    assert!(rc < 0.0);
}

#[test]
fn test_climb_rate_zero_velocity() {
    let rc = climb_rate(5000.0, 3000.0, 0.0, 60000.0);
    assert_eq!(rc, 0.0);
}

#[test]
fn test_climb_rate_negative_weight() {
    let rc = climb_rate(5000.0, 3000.0, 100.0, -60000.0);
    assert!(rc < 0.0);
}

#[test]
fn test_service_ceiling_below() {
    assert!(service_ceiling(0.3));
}

#[test]
fn test_service_ceiling_exact() {
    assert!(service_ceiling(0.5));
}

#[test]
fn test_service_ceiling_above() {
    assert!(!service_ceiling(0.7));
}

#[test]
fn test_service_ceiling_negative() {
    assert!(service_ceiling(-0.2));
}
