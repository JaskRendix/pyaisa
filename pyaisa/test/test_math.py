import numpy as np
import pytest

from pyaisa import atm
from pyaisa._core import (
    altitude_to_fl,
    density_altitude,
    dew_point,
    dynamic_pressure,
    fl_to_altitude,
    freezing_fraction,
    geometric_to_fl,
    gust,
    icing_severity,
    indicated_altitude,
    lwc,
    mach,
    mixing_ratio,
    moist_lapse_rate,
    potential_temperature,
    pressure_altitude,
    saturation_vapor_pressure,
    speed_of_sound,
    supercooled_fraction,
    vapor_pressure,
    virtual_temperature,
    wet_bulb_temperature,
    wind_ekman,
    wind_linear_shear,
    wind_loglaw,
    wind_loglaw_displaced,
    wind_power_law,
)


@pytest.mark.parametrize(
    "T, expected",
    [
        (288.15, 340.29),  # sea level
        (216.65, 295.07),  # 11 km ISA
    ],
)
def test_speed_of_sound(T, expected):
    assert pytest.approx(speed_of_sound(T), rel=1e-3) == expected


def test_speed_of_sound_from_atm():
    T, _, _ = atm(11000)
    a = speed_of_sound(T)
    assert pytest.approx(a, rel=1e-3) == 295.0


@pytest.mark.parametrize(
    "rho, V, expected",
    [
        (1.225, 50.0, 1531.25),
        (0.3639, 250.0, 11371.875),
    ],
)
def test_dynamic_pressure(rho, V, expected):
    assert pytest.approx(dynamic_pressure(rho, V), rel=1e-6) == expected


@pytest.mark.parametrize(
    "V, a, expected",
    [
        (170.0, 340.0, 0.5),
        (300.0, 295.0, 1.0169),
    ],
)
def test_mach(V, a, expected):
    assert pytest.approx(mach(V, a), rel=1e-3) == expected


@pytest.mark.parametrize(
    "p, expected",
    [
        (101325.0, 0.0),  # sea level
        (79495.0, 2000.0),  # ~2 km
        (5474.8, 18887.33),  # ~18 km
    ],
)
def test_pressure_altitude(p, expected):
    assert pytest.approx(pressure_altitude(p), rel=1e-2) == expected


def test_density_altitude_standard():
    da = density_altitude(101325.0, 288.15)
    assert pytest.approx(da, abs=1e-6) == 0.0


def test_density_altitude_hot_day():
    # +20°C hotter → Δh = ΔT / L = 20 / 0.0065 = 3076.92 m
    da = density_altitude(101325.0, 308.15)
    assert pytest.approx(da, rel=1e-2) == 2376.0


@pytest.mark.parametrize(
    "T, expected",
    [
        (293.15, 2338.8),  # 20°C
        (303.15, 4246.0),  # 30°C
    ],
)
def test_saturation_vapor_pressure(T, expected):
    assert pytest.approx(saturation_vapor_pressure(T), rel=1e-2) == expected


def test_vapor_pressure():
    T = 293.15
    rh = 0.5
    e = vapor_pressure(T, rh)
    assert pytest.approx(e, rel=1e-2) == saturation_vapor_pressure(T) * 0.5


def test_mixing_ratio():
    p = 90000.0
    e = 2000.0
    w = mixing_ratio(p, e)
    assert pytest.approx(w, rel=1e-3) == 0.622 * e / (p - e)


def test_dew_point():
    e = 2338.8  # vapor pressure at 20°C
    td = dew_point(e)
    assert pytest.approx(td, rel=1e-2) == 293.15  # 20°C in Kelvin


def test_virtual_temperature():
    T = 300.0
    w = 0.01
    Tv = virtual_temperature(T, w)
    assert pytest.approx(Tv, rel=1e-6) == T * (1 + 0.61 * w)


@pytest.mark.parametrize(
    "z, z_ref, u_ref, z0, expected",
    [
        (10.0, 10.0, 5.0, 0.1, 5.0),
        (20.0, 10.0, 5.0, 0.1, 5.0 * np.log(20 / 0.1) / np.log(10 / 0.1)),
    ],
)
def test_wind_loglaw(z, z_ref, u_ref, z0, expected):
    assert pytest.approx(wind_loglaw(z, z_ref, u_ref, z0), rel=1e-6) == expected


def test_wind_power_law():
    z = 20.0
    z_ref = 10.0
    u_ref = 5.0
    alpha = 0.14

    expected = u_ref * (z / z_ref) ** alpha
    assert pytest.approx(wind_power_law(z, z_ref, u_ref, alpha), rel=1e-6) == expected


def test_wind_loglaw_displaced():
    z = 20.0
    z_ref = 10.0
    u_ref = 5.0
    z0 = 0.1
    d = 2.0

    expected = u_ref * np.log((z - d) / z0) / np.log((z_ref - d) / z0)
    assert (
        pytest.approx(wind_loglaw_displaced(z, z_ref, u_ref, z0, d), rel=1e-6)
        == expected
    )


def test_wind_linear_shear():
    z0 = 0.0
    z1 = 100.0
    u0 = 5.0
    u1 = 15.0

    # midpoint
    z = 50.0
    expected = u0 + (z - z0) / (z1 - z0) * (u1 - u0)

    assert pytest.approx(wind_linear_shear(z, z0, z1, u0, u1), rel=1e-6) == expected


def test_wind_linear_shear_bounds():
    z0 = 10.0
    z1 = 20.0
    u0 = 3.0
    u1 = 9.0

    assert wind_linear_shear(5.0, z0, z1, u0, u1) == u0
    assert wind_linear_shear(25.0, z0, z1, u0, u1) == u1


def test_wind_ekman_rotation():
    # Base wind: 10 m/s east (u0), 0 m/s north (v0)
    u0 = 10.0
    v0 = 0.0

    z = 300.0
    z_ek = 300.0
    angle_max_deg = 30.0

    # Expected: rotate by 30 degrees
    speed = np.sqrt(u0**2 + v0**2)
    angle = np.radians(angle_max_deg)

    expected_u = speed * np.cos(angle)
    expected_v = speed * np.sin(angle)

    u, v = wind_ekman(z, u0, v0, z_ek, angle_max_deg)

    assert pytest.approx(u, rel=1e-6) == expected_u
    assert pytest.approx(v, rel=1e-6) == expected_v


def test_gust():
    u_mean = 10.0
    g_factor = 0.3
    expected = u_mean * (1.0 + g_factor)

    assert pytest.approx(gust(u_mean, g_factor), rel=1e-6) == expected


def test_potential_temperature_dry():
    T = 288.15  # K
    p = 101325.0  # Pa
    theta = potential_temperature(T, p)
    # At sea level, θ ≈ T
    expected = T * (100000.0 / p) ** (287.05287 / 1004.0)
    assert pytest.approx(theta, rel=1e-3) == expected


def test_moist_lapse_rate_sign():
    T = 283.15  # 10°C
    p = 90000.0
    rh = 0.8
    gamma_m = moist_lapse_rate(T, p, rh)
    # Should be positive (K/m) and less than dry (~0.0098 K/m)
    assert gamma_m > 0.0
    assert gamma_m < 0.0098


def test_wet_bulb_temperature_bounds():
    T = 293.15  # 20°C
    rh = 0.5
    Tw = wet_bulb_temperature(T, rh)
    # Wet-bulb between air temperature and ~0°C
    assert 273.15 <= Tw <= T


def test_altitude_to_fl_and_back():
    h_p = 10000.0  # 10 000 ft
    fl = altitude_to_fl(h_p)
    assert pytest.approx(fl_to_altitude(fl), rel=1e-6) == h_p


def test_geometric_to_fl_consistency():
    # Just check it returns a finite value
    h = 3000.0
    T, p, _ = atm(h)
    fl = geometric_to_fl(h, p)
    assert np.isfinite(fl)


def test_indicated_altitude_qnh_shift():
    h = 1000.0
    T, p, _ = atm(h)
    qnh_high = p + 1000.0
    qnh_low = p - 1000.0

    h_ind_high = indicated_altitude(h, p, qnh_high)
    h_ind_low = indicated_altitude(h, p, qnh_low)

    # Higher QNH → lower indicated altitude
    assert h_ind_high < h_ind_low


def test_lwc_monotonic_in_rh():
    T = 273.15  # 0°C
    lwc_low = lwc(T, 0.3)
    lwc_high = lwc(T, 0.9)
    assert lwc_high > lwc_low


@pytest.mark.parametrize(
    "T, expected",
    [
        (275.0, 0.0),  # above freezing
        (260.0, (273.15 - 260.0) / 20.0),  # between -20 and 0°C
        (250.0, 1.0),  # below -20°C
    ],
)
def test_supercooled_fraction_profile(T, expected):
    assert pytest.approx(supercooled_fraction(T), rel=1e-6) == expected


def test_icing_severity_range():
    T = 268.15  # -5°C
    rh = 0.8
    s = icing_severity(T, rh)
    assert 0.0 <= s <= 1.0


def test_freezing_fraction_matches_supercooled():
    T = 260.0
    assert pytest.approx(freezing_fraction(T), rel=1e-6) == supercooled_fraction(T)
