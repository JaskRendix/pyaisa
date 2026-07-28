use pyaisa_core::math::{
    cas_to_eas, d, dynamic_pressure, dynamic_viscosity_sutherland, eas_to_tas, kinematic_viscosity,
    mach, mach_from_tas, prandtl_glauert, reynolds_number, speed_of_sound, stagnation_entropy,
    stagnation_pressure, stagnation_temperature, tas_to_eas,
};

const EPS: f64 = 1e-9;

//
// ─────────────────────────────────────────────────────────────
// ZERO INDICATOR
// ─────────────────────────────────────────────────────────────
//

#[test]
fn d_zero() {
    assert_eq!(d(0.0), 1.0);
}

#[test]
fn d_nonzero() {
    assert_eq!(d(1e-3), 0.0);
}

//
// ─────────────────────────────────────────────────────────────
// SPEED OF SOUND
// ─────────────────────────────────────────────────────────────
//

#[test]
fn speed_of_sound_basic() {
    let a = speed_of_sound(288.15);
    assert!(a > 300.0 && a < 350.0);
}

#[test]
fn speed_of_sound_monotonic() {
    assert!(speed_of_sound(300.0) > speed_of_sound(280.0));
}

//
// ─────────────────────────────────────────────────────────────
// DYNAMIC PRESSURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn dynamic_pressure_basic() {
    let q = dynamic_pressure(1.225, 50.0);
    assert!((q - 1531.25).abs() < 1e-2);
}

#[test]
fn dynamic_pressure_zero_velocity() {
    assert_eq!(dynamic_pressure(1.225, 0.0), 0.0);
}

//
// ─────────────────────────────────────────────────────────────
// MACH NUMBER
// ─────────────────────────────────────────────────────────────
//

#[test]
fn mach_basic() {
    let m = mach(340.0, 340.0);
    assert!((m - 1.0).abs() < EPS);
}

#[test]
fn mach_subsonic() {
    assert!(mach(200.0, 340.0) < 1.0);
}

#[test]
fn mach_supersonic() {
    assert!(mach(400.0, 340.0) > 1.0);
}

//
// ─────────────────────────────────────────────────────────────
// SUTHERLAND VISCOSITY
// ─────────────────────────────────────────────────────────────
//

#[test]
fn sutherland_basic() {
    let mu = dynamic_viscosity_sutherland(300.0);
    assert!(mu > 1e-5 && mu < 3e-5);
}

#[test]
fn sutherland_monotonic() {
    assert!(dynamic_viscosity_sutherland(350.0) > dynamic_viscosity_sutherland(250.0));
}

//
// ─────────────────────────────────────────────────────────────
// KINEMATIC VISCOSITY
// ─────────────────────────────────────────────────────────────
//

#[test]
fn kinematic_viscosity_basic() {
    let nu = kinematic_viscosity(1.8e-5, 1.225);
    assert!(nu > 0.0);
}

#[test]
fn kinematic_viscosity_zero_density() {
    let nu = kinematic_viscosity(1.8e-5, 0.0);
    assert!(nu.is_infinite());
}

//
// ─────────────────────────────────────────────────────────────
// REYNOLDS NUMBER
// ─────────────────────────────────────────────────────────────
//

#[test]
fn reynolds_basic() {
    let re = reynolds_number(1.225, 50.0, 1.0, 1.8e-5);
    assert!(re > 1e6);
}

#[test]
fn reynolds_zero_velocity() {
    assert_eq!(reynolds_number(1.225, 0.0, 1.0, 1.8e-5), 0.0);
}

//
// ─────────────────────────────────────────────────────────────
// STAGNATION TEMPERATURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn stagnation_temperature_basic() {
    let t0 = stagnation_temperature(288.15, 0.5);
    assert!(t0 > 288.15);
}

#[test]
fn stagnation_temperature_zero_mach() {
    assert_eq!(stagnation_temperature(288.15, 0.0), 288.15);
}

//
// ─────────────────────────────────────────────────────────────
// STAGNATION PRESSURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn stagnation_pressure_basic() {
    let p0 = stagnation_pressure(101325.0, 0.5);
    assert!(p0 > 101325.0);
}

#[test]
fn stagnation_pressure_zero_mach() {
    assert_eq!(stagnation_pressure(101325.0, 0.0), 101325.0);
}

//
// ─────────────────────────────────────────────────────────────
// STAGNATION ENTROPY
// ─────────────────────────────────────────────────────────────
//

#[test]
fn stagnation_entropy_basic() {
    let s = stagnation_entropy(288.15, 101325.0);
    assert!(s.is_finite());
}

#[test]
fn stagnation_entropy_monotonic() {
    let s1 = stagnation_entropy(280.0, 101325.0);
    let s2 = stagnation_entropy(300.0, 101325.0);
    assert!(s2 > s1);
}

//
// ─────────────────────────────────────────────────────────────
// PRANDTL–GLAUERT
// ─────────────────────────────────────────────────────────────
//

#[test]
fn prandtl_glauert_subsonic() {
    let pg = prandtl_glauert(0.5);
    assert!(pg > 1.0);
}

#[test]
fn prandtl_glauert_transonic_divergence() {
    let pg = prandtl_glauert(1.0);
    assert!(pg.is_infinite());
}

#[test]
fn prandtl_glauert_supersonic_invalid() {
    let pg = prandtl_glauert(1.2);
    assert!(pg.is_infinite());
}

//
// ─────────────────────────────────────────────────────────────
// EAS ↔ TAS
// ─────────────────────────────────────────────────────────────
//

#[test]
fn eas_to_tas_basic() {
    let tas = eas_to_tas(100.0, 0.8, 1.225);
    assert!(tas > 100.0);
}

#[test]
fn tas_to_eas_basic() {
    let eas = tas_to_eas(200.0, 0.8, 1.225);
    assert!(eas < 200.0);
}

#[test]
fn eas_tas_round_trip() {
    let eas = 150.0;
    let tas = eas_to_tas(eas, 0.8, 1.225);
    let eas2 = tas_to_eas(tas, 0.8, 1.225);
    assert!((eas2 - eas).abs() < EPS);
}

//
// ─────────────────────────────────────────────────────────────
// CAS → EAS
// ─────────────────────────────────────────────────────────────
//

#[test]
fn cas_to_eas_basic() {
    let eas = cas_to_eas(100.0, 101325.0, 1.225);
    assert!((eas - 101.08).abs() < 0.5);
}

#[test]
fn cas_to_eas_zero() {
    let eas = cas_to_eas(0.0, 101325.0, 1.225);
    assert_eq!(eas, 0.0);
}

//
// ─────────────────────────────────────────────────────────────
// MACH FROM TAS
// ─────────────────────────────────────────────────────────────
//

#[test]
fn mach_from_tas_basic() {
    let m = mach_from_tas(340.0, 340.0);
    assert!((m - 1.0).abs() < EPS);
}

#[test]
fn mach_from_tas_subsonic() {
    assert!(mach_from_tas(200.0, 340.0) < 1.0);
}

#[test]
fn mach_from_tas_supersonic() {
    assert!(mach_from_tas(400.0, 340.0) > 1.0);
}
