import pytest

from pyaisa.isa.performance import Performance

perf = Performance()


@pytest.mark.parametrize(
    "cd0, k, cl, expected_min",
    [
        (0.02, 0.04, 0.5, 0.02),  # nominal
        (0.02, 0.04, 0.0, 0.02),  # zero lift
        (0.02, 0.04, -0.5, 0.02),  # negative lift
        (0.02, 0.04, 2.0, 0.02),  # large lift
        (0.0, 0.04, 0.5, 0.0),  # zero CD0
        (0.02, 0.0, 0.5, 0.02),  # zero k
        (0.02, 0.04, -3.0, 0.02),  # pathological CL
    ],
)
def test_drag_polar(cd0: float, k: float, cl: float, expected_min: float):
    cd = perf.drag_polar(cd0, k, cl)
    assert isinstance(cd, float)
    assert cd >= expected_min


@pytest.mark.parametrize(
    "thrust_sl, T, p",
    [
        (10000.0, 288.15, 101325.0),  # sea level
        (10000.0, 223.15, 25000.0),  # high altitude
        (10000.0, 300.0, 100.0),  # near vacuum
        (0.0, 288.15, 101325.0),  # zero thrust
        (-5000.0, 288.15, 101325.0),  # negative thrust
        (10000.0, 100.0, 101325.0),  # extreme cold
        (10000.0, 400.0, 101325.0),  # extreme hot
    ],
)
def test_thrust_lapse(thrust_sl: float, T: float, p: float):
    tl = perf.thrust_lapse(thrust_sl, T, p)
    assert isinstance(tl, float)


@pytest.mark.parametrize(
    "q, S, CD",
    [
        (500.0, 20.0, 0.02),  # nominal
        (0.0, 20.0, 0.02),  # zero dynamic pressure
        (500.0, 0.0, 0.02),  # zero wing area
        (500.0, 20.0, 0.0),  # zero CD
        (500.0, 20.0, -0.02),  # negative CD
        (50000.0, 20.0, 0.02),  # extreme q
    ],
)
def test_drag_force(q: float, S: float, CD: float):
    D = perf.drag_force(q, S, CD)
    assert isinstance(D, float)


@pytest.mark.parametrize(
    "thrust, drag, V, weight",
    [
        (5000.0, 3000.0, 100.0, 60000.0),  # positive climb
        (3000.0, 3000.0, 100.0, 60000.0),  # zero climb
        (2000.0, 3000.0, 100.0, 60000.0),  # negative climb
        (5000.0, 3000.0, 100.0, 1e-6),  # near-zero weight
        (5000.0, 3000.0, 100.0, -60000.0),  # negative weight
        (5000.0, 3000.0, 0.0, 60000.0),  # zero velocity
        (20000.0, 3000.0, 100.0, 60000.0),  # extreme thrust
    ],
)
def test_climb_rate(thrust: float, drag: float, V: float, weight: float):
    rc = perf.climb_rate(thrust, drag, V, weight)
    assert isinstance(rc, float)


@pytest.mark.parametrize(
    "rc, expected",
    [
        (0.3, True),  # below threshold
        (0.5, True),  # exactly threshold
        (0.7, False),  # above threshold
        (-0.2, True),  # negative climb rate
        (10.0, False),  # extreme positive
        (-10.0, True),  # extreme negative
    ],
)
def test_service_ceiling(rc: float, expected: bool):
    assert perf.service_ceiling(rc) == expected
