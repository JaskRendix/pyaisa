from __future__ import annotations

from collections.abc import Callable

import numpy as np
from numpy.typing import ArrayLike, NDArray

from .core import ISA


def build_atm(**kwargs) -> Callable[[ArrayLike, float], tuple]:
    base = ISA(**kwargs)

    def atm(
        h: ArrayLike, dT: float = 0.0
    ) -> (
        tuple[float, float, float]
        | tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]
    ):
        if dT != 0.0:
            params = base.params.copy()
            params["T0"] += dT
            return ISA(**params).atm(h)
        return base.atm(h)

    return atm
