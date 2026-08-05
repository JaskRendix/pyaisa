use pyaisa_core::icing::{freezing_fraction, icing_severity, lwc, supercooled_fraction};
use pyaisa_core::thermo::{saturation_vapor_pressure, vapor_pressure};

const EPS: f64 = 1e-9;

//
// --- LWC TESTS --------------------------------------------------------------
//

#[test]
fn lwc_zero_rh() {
    let t = 263.15;
    let rh = 0.0;
    assert!((lwc(t, rh)).abs() < EPS);
}

#[test]
fn lwc_full_rh() {
    let t = 263.15;
    let rh = 1.0;
    let e = vapor_pressure(t, rh);
    let es = saturation_vapor_pressure(t);
    let expected = 0.5 * (e / es).min(1.0);
    assert!((lwc(t, rh) - expected).abs() < EPS);
}

#[test]
fn lwc_monotonic_in_rh() {
    let t = 263.15;
    assert!(lwc(t, 0.8) > lwc(t, 0.4));
}

#[test]
fn lwc_caps_at_half() {
    let t = 263.15;
    let rh = 10.0; // absurd but tests min(1.0)
    assert!((lwc(t, rh) - 0.5).abs() < EPS);
}

//
// --- SUPERCOOLED FRACTION TESTS --------------------------------------------
//

#[test]
fn scf_above_freezing() {
    assert_eq!(supercooled_fraction(273.15), 0.0);
    assert_eq!(supercooled_fraction(280.0), 0.0);
}

#[test]
fn scf_below_minus20() {
    assert_eq!(supercooled_fraction(253.15), 1.0);
    assert_eq!(supercooled_fraction(240.0), 1.0);
}

#[test]
fn scf_linear_region() {
    let t = 263.15; // halfway between 273.15 and 253.15
    let sc = supercooled_fraction(t);
    assert!((sc - 0.5).abs() < EPS);
}

#[test]
fn scf_monotonic() {
    assert!(supercooled_fraction(260.0) > supercooled_fraction(265.0));
}

//
// --- ICING SEVERITY TESTS ---------------------------------------------------
//

#[test]
fn icing_severity_zero_when_no_lwc() {
    let t = 260.0;
    let rh = 0.0;
    assert!((icing_severity(t, rh)).abs() < EPS);
}

#[test]
fn icing_severity_zero_when_not_supercooled() {
    let t = 280.0;
    let rh = 1.0;
    assert!((icing_severity(t, rh)).abs() < EPS);
}

#[test]
fn icing_severity_increases_with_rh() {
    let t = 260.0;
    assert!(icing_severity(t, 0.8) > icing_severity(t, 0.4));
}

#[test]
fn icing_severity_increases_with_cooling() {
    let rh = 1.0;
    assert!(icing_severity(260.0, rh) > icing_severity(270.0, rh));
}

#[test]
fn icing_severity_caps_at_one() {
    let t = 240.0;
    let rh = 10.0;
    assert!(icing_severity(t, rh) <= 0.5 + EPS);
    assert!(icing_severity(t, rh) >= 0.5 - EPS);
}

//
// --- FREEZING FRACTION TESTS ------------------------------------------------
//

#[test]
fn freezing_fraction_matches_supercooled_fraction() {
    let temps = [280.0, 270.0, 260.0, 250.0];
    for &t in temps.iter() {
        assert!((freezing_fraction(t) - supercooled_fraction(t)).abs() < EPS);
    }
}

#[test]
fn freezing_fraction_bounds() {
    let t = 260.0;
    let ff = freezing_fraction(t);
    assert!((0.0..=1.0).contains(&ff));
}

#[test]
fn freezing_fraction_monotonic() {
    assert!(freezing_fraction(260.0) > freezing_fraction(270.0));
}
