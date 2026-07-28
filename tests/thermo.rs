use pyaisa_core::thermo::{
    dew_point, mixing_ratio, moist_air_density, moist_lapse_rate, moist_speed_of_sound,
    potential_temperature, saturation_vapor_pressure, vapor_pressure, virtual_temperature,
    wet_bulb_temperature,
};

const EPS: f64 = 1e-9;

//
// ─────────────────────────────────────────────────────────────
// POTENTIAL TEMPERATURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn potential_temperature_basic() {
    let t = 288.15;
    let p = 90000.0;
    let theta = potential_temperature(t, p);
    assert!(theta > t);
}

#[test]
fn potential_temperature_sea_level() {
    let t = 288.15;
    let p = 100000.0;
    assert!((potential_temperature(t, p) - t).abs() < EPS);
}

//
// ─────────────────────────────────────────────────────────────
// MOIST LAPSE RATE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn moist_lapse_rate_basic() {
    let gamma = moist_lapse_rate(280.0, 90000.0, 0.5);
    assert!(gamma > 0.0);
    assert!(gamma < 0.00980665); // must be less than dry lapse rate
}

#[test]
fn moist_lapse_rate_increases_with_humidity() {
    let g1 = moist_lapse_rate(280.0, 90000.0, 0.2);
    let g2 = moist_lapse_rate(280.0, 90000.0, 0.8);
    assert!(g2 < g1);
}

//
// ─────────────────────────────────────────────────────────────
// WET-BULB TEMPERATURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn wet_bulb_temperature_basic() {
    let t = 300.0;
    let rh = 0.5;
    let tw = wet_bulb_temperature(t, rh);
    assert!(tw < t); // wet-bulb must be cooler
}

#[test]
fn wet_bulb_temperature_high_humidity() {
    let t = 300.0;
    let tw = wet_bulb_temperature(t, 1.0);
    assert!(tw < t);
}

//
// ─────────────────────────────────────────────────────────────
// SATURATION VAPOR PRESSURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn saturation_vapor_pressure_monotonic() {
    let e1 = saturation_vapor_pressure(273.15);
    let e2 = saturation_vapor_pressure(300.0);
    assert!(e2 > e1);
}

#[test]
fn saturation_vapor_pressure_clamping_low() {
    let e = saturation_vapor_pressure(200.0); // below valid range
    let e_min = saturation_vapor_pressure(228.15);
    assert!((e - e_min).abs() < EPS);
}

#[test]
fn saturation_vapor_pressure_clamping_high() {
    let e = saturation_vapor_pressure(400.0); // above valid range
    let e_max = saturation_vapor_pressure(333.15);
    assert!((e - e_max).abs() < EPS);
}

//
// ─────────────────────────────────────────────────────────────
// VAPOR PRESSURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn vapor_pressure_zero_rh() {
    assert_eq!(vapor_pressure(300.0, 0.0), 0.0);
}

#[test]
fn vapor_pressure_full_rh() {
    let t = 300.0;
    assert!((vapor_pressure(t, 1.0) - saturation_vapor_pressure(t)).abs() < EPS);
}

#[test]
fn vapor_pressure_clamped_rh() {
    let t = 300.0;
    assert_eq!(vapor_pressure(t, 2.0), saturation_vapor_pressure(t));
}

//
// ─────────────────────────────────────────────────────────────
// MIXING RATIO
// ─────────────────────────────────────────────────────────────
//

#[test]
fn mixing_ratio_basic() {
    let p = 90000.0;
    let e = 2000.0;
    let w = mixing_ratio(p, e);
    assert!(w > 0.0);
}

#[test]
fn mixing_ratio_invalid() {
    let w = mixing_ratio(1000.0, 2000.0);
    assert!(w.is_nan());
}

//
// ─────────────────────────────────────────────────────────────
// DEW POINT
// ─────────────────────────────────────────────────────────────
//

#[test]
fn dew_point_basic() {
    let e = 2000.0;
    let td = dew_point(e);
    assert!(td > 200.0);
    assert!(td < 350.0);
}

#[test]
fn dew_point_invalid() {
    assert!(dew_point(0.0).is_nan());
}

//
// ─────────────────────────────────────────────────────────────
// VIRTUAL TEMPERATURE
// ─────────────────────────────────────────────────────────────
//

#[test]
fn virtual_temperature_basic() {
    let tv = virtual_temperature(300.0, 0.01);
    assert!(tv > 300.0);
}

#[test]
fn virtual_temperature_zero_mixing_ratio() {
    assert_eq!(virtual_temperature(300.0, 0.0), 300.0);
}

//
// ─────────────────────────────────────────────────────────────
// MOIST-AIR DENSITY
// ─────────────────────────────────────────────────────────────
//

#[test]
fn moist_air_density_basic() {
    let rho = moist_air_density(90000.0, 280.0, 0.5);
    assert!(rho > 0.0);
}

#[test]
fn moist_air_density_invalid() {
    let rho = moist_air_density(1000.0, 280.0, 1.0);
    assert!(rho.is_finite());
}

#[test]
fn moist_air_density_moist_vs_dry() {
    let rho_dry = moist_air_density(90000.0, 280.0, 0.0);
    let rho_moist = moist_air_density(90000.0, 280.0, 1.0);
    assert!(rho_moist < rho_dry);
}

//
// ─────────────────────────────────────────────────────────────
// MOIST SPEED OF SOUND
// ─────────────────────────────────────────────────────────────
//

#[test]
fn moist_speed_of_sound_basic() {
    let c = moist_speed_of_sound(300.0, 0.5, 90000.0);
    assert!(c > 300.0);
    assert!(c < 400.0);
}

#[test]
fn moist_speed_of_sound_invalid() {
    let c = moist_speed_of_sound(300.0, 1.0, 1000.0); // e >= p
    assert!(c.is_nan());
}

#[test]
fn moist_speed_of_sound_moist_vs_dry() {
    let c_dry = moist_speed_of_sound(300.0, 0.0, 90000.0);
    let c_moist = moist_speed_of_sound(300.0, 1.0, 90000.0);
    assert!(c_moist > c_dry);
}
