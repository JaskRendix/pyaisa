import numpy as np
import pytest

from pyaisa import atm
from pyaisa.isa.factory import build_atm


def test_factory_returns_callable():
    f = build_atm()
    assert callable(f)


def test_factory_default_values():
    f = build_atm()
    T, p, rho = f(0.0)
    assert pytest.approx(T, rel=1e-6) == 288.15
    assert pytest.approx(p, rel=1e-6) == 101325.0
    assert pytest.approx(rho, rel=1e-6) == 1.225


def test_factory_param_override():
    f = build_atm(T0=300.0)
    T, _, _ = f(0.0)
    assert pytest.approx(T, rel=1e-6) == 300.0


@pytest.mark.parametrize("dT", [-20.0, -5.0, 0.0, 5.0, 20.0])
def test_temperature_offset(dT):
    f = build_atm()
    T0 = f(0.0)[0]
    T1 = f(0.0, dT=dT)[0]
    assert pytest.approx(T1, rel=1e-6) == T0 + dT


def test_temperature_offset_does_not_mutate_base():
    f = build_atm()
    T_before = f(0.0)[0]
    _ = f(0.0, dT=15.0)
    T_after = f(0.0)[0]
    assert T_before == T_after


@pytest.mark.parametrize(
    "h",
    [
        np.array([0.0]),
        np.array([0.0, 1000.0, 5000.0]),
        np.linspace(0, 20000, 10),
    ],
)
def test_array_shapes(h):
    f = build_atm()
    T, p, rho = f(h)

    if h.size == 1:
        # ISA returns scalars for size-1 arrays
        assert isinstance(T, float)
        assert isinstance(p, float)
        assert isinstance(rho, float)
    else:
        assert T.shape == h.shape
        assert p.shape == h.shape
        assert rho.shape == h.shape


def test_negative_altitude():
    f = build_atm()
    T, p, rho = f(-500.0)

    # ISA is undefined below 0 m → NaN is correct
    assert np.isnan(T)
    assert np.isnan(p)
    assert np.isnan(rho)


def test_extremely_high_altitude_warning():
    f = build_atm()
    with pytest.warns(RuntimeWarning):
        T, p, rho = f(200000.0)
    assert np.isnan(T) or np.isnan(p) or np.isnan(rho)


def test_nan_input_propagates():
    f = build_atm()
    T, p, rho = f(np.nan)
    assert np.isnan(T)
    assert np.isnan(p)
    assert np.isnan(rho)


def test_top_level_atm_scalar():
    T, p, rho = atm(0.0)
    assert pytest.approx(T, rel=1e-6) == 288.15
    assert pytest.approx(p, rel=1e-6) == 101325.0
    assert pytest.approx(rho, rel=1e-6) == 1.225


def test_top_level_atm_array():
    h = np.array([0.0, 1000.0])
    T, p, rho = atm(h)
    assert T.shape == h.shape
    assert p.shape == h.shape
    assert rho.shape == h.shape


def test_top_level_atm_temperature_offset():
    T0 = atm(0.0)[0]
    T1 = atm(0.0, dT=10.0)[0]
    assert pytest.approx(T1, rel=1e-6) == T0 + 10.0


def test_factory_instances_are_independent():
    f1 = build_atm(T0=280.0)
    f2 = build_atm(T0=300.0)
    assert f1(0.0)[0] != f2(0.0)[0]


def test_factory_does_not_mutate_original_params():
    f = build_atm(T0=280.0)
    _ = f(0.0, dT=20.0)
    assert pytest.approx(f(0.0)[0], rel=1e-6) == 280.0
