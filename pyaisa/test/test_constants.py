import numpy as np
import pytest

from pyaisa.constants import ARRAYS, DOUBLES, INTS, isa_params


def test_defaults_loaded():
    p = isa_params()

    # integer defaults
    for key, val in INTS.items():
        assert p[key] == val

    # float defaults
    for key, val in DOUBLES.items():
        assert p[key] == val

    # layer defaults
    assert np.array_equal(p["layers"]["h"], ARRAYS["h"])
    assert np.array_equal(p["layers"]["a"], ARRAYS["a"])


def test_integer_override():
    p = isa_params(psize=500)
    assert p["psize"] == 500
    assert isinstance(p["psize"], int)


def test_float_override():
    p = isa_params(R=300.0)
    assert p["R"] == 300.0
    assert isinstance(p["R"], float)


def test_layer_override_valid():
    h = np.array([0.0, 5000.0, 10000.0], dtype=float)
    a = np.array([-0.0065, 0.0], dtype=float)

    p = isa_params(layers={"h": h, "a": a})

    assert np.array_equal(p["layers"]["h"], h)
    assert np.array_equal(p["layers"]["a"], a)


@pytest.mark.parametrize(
    "h, a",
    [
        (np.array([0.0, 10000.0]), np.array([-0.0065, 0.0])),  # h too short
        (np.array([0.0, 10000.0, 20000.0]), np.array([-0.0065])),  # mismatch
        (np.array([0.0]), np.array([])),  # empty lapse rates
    ],
)
def test_layer_override_invalid(h, a):
    with pytest.raises(ValueError):
        isa_params(layers={"h": h, "a": a})


def test_setitem_integer():
    p = isa_params()
    p["psize"] = 123
    assert p["psize"] == 123
    assert isinstance(p["psize"], int)


def test_setitem_float():
    p = isa_params()
    p["R"] = 300
    assert p["R"] == 300.0
    assert isinstance(p["R"], float)


def test_setitem_layers_valid():
    p = isa_params()
    h = np.array([0.0, 8000.0, 16000.0], dtype=float)
    a = np.array([-0.0065, 0.0], dtype=float)

    p["layers"] = {"h": h, "a": a}

    assert np.array_equal(p["layers"]["h"], h)
    assert np.array_equal(p["layers"]["a"], a)


def test_setitem_layers_invalid():
    p = isa_params()
    with pytest.raises(ValueError):
        p["layers"] = {"h": np.array([0.0, 10000.0]), "a": np.array([-0.0065, 0.0])}


def test_callback_called_on_setitem():
    calls = []

    def cb():
        calls.append(True)

    p = isa_params(callback=cb)
    p["R"] = 300.0

    assert len(calls) == 1


def test_callback_not_called_during_init():
    calls = []

    def cb():
        calls.append(True)

    p = isa_params(callback=cb)

    # callback should only be called after initialization
    # during init, callback is set *after* defaults are applied
    assert len(calls) == 0

    p["g"] = 9.7
    assert len(calls) == 1


def test_delitem_no_effect():
    p = isa_params()
    p["R"] = 300.0

    # __delitem__ is disabled
    p.__delitem__("R")

    # value must remain unchanged
    assert p["R"] == 300.0


def test_layers_are_copied_not_shared():
    p1 = isa_params()
    p2 = isa_params()

    # mutate p1 layers
    p1["layers"]["h"][0] = 999.0

    # p2 must remain unchanged
    assert p2["layers"]["h"][0] == ARRAYS["h"][0]
