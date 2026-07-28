use pyaisa_core::math::{
    cas_to_eas, cas_to_mach, cas_to_tas, d, dynamic_pressure, dynamic_viscosity_sutherland,
    eas_to_cas, eas_to_tas, kinematic_viscosity, mach, mach_from_tas, mach_moist, mach_to_cas,
    prandtl_glauert, reynolds_number, speed_of_sound, stagnation_entropy, stagnation_pressure,
    stagnation_temperature, tas_to_cas, tas_to_eas, tas_to_mach_moist,
};

const EPS: f64 = 1e-9;

#[test]
fn d_zero() {
    assert_eq!(d(0.0), 1.0);
}

#[test]
fn d_nonzero() {
    assert_eq!(d(1e-3), 0.0);
}

#[test]
fn speed_of_sound_basic() {
    let a = speed_of_sound(288.15);
    assert!(a > 300.0 && a < 350.0);
}

#[test]
fn speed_of_sound_monotonic() {
    assert!(speed_of_sound(300.0) > speed_of_sound(280.0));
}

#[test]
fn dynamic_pressure_basic() {
    let q = dynamic_pressure(1.225, 50.0);
    assert!((q - 1531.25).abs() < 1e-2);
}

#[test]
fn dynamic_pressure_zero_velocity() {
    assert_eq!(dynamic_pressure(1.225, 0.0), 0.0);
}

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

#[test]
fn sutherland_basic() {
    let mu = dynamic_viscosity_sutherland(300.0);
    assert!(mu > 1e-5 && mu < 3e-5);
}

#[test]
fn sutherland_monotonic() {
    assert!(dynamic_viscosity_sutherland(350.0) > dynamic_viscosity_sutherland(250.0));
}

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

#[test]
fn reynolds_basic() {
    let re = reynolds_number(1.225, 50.0, 1.0, 1.8e-5);
    assert!(re > 1e6);
}

#[test]
fn reynolds_zero_velocity() {
    assert_eq!(reynolds_number(1.225, 0.0, 1.0, 1.8e-5), 0.0);
}

#[test]
fn stagnation_temperature_basic() {
    let t0 = stagnation_temperature(288.15, 0.5);
    assert!(t0 > 288.15);
}

#[test]
fn stagnation_temperature_zero_mach() {
    assert_eq!(stagnation_temperature(288.15, 0.0), 288.15);
}

#[test]
fn stagnation_pressure_basic() {
    let p0 = stagnation_pressure(101325.0, 0.5);
    assert!(p0 > 101325.0);
}

#[test]
fn stagnation_pressure_zero_mach() {
    assert_eq!(stagnation_pressure(101325.0, 0.0), 101325.0);
}

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

#[test]
fn eas_to_cas_basic() {
    let cas = eas_to_cas(100.0, 101325.0, 1.225);
    assert!(cas < 100.0);
    assert!(cas > 95.0);
}

#[test]
fn eas_to_cas_round_trip() {
    let cas = 150.0;
    let eas = cas_to_eas(cas, 101325.0, 1.225);
    let cas2 = eas_to_cas(eas, 101325.0, 1.225);
    assert!((cas2 - cas).abs() < 0.5);
}

#[test]
fn cas_to_mach_basic() {
    let mach = cas_to_mach(100.0, 101325.0, 1.225);
    assert!(mach > 0.0);
}

#[test]
fn mach_to_cas_basic() {
    let cas = mach_to_cas(0.3, 101325.0, 1.225);
    assert!(cas > 0.0);
}

#[test]
fn cas_mach_round_trip() {
    let cas = 120.0;
    let mach = cas_to_mach(cas, 101325.0, 1.225);
    let cas2 = mach_to_cas(mach, 101325.0, 1.225);
    assert!((cas2 - cas).abs() < 0.5);
}

#[test]
fn tas_to_cas_basic() {
    let cas = tas_to_cas(250.0, 288.15, 90000.0, 1.0);
    assert!(cas > 0.0);
}

#[test]
fn cas_to_tas_basic() {
    let tas = cas_to_tas(150.0, 288.15, 90000.0, 1.0);
    assert!(tas.is_finite());
}

#[test]
fn tas_cas_round_trip() {
    let tas = 220.0;
    let cas = tas_to_cas(tas, 288.15, 90000.0, 1.0);
    let tas2 = cas_to_tas(cas, 288.15, 90000.0, 1.0);
    assert!((tas2 - tas).abs() < 1.0);
}

#[test]
fn mach_moist_basic() {
    let m = mach_moist(300.0, 300.0, 0.5, 90000.0);
    assert!(m > 0.0);
}

#[test]
fn tas_to_mach_moist_basic() {
    let m = tas_to_mach_moist(250.0, 300.0, 0.5, 90000.0);
    assert!(m > 0.0);
}

#[test]
fn mach_moist_vs_dry() {
    let a_dry = speed_of_sound(300.0);
    let a_moist = pyaisa_core::thermo::moist_speed_of_sound(300.0, 1.0, 90000.0);

    let m_dry = 250.0 / a_dry;
    let m_moist = 250.0 / a_moist;

    // moist air has lower speed of sound → higher Mach
    assert!(m_moist < m_dry);
}
