"""
Copyright (c) Pyaisa 2015  - Alberto Lorenzo
Copyright (c) Pyaisa 2026  - Giorgio
Distributed under the MIT License.
"""

import numpy as np

INTS = {
    "psize": -1,  # threshold for switching to parallel mode
}

DOUBLES = {
    "R": 287.05287,  # specific gas constant [J/(kg·K)]
    "g": 9.80665,  # gravitational acceleration [m/s²]
    "T0": 288.15,  # sea-level temperature [K]
    "p0": 101325.0,  # sea-level pressure [Pa]
}

ARRAYS = {
    # COESA layer boundaries (geometric altitude)
    "h": np.array([0.0, 11000.0, 20000.0, 32000.0], dtype=float),
    # COESA lapse rates for each layer
    "a": np.array([-0.0065, 0.0, 0.0010], dtype=float),
}


class isa_params(dict):
    """
    Clean, modern parameter container for ISA configuration.
    Automatically validates layer structure and triggers refresh callbacks.
    """

    def __init__(self, **kwargs):
        super().__init__()

        # Callback must be defined before any __setitem__
        self.callback = kwargs.pop("callback", None)

        # Load defaults
        self.update(INTS)
        self.update(DOUBLES)

        # Load default layers (uses __setitem__)
        self["layers"] = {
            "h": ARRAYS["h"].copy(),
            "a": ARRAYS["a"].copy(),
        }

        # Apply user overrides
        for key, value in kwargs.items():
            self.__setitem__(key, value)

    def __setitem__(self, key, value):
        # Integer parameters
        if key in INTS:
            value = int(value)

        # Float parameters
        elif key in DOUBLES:
            value = float(value)

        # Layer structure
        elif key == "layers":
            h = np.atleast_1d(value["h"]).astype(float)
            a = np.atleast_1d(value["a"]).astype(float)

            if h.size != a.size + 1 or a.size == 0:
                raise ValueError('"h" array must be one element longer than "a" array')

            value = {"h": h, "a": a}

        # Store
        super().__setitem__(key, value)

        # Trigger refresh
        if self.callback is not None:
            self.callback()

    def __delitem__(self, key):
        pass
