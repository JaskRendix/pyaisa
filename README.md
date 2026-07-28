# Pyaisa

Pyaisa implements the International Standard Atmosphere (ISA) with a Rust core exposed through PyO3.  
It provides temperature, pressure, and density as functions of altitude, following the COESA reference model.  
The project originally used Python with a C++ backend and now uses Rust for clarity and extensibility.

Repository: [https://github.com/newlawrence/Pyaisa](https://github.com/newlawrence/Pyaisa)

---

## Features

### Core ISA Model
- COESA‑based standard atmosphere  
- Scalar and vector altitude evaluation  
- NumPy array support  
- Configurable parameters (`R`, `g`, `T0`, `p0`, layer structure)  
- ISA object and `build_atm` constructor  
- Geometric and geopotential altitude support  

### Thermodynamics and Humidity  
Implemented in `thermo.rs`:

- Saturation vapor pressure  
- Vapor pressure  
- Mixing ratio  
- Dew point  
- Virtual temperature  
- Potential temperature  
- Moist adiabatic lapse rate  
- Wet‑bulb temperature  
- Moist‑air density  
- Moist‑air speed of sound  

### Aerodynamic and Compressible‑Flow Quantities  
Implemented in `math.rs`:

- Speed of sound  
- Dynamic pressure  
- Mach number  
- Dynamic viscosity (Sutherland)  
- Kinematic viscosity  
- Reynolds number  
- Stagnation temperature  
- Stagnation pressure  
- Stagnation entropy  
- Prandtl–Glauert factor  
- Airspeed conversions (CAS→EAS, EAS↔TAS, Mach from TAS)
- CAS↔Mach conversions
- TAS↔CAS conversions
- Moist‑air Mach number

### Wind Models  
Implemented in `wind.rs`:

- Log‑law wind profile  
- Power‑law wind profile  
- Displaced log‑law  
- Linear shear  
- Ekman‑type rotation  
- Gust factor  

### Flight‑Level and Altitude Conversions  
Implemented in `flight.rs`:

- Pressure altitude  
- Density altitude  
- Geometric↔geopotential altitude  
- Flight level conversions  
- Indicated altitude with QNH correction  

### Icing Conditions  
Implemented in `icing.rs`:

- Liquid water content  
- Supercooled fraction  
- Icing severity index  
- Freezing fraction  

---

## ISA Extensions

### Geopotential Altitude

```python
isa.atm_geopotential(H)
geometric_to_geopotential(h)
geopotential_to_geometric(H)
```

### Moist‑Air ISA

```python
isa.atm_moist(h, rh)
isa.speed_of_sound_moist(h, rh)
isa.dynamic_pressure_moist(h, V, rh)
```

### ISA Deviations

```python
isa.atm_deviation(h, dT=10)
isa.atm_deviation(h, dp=-500)
```

### Layer Introspection

```python
isa.layer_at(15000)
```

---

## ISA Diagnostics

### ISA Ratios

```python
isa.delta(h)   # pressure ratio p / p0
isa.theta(h)   # temperature ratio T / T0
isa.sigma(h)   # density ratio rho / rho0
```

### Tropopause Detection

```python
isa.tropopause()
```

### Static Stability (Brunt–Väisälä Frequency)

```python
isa.static_stability(h)
```

### ISA Deviation Reporting

```python
isa.isa_deviation(h)
```

Returns:

```python
(dT, dp, drho)
```

---

## Installation

### Editable mode

```
pip install -e .[test]
```

### Standard installation

```
pip install .
```

---

## Testing

```
pytest
```

---

## Basic Usage

```python
import pyaisa

isa = pyaisa.ISA()
isa.atm(0)
isa.atm([0, 11000])
```

### Modifying Parameters

```python
new = pyaisa.ISA(R=300)
new.atm(0)

new.params["R"] = 287.05287
new.atm(0)
```

### Using `build_atm`

```python
atm = pyaisa.build_atm(R=300, g=10)
atm(11000)
```

### Temperature Offset

```python
pyaisa.atm(0, 15)
pyaisa.atm(11000, 15)
```

---

## Extended Physics Examples

### ICAO Airspeed Conversions

```python
# CAS ↔ EAS
eas = isa.cas_to_eas(100)
cas = isa.eas_to_cas(100)

# EAS ↔ TAS
tas = isa.eas_to_tas(10000, eas=120)
eas2 = isa.tas_to_eas(10000, tas)

# CAS ↔ TAS
tas = isa.cas_to_tas(5000, cas=150)
cas2 = isa.tas_to_cas(5000, tas)

# CAS ↔ Mach
M = isa.cas_to_mach(150)
cas3 = isa.mach_to_cas(M)

# Moist‑air Mach
M_moist = isa.mach_moist(5000, V=250, rh=0.8)
```

### Speed of Sound and Mach Number

```python
T, p, rho = isa.atm(11000)
a = isa.speed_of_sound(11000)
M = isa.mach(11000, V=250)
```

### Density Altitude

```python
da = isa.density_altitude(1500)
```

### Humidity and Dew Point

```python
e = isa.vapor_pressure(0, rh=0.6)
td = isa.dew_point(0, rh=0.6)
```

### Wind Profile

```python
u = isa.wind_loglaw(z=50, z_ref=10, u_ref=5, z0=0.1)
```

### Aerodynamic Quantities

```python
mu = isa.dynamic_viscosity(10000)
nu = isa.kinematic_viscosity(10000)
Re = isa.reynolds(10000, V=200, L=1.0)
T0, p0, s0 = isa.stagnation(10000, V=250)
beta = isa.compressibility(10000, V=250)
tas = isa.eas_to_tas(10000, eas=120)
```

---

## Background

Pyaisa originated from work in the AeroPython group, where the ISA model was used for numerical methods and open‑source workflows.  
This repository continues that effort with a Rust backend and an extended set of atmospheric utilities.
