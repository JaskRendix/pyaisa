"""
Copyright (c) Pyaisa 2015  - Alberto Lorenzo
Copyright (c) Pyaisa 2026  - Giorgio
Distributed under the MIT License.
"""

from __future__ import annotations

from collections.abc import Callable

import numpy as np
from numpy.typing import NDArray

INTS: dict[str, int] = {
    "psize": -1,  # threshold for switching to parallel mode
}

DOUBLES: dict[str, float] = {
    "R": 287.05287,  # specific gas constant [J/(kg·K)]
    "g": 9.80665,  # gravitational acceleration [m/s²]
    "T0": 288.15,  # sea-level temperature [K]
    "p0": 101325.0,  # sea-level pressure [Pa]
}

ARRAYS: dict[str, NDArray[np.float64]] = {
    # COESA layer boundaries (geometric altitude)
    "h": np.array([0.0, 11000.0, 20000.0, 32000.0], dtype=float),
    # COESA lapse rates for each layer
    "a": np.array([-0.0065, 0.0, 0.0010], dtype=float),
}


class isa_params(dict):
    """
    Parameter container for ISA configuration.
    Validates layer structure and triggers refresh callbacks.
    """

    callback: Callable[[], None] | None

    def __init__(self, **kwargs) -> None:
        super().__init__()

        self._initializing = True

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

        self._initializing = False

    def __setitem__(self, key: str, value) -> None:
        # Integer parameters
        if key in INTS:
            value = int(value)

        # Float parameters
        elif key in DOUBLES:
            value = float(value)

        # Layer structure
        elif key == "layers":
            h_arr: NDArray[np.float64] = np.atleast_1d(value["h"]).astype(float)
            a_arr: NDArray[np.float64] = np.atleast_1d(value["a"]).astype(float)

            if h_arr.size != a_arr.size + 1 or a_arr.size == 0:
                raise ValueError('"h" array must be one element longer than "a" array')

            value = {"h": h_arr, "a": a_arr}

        # Store
        super().__setitem__(key, value)

        # Trigger refresh
        if (not self._initializing) and self.callback is not None:
            self.callback()

    def __delitem__(self, key: str) -> None:
        # Prevent deletion of ISA parameters
        pass
