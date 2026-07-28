from __future__ import annotations

from collections.abc import Callable
from warnings import warn

import numpy as np
from numpy.typing import ArrayLike, NDArray

from pyaisa.constants import isa_params
from pyaisa.pyaisa_core import ISA as RustISA

from .functions import (
    altitude_to_fl,
    cas_to_eas,
    density_altitude,
    dew_point,
    dynamic_pressure,
    dynamic_viscosity_sutherland,
    eas_to_tas,
    fl_to_altitude,
    freezing_fraction,
    geometric_to_fl,
    geometric_to_geopotential,
    geopotential_to_geometric,
    gust,
    icing_severity,
    indicated_altitude,
    kinematic_viscosity,
    lwc,
    mach,
    mach_from_tas,
    mixing_ratio,
    moist_air_density,
    moist_lapse_rate,
    moist_speed_of_sound,
    potential_temperature,
    prandtl_glauert,
    pressure_altitude,
    reynolds_number,
    saturation_vapor_pressure,
    speed_of_sound,
    stagnation_entropy,
    stagnation_pressure,
    stagnation_temperature,
    supercooled_fraction,
    tas_to_eas,
    vapor_pressure,
    virtual_temperature,
    wet_bulb_temperature,
    wind_ekman,
    wind_linear_shear,
    wind_loglaw,
    wind_loglaw_displaced,
    wind_power_law,
)


class ISA:
    _allow_refresh: bool
    _params: dict
    _isa: RustISA

    def __init__(self, **kwargs) -> None:
        self._allow_refresh = False
        self._params = isa_params(callback=self._refresh, **kwargs)
        self._allow_refresh = True
        self._refresh()

    def _refresh(self) -> None:
        if not self._allow_refresh:
            return

        rust_kwargs: dict[str, object] = {
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

    def atm(
        self, h: ArrayLike
    ) -> (
        tuple[float, float, float]
        | tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]
    ):
        h_arr = np.atleast_1d(h).astype(float)
        T, p, rho = self._isa.atm(h_arr)

        if np.isnan(T).any():
            warn("Altitude value outside range", RuntimeWarning)

        if h_arr.size == 1:
            return T.item(), p.item(), rho.item()

        return T, p, rho

    def atm_geopotential(
        self, H: ArrayLike
    ) -> (
        tuple[float, float, float]
        | tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]
    ):
        H_arr = np.atleast_1d(H).astype(float)
        h_arr = np.array([geopotential_to_geometric(H_i) for H_i in H_arr])

        T, p, rho = self._isa.atm(h_arr)

        if np.isnan(T).any():
            warn("Geopotential altitude value outside range", RuntimeWarning)

        if H_arr.size == 1:
            return T.item(), p.item(), rho.item()

        return T, p, rho

    def atm_moist(
        self, h: ArrayLike, rh: float
    ) -> (
        tuple[float, float, float]
        | tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]
    ):
        h_arr = np.atleast_1d(h).astype(float)
        T, p, _ = self._isa.atm(h_arr)

        rho_m = np.array([moist_air_density(pi, Ti, rh) for pi, Ti in zip(p, T)])

        if np.isnan(T).any():
            warn("Altitude value outside range", RuntimeWarning)

        if h_arr.size == 1:
            return T.item(), p.item(), rho_m.item()

        return T, p, rho_m

    def atm_deviation(
        self, h: ArrayLike, dT: float = 0.0, dp: float = 0.0, drho: float = 0.0
    ) -> (
        tuple[float, float, float]
        | tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]
    ):
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
        self, z: float, z_ref: float, u_ref: float, z0: float, d: float
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
    ) -> tuple[float, float]:
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

    def geometric_to_geopotential(self, h: float) -> float:
        return geometric_to_geopotential(h)

    def viscosity(self, h: float) -> tuple[float, float]:
        T, _, rho = self.atm(h)
        mu = dynamic_viscosity_sutherland(T)
        nu = kinematic_viscosity(mu, rho)
        return mu, nu

    def reynolds(self, h: float, V: float, L: float) -> float:
        _, _, rho = self.atm(h)
        mu = dynamic_viscosity_sutherland(self.atm(h)[0])
        return reynolds_number(rho, V, L, mu)

    def stagnation(self, h: float, V: float) -> tuple[float, float, float]:
        T, p, _ = self.atm(h)
        M = mach(V, speed_of_sound(T))
        return (
            stagnation_temperature(T, M),
            stagnation_pressure(p, M),
            stagnation_entropy(T, p),
        )

    def compressibility(self, h: float, V: float) -> float:
        M = self.mach(h, V)
        return prandtl_glauert(M)

    def tas_to_eas(self, h: float, tas: float) -> float:
        _, _, rho = self.atm(h)
        rho0 = 1.225
        return tas_to_eas(tas, rho, rho0)

    def eas_to_tas(self, h: float, eas: float) -> float:
        _, _, rho = self.atm(h)
        rho0 = 1.225
        return eas_to_tas(eas, rho, rho0)

    def cas_to_eas(self, cas: float) -> float:
        p0 = 101325.0
        rho0 = 1.225
        return cas_to_eas(cas, p0, rho0)

    def mach_from_tas(self, h: float, tas: float) -> float:
        a = self.speed_of_sound(h)
        return mach_from_tas(tas, a)

    def delta(self, h: float) -> float:
        """ISA pressure ratio δ = p / p0"""
        return self._isa.delta(h)

    def theta(self, h: float) -> float:
        """ISA temperature ratio θ = T / T0"""
        return self._isa.theta(h)

    def sigma(self, h: float) -> float:
        """ISA density ratio σ = ρ / ρ0"""
        return self._isa.sigma(h)

    def tropopause(self) -> float | None:
        """Geometric altitude of first zero‑lapse layer"""
        return self._isa.tropopause()

    def static_stability(self, h: float) -> float:
        """Brunt–Väisälä frequency squared at altitude"""
        return self._isa.static_stability(h)

    def isa_deviation(self, h: float) -> tuple[float, float, float]:
        """ISA deviation ΔT, Δp, Δρ at altitude"""
        dT, dp, drho = self._isa.isa_deviation(h)
        return dT, dp, drho
