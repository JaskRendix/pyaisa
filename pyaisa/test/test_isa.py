import numpy as np
import pytest
from numpy.testing import assert_almost_equal, assert_array_almost_equal, assert_equal

from pyaisa import atm
from pyaisa.isa.core import ISA
from pyaisa.pyaisa_core import (
    geometric_to_geopotential,
    geopotential_to_geometric,
    moist_speed_of_sound,
    speed_of_sound,
)


@pytest.mark.parametrize(
    "h, expected_T, expected_p, expected_rho",
    [
        (0.0, 288.15, 101325.0, 1.2250),
        (50.0, 287.825, 100720.0, 1.2191),
        (550.0, 284.575, 94890.0, 1.1616),
    ],
)
def test_scalar_values(h, expected_T, expected_p, expected_rho):
    T, p, rho = atm(h)
    assert_almost_equal(T, expected_T, decimal=3)
    assert_almost_equal(p, expected_p, decimal=-1)
    assert_almost_equal(rho, expected_rho, decimal=4)


def test_scalar_output_types():
    T, p, rho = atm(0.0)
    assert isinstance(T, float)
    assert isinstance(p, float)
    assert isinstance(rho, float)


@pytest.mark.parametrize(
    "h",
    [
        np.zeros(5),
        np.linspace(0, 11000, 10),
        np.array([0.0, 100.0, 500.0, 1000.0]),
    ],
)
def test_array_output_shapes(h):
    T, p, rho = atm(h)
    assert_equal(len(T), len(h))
    assert_equal(len(p), len(h))
    assert_equal(len(rho), len(h))


def test_warning_for_out_of_range(recwarn):
    atm(-1.0)
    w = recwarn.pop(RuntimeWarning)
    assert issubclass(w.category, RuntimeWarning)


def test_nan_for_out_of_range_array():
    h = np.array([-1.0, 0.0])
    T, p, rho = atm(h)
    assert np.isnan(T[0])
    assert np.isnan(p[0])
    assert np.isnan(rho[0])
    assert not np.isnan(T[1])


@pytest.mark.parametrize(
    "h",
    [
        11000.0,
        20000.0,
        32000.0,
    ],
)
def test_layer_boundaries(h):
    T, p, rho = atm(h)
    assert isinstance(T, float)
    assert isinstance(p, float)
    assert isinstance(rho, float)
    assert not np.isnan(T)


def test_results_under_11km():
    h = np.array([0.0, 50.0, 550.0, 6500.0, 10000.0, 11000.0])
    expected_T = np.array([288.150, 287.825, 284.575, 245.900, 223.150, 216.650])
    expected_p = np.array([101325.0, 100720.0, 94890.0, 44034.0, 26436.0, 22632.0])
    expected_rho = np.array([1.2250, 1.2191, 1.1616, 0.62384, 0.41271, 0.36392])

    T, p, rho = atm(h)
    assert_array_almost_equal(T, expected_T, decimal=3)
    assert_array_almost_equal(p, expected_p, decimal=-1)
    assert_array_almost_equal(rho, expected_rho, decimal=4)


def test_results_under_20km():
    h = np.array([12000, 14200, 17500, 20000])
    expected_T = np.array([216.650, 216.650, 216.650, 216.650])
    expected_p = np.array([19330.0, 13663.0, 8120.5, 5474.8])
    expected_rho = np.array([0.31083, 0.21971, 0.13058, 0.088035])

    T, p, rho = atm(h)
    assert_array_almost_equal(T, expected_T, decimal=3)
    assert_array_almost_equal(p, expected_p, decimal=0)
    assert_array_almost_equal(rho, expected_rho, decimal=5)


def test_results_under_32km():
    h = np.array([22100, 24000, 28800, 32000])
    expected_T = np.array([218.750, 220.650, 225.450, 228.650])
    expected_p = np.array([3937.7, 2930.4, 1404.8, 868.01])
    expected_rho = np.array([0.062711, 0.046267, 0.021708, 0.013225])

    T, p, rho = atm(h)
    assert_array_almost_equal(T, expected_T, decimal=3)
    assert_array_almost_equal(p, expected_p, decimal=1)
    assert_array_almost_equal(rho, expected_rho, decimal=5)


def test_isa_object_scalar():
    isa = ISA()
    T, p, rho = isa.atm(0.0)
    assert_equal(T, 288.15)
    assert_equal(p, 101325.0)


def test_isa_object_array():
    isa = ISA()
    h = np.array([0.0, 100.0])
    T, p, rho = isa.atm(h)
    assert_equal(len(T), 2)
    assert_equal(len(p), 2)
    assert_equal(len(rho), 2)


@pytest.mark.parametrize("psize", [-1, 0])
def test_parallel_modes(psize):
    isa = ISA(psize=psize)
    h = np.linspace(0, 11000, 100)
    T, p, rho = isa.atm(h)
    assert_equal(len(T), 100)


def test_geopotential_matches_geometric():
    isa = ISA()
    H = 11000.0
    h = geopotential_to_geometric(H)

    T1, p1, rho1 = isa.atm_geopotential(H)
    T2, p2, rho2 = isa.atm(h)

    assert_almost_equal(T1, T2)
    assert_almost_equal(p1, p2)
    assert_almost_equal(rho1, rho2)


def test_moist_air_density_lower_than_dry():
    isa = ISA()
    h = 5000.0

    T, p, rho_dry = isa.atm(h)
    Tm, pm, rho_moist = isa.atm_moist(h, rh=0.8)

    assert rho_moist < rho_dry


def test_isa_deviation_temperature():
    isa = ISA()
    T0, _, _ = isa.atm(0)
    T1, _, _ = isa.atm_deviation(0, dT=10)

    assert_almost_equal(T1, T0 + 10)


def test_layer_introspection():
    isa = ISA()

    assert isa.layer_at(0) == 0
    assert isa.layer_at(15000) == 1
    assert isa.layer_at(30000) == 2


@pytest.mark.parametrize("h", [0.0, 5000.0, 11000.0, 15000.0, 30000.0])
def test_geopotential_geometric_roundtrip_and_atm(h):
    isa = ISA()

    H = geometric_to_geopotential(h)
    h2 = geopotential_to_geometric(H)

    assert_almost_equal(h2, h, decimal=6)

    T1, p1, rho1 = isa.atm_geopotential(H)
    T2, p2, rho2 = isa.atm(h)

    assert_almost_equal(T1, T2)
    assert_almost_equal(p1, p2)
    assert_almost_equal(rho1, rho2)


def test_isa_ratios():
    isa = ISA()
    h = 0.0

    delta = isa.delta(h)
    theta = isa.theta(h)
    sigma = isa.sigma(h)

    # At sea level, all ratios should be 1
    assert_almost_equal(delta, 1.0)
    assert_almost_equal(theta, 1.0)
    assert_almost_equal(sigma, 1.0)


def test_tropopause_detection():
    isa = ISA()
    tropo = isa.tropopause()

    # Default ICAO ISA tropopause is at 11 km
    assert_almost_equal(tropo, 11000.0, decimal=0)


@pytest.mark.parametrize("h", [0.0, 5000.0, 11000.0, 20000.0])
def test_static_stability(h):
    isa = ISA()
    N2 = isa.static_stability(h)

    # Static stability must be finite and non‑NaN
    assert np.isfinite(N2)


def test_isa_deviation_zero():
    isa = ISA()
    dT, dp, drho = isa.isa_deviation(5000.0)

    # Standard ISA deviation at altitude should be zero
    assert_almost_equal(dT, 0.0)
    assert_almost_equal(dp, 0.0)
    assert_almost_equal(drho, 0.0)


def test_isa_deviation_nonzero():
    isa = ISA()
    T0, p0, rho0 = isa.atm(5000.0)

    # Apply deviation
    T1, p1, rho1 = isa.atm_deviation(5000.0, dT=5.0, dp=100.0, drho=0.01)
    dT, dp, drho = isa.isa_deviation(5000.0)

    # Deviation must match applied values
    assert_almost_equal(T1, T0 + 5.0)
    assert_almost_equal(p1, p0 + 100.0)
    assert_almost_equal(rho1, rho0 + 0.01)


def test_mach_moist_vs_dry():
    T, p, _ = atm(5000)

    a_dry = speed_of_sound(T)
    isa = ISA()
    a_moist = moist_speed_of_sound(isa._isa, 5000, 1.0)

    m_dry = 250.0 / a_dry
    m_moist = 250.0 / a_moist

    assert m_moist < m_dry


def test_virtual_potential_temperature():
    isa = ISA()
    theta = isa.virtual_potential_temperature(5000, rh=0.5)
    theta_dry = isa.potential_temperature(5000)
    assert theta > theta_dry


def test_equivalent_potential_temperature():
    isa = ISA()
    theta_e = isa.equivalent_potential_temperature(5000, rh=0.5)
    theta = isa.potential_temperature(5000)
    assert theta_e > theta


def test_moist_static_energy():
    isa = ISA()
    mse = isa.moist_static_energy(5000, rh=0.5)
    assert mse > 0.0
