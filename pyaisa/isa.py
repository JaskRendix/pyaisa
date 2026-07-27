"""
Copyright (c) Pyaisa 2015  - Alberto Lorenzo
Copyright (c) Pyaisa 2026  - Giorgio
Distributed under the MIT License.
"""

from __future__ import annotations

from collections.abc import Callable
from warnings import warn

import numpy as np
from numpy.typing import ArrayLike

from pyaisa._core import ISA as RustISA
from pyaisa._core import (
    altitude_to_fl,
    density_altitude,
    dew_point,
    dynamic_pressure,
    fl_to_altitude,
    freezing_fraction,
    geometric_to_fl,
    geometric_to_geopotential,
    geopotential_to_geometric,
    gust,
    icing_severity,
    indicated_altitude,
    lwc,
    mach,
    mixing_ratio,
    moist_air_density,
    moist_lapse_rate,
    moist_speed_of_sound,
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
from pyaisa.constants import isa_params


class ISA:
    """
    Python wrapper around the Rust ISA class.
    Provides a clean, modern interface for atmospheric queries.
    """

    def __init__(self, **kwargs) -> None:
        self._allow_refresh = False
        self._params = isa_params(callback=self._refresh, **kwargs)
        self._allow_refresh = True
        self._refresh()

    def _refresh(self) -> None:
        if not self._allow_refresh:
            return

        rust_kwargs = {
            "R": self._params["R"],
            "g": self._params["g"],
            "layers": self._params["layers"],
            "T0": self._params["T0"],
            "p0": self._params["p0"],
            "psize": self._params["psize"],
        }

        self._isa = RustISA(rust_kwargs)

    @property
    def params(self) -> dict:
        return self._params

    def atm(self, h: ArrayLike):
        h_arr = np.atleast_1d(h).astype(float)
        T, p, rho = self._isa.atm(h_arr)

        if np.isnan(T).any():
            warn("Altitude value outside range", RuntimeWarning)

        if h_arr.size == 1:
            return T.item(), p.item(), rho.item()

        return T, p, rho

    def atm_geopotential(self, H: ArrayLike):
        H_arr = np.atleast_1d(H).astype(float)
        h_arr = np.array([geopotential_to_geometric(H_i) for H_i in H_arr])

        T, p, rho = self._isa.atm(h_arr)

        if np.isnan(T).any():
            warn("Geopotential altitude value outside range", RuntimeWarning)

        if H_arr.size == 1:
            return T.item(), p.item(), rho.item()

        return T, p, rho

    def atm_moist(self, h: ArrayLike, rh: float):
        h_arr = np.atleast_1d(h).astype(float)
        T, p, _ = self._isa.atm(h_arr)

        rho_m = np.array([moist_air_density(pi, Ti, rh) for pi, Ti in zip(p, T)])

        if np.isnan(T).any():
            warn("Altitude value outside range", RuntimeWarning)

        if h_arr.size == 1:
            return T.item(), p.item(), rho_m.item()

        return T, p, rho_m

    def atm_deviation(self, h: ArrayLike, dT=0.0, dp=0.0, drho=0.0):
        h_arr = np.atleast_1d(h).astype(float)
        T, p, rho = self._isa.atm_deviation(h_arr, dT, dp, drho)

        if np.isnan(T).any():
            warn("Altitude value outside range", RuntimeWarning)

        if h_arr.size == 1:
            return T.item(), p.item(), rho.item()

        return T, p, rho

    def layer_at(self, h: float) -> int | None:
        return self._isa.layer_at(h)

    def speed_of_sound(self, h: float) -> float:
        T, _, _ = self.atm(h)
        return speed_of_sound(T)

    def speed_of_sound_moist(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        return moist_speed_of_sound(T, rh)

    def mach(self, h: float, V: float) -> float:
        return mach(V, self.speed_of_sound(h))

    def mach_moist(self, h: float, V: float, rh: float) -> float:
        a = self.speed_of_sound_moist(h, rh)
        return mach(V, a)

    def dynamic_pressure(self, h: float, V: float) -> float:
        _, _, rho = self.atm(h)
        return dynamic_pressure(rho, V)

    def dynamic_pressure_moist(self, h: float, V: float, rh: float) -> float:
        _, _, rho_m = self.atm_moist(h, rh)
        return dynamic_pressure(rho_m, V)

    def pressure_altitude(self, h: float) -> float:
        _, p, _ = self.atm(h)
        return pressure_altitude(p)

    def density_altitude(self, h: float) -> float:
        T, p, _ = self.atm(h)
        return density_altitude(p, T)

    def saturation_vapor_pressure(self, h: float) -> float:
        T, _, _ = self.atm(h)
        return saturation_vapor_pressure(T)

    def vapor_pressure(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        return vapor_pressure(T, rh)

    def mixing_ratio(self, h: float, rh: float) -> float:
        T, p, _ = self.atm(h)
        e = vapor_pressure(T, rh)
        return mixing_ratio(p, e)

    def dew_point(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        e = vapor_pressure(T, rh)
        return dew_point(e)

    def virtual_temperature(self, h: float, rh: float) -> float:
        T, p, _ = self.atm(h)
        e = vapor_pressure(T, rh)
        w = mixing_ratio(p, e)
        return virtual_temperature(T, w)

    def wind(
        self, z: float, z_ref: float = 10.0, u_ref: float = 5.0, z0: float = 0.1
    ) -> float:
        return wind_loglaw(z, z_ref, u_ref, z0)

    def wind_power_law(
        self, z: float, z_ref: float = 10.0, u_ref: float = 5.0, alpha: float = 0.14
    ) -> float:
        return wind_power_law(z, z_ref, u_ref, alpha)

    def wind_loglaw_displaced(
        self,
        z: float,
        z_ref: float = 10.0,
        u_ref: float = 5.0,
        z0: float = 0.1,
        d: float = 0.0,
    ) -> float:
        return wind_loglaw_displaced(z, z_ref, u_ref, z0, d)

    def wind_linear_shear(
        self, z: float, z0: float, z1: float, u0: float, u1: float
    ) -> float:
        return wind_linear_shear(z, z0, z1, u0, u1)

    def wind_ekman(
        self,
        z: float,
        u0: float,
        v0: float,
        z_ek: float = 300.0,
        angle_max_deg: float = 30.0,
    ):
        return wind_ekman(z, u0, v0, z_ek, angle_max_deg)

    def gust(self, u_mean: float, g_factor: float = 0.3) -> float:
        return gust(u_mean, g_factor)

    def potential_temperature(self, h: float) -> float:
        T, p, _ = self.atm(h)
        return potential_temperature(T, p)

    def moist_lapse_rate(self, h: float, rh: float) -> float:
        T, p, _ = self.atm(h)
        return moist_lapse_rate(T, p, rh)

    def wet_bulb_temperature(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        return wet_bulb_temperature(T, rh)

    def altitude_to_fl(self, h: float) -> float:
        _, p, _ = self.atm(h)
        return altitude_to_fl(pressure_altitude(p))

    def fl_to_altitude(self, fl: float) -> float:
        return fl_to_altitude(fl)

    def geometric_to_fl(self, h: float) -> float:
        T, p, _ = self.atm(h)
        return geometric_to_fl(h, p)

    def indicated_altitude(self, h: float, qnh: float) -> float:
        _, p, _ = self.atm(h)
        return indicated_altitude(h, p, qnh)

    def lwc(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        return lwc(T, rh)

    def supercooled_fraction(self, h: float) -> float:
        T, _, _ = self.atm(h)
        return supercooled_fraction(T)

    def icing_severity(self, h: float, rh: float) -> float:
        T, _, _ = self.atm(h)
        return icing_severity(T, rh)

    def freezing_fraction(self, h: float) -> float:
        T, _, _ = self.atm(h)
        return freezing_fraction(T)


def build_atm(**kwargs) -> Callable[[ArrayLike, float], tuple]:
    base = ISA(**kwargs)

    def atm(h: ArrayLike, dT: float = 0.0):
        if dT != 0.0:
            params = base.params.copy()
            params["T0"] += dT
            return ISA(**params).atm(h)
        return base.atm(h)

    return atm


atm = build_atm()
