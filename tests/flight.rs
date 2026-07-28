use pyaisa_core::flight::{
    altitude_to_fl, density_altitude, fl_to_altitude, geometric_to_fl, geometric_to_geopotential,
    geopotential_to_geometric, indicated_altitude, pressure_altitude,
};

const EPS: f64 = 1e-9;

#[test]
fn fl_round_trip() {
    let h = 35000.0;
    let fl = altitude_to_fl(h);
    let h2 = fl_to_altitude(fl);
    assert!((h2 - h).abs() < EPS);
}

#[test]
fn fl_zero() {
    assert_eq!(altitude_to_fl(0.0), 0.0);
    assert_eq!(fl_to_altitude(0.0), 0.0);
}

#[test]
fn fl_negative_altitude() {
    let h = -500.0;
    let fl = altitude_to_fl(h);
    assert_eq!(fl, -5.0);
    assert_eq!(fl_to_altitude(fl), h);
}

#[test]
fn pressure_altitude_sea_level() {
    let pa = pressure_altitude(101325.0);
    assert!(pa.abs() < EPS);
}

#[test]
fn pressure_altitude_high() {
    let pa = pressure_altitude(50000.0);
    assert!(pa > 0.0);
    assert!(pa < 20000.0);
}

#[test]
fn pressure_altitude_monotonic() {
    let p1 = 90000.0;
    let p2 = 80000.0;
    assert!(pressure_altitude(p2) > pressure_altitude(p1));
}

#[test]
fn density_altitude_sea_level_std() {
    let da = density_altitude(101325.0, 288.15);
    assert!(da.abs() < EPS);
}

#[test]
fn density_altitude_hot_air() {
    let da = density_altitude(101325.0, 308.15);
    assert!(da > 0.0);
}

#[test]
fn density_altitude_cold_air() {
    let da = density_altitude(101325.0, 268.15);
    assert!(da < 0.0);
}

#[test]
fn geometric_to_geopotential_round_trip() {
    let h = 8000.0;
    let g = geometric_to_geopotential(h);
    let h2 = geopotential_to_geometric(g);
    assert!((h2 - h).abs() < 1e-6);
}

#[test]
fn geometric_to_geopotential_zero() {
    assert_eq!(geometric_to_geopotential(0.0), 0.0);
}

#[test]
fn geopotential_to_geometric_zero() {
    assert_eq!(geopotential_to_geometric(0.0), 0.0);
}

#[test]
fn geopotential_to_geometric_infinite() {
    let re = 6_356_766.0;
    assert!(geopotential_to_geometric(re).is_infinite());
}

#[test]
fn geometric_to_geopotential_monotonic() {
    let h1 = 1000.0;
    let h2 = 2000.0;
    assert!(geometric_to_geopotential(h2) > geometric_to_geopotential(h1));
}

#[test]
fn indicated_altitude_qnh_equal_pressure() {
    let h = 1500.0;
    let p = 90000.0;
    assert!((indicated_altitude(h, p, p) - h).abs() < EPS);
}

#[test]
fn indicated_altitude_qnh_higher() {
    let h = 1500.0;
    let p = 90000.0;
    let qnh = 101325.0;
    let ind = indicated_altitude(h, p, qnh);
    assert!(ind < h);
}

#[test]
fn indicated_altitude_qnh_lower() {
    let h = 1500.0;
    let p = 101325.0;
    let qnh = 90000.0;
    let ind = indicated_altitude(h, p, qnh);
    assert!(ind > h);
}

#[test]
fn geometric_to_fl_consistency() {
    let p = 70000.0;
    let fl = geometric_to_fl(5000.0, p);
    let pa = pressure_altitude(p);
    assert!((fl - altitude_to_fl(pa)).abs() < EPS);
}

#[test]
fn pressure_altitude_extremely_low_pressure() {
    let pa = pressure_altitude(1.0);
    assert!(pa > 30000.0);
    assert!(pa < 45000.0);
}

#[test]
fn pressure_altitude_extremely_high_pressure() {
    let pa = pressure_altitude(200000.0);
    assert!(pa < 0.0);
}

#[test]
fn density_altitude_extreme_temperature() {
    let da_hot = density_altitude(101325.0, 400.0);
    let da_cold = density_altitude(101325.0, 150.0);
    assert!(da_hot > 0.0);
    assert!(da_cold < 0.0);
}
