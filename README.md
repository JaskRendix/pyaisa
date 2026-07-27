# Pyaisa

Pyaisa is a compact implementation of the International Standard Atmosphere (ISA).  
It provides temperature, pressure, and density as functions of altitude, following the COESA reference model.  
The project began as an exercise in scientific programming and collaborative development.

The original version used Python with a C++ OpenMP backend accessed through SWIG.  
The current version replaces that backend with a Rust implementation exposed through PyO3, improving clarity, performance, and extensibility.

Repository: [https://github.com/newlawrence/Pyaisa](https://github.com/newlawrence/Pyaisa)

---

## Features

### Core ISA Model
- COESA‑based standard atmosphere  
- Scalar or vector altitude inputs  
- NumPy array outputs  
- Configurable parameters (`R`, `g`, `T0`, `p0`, layer structure)  
- Python wrapper around a Rust core  
- Simple API: `ISA` object and `build_atm` function  

### Extended Atmosphere Physics
Derived quantities used in aeronautics and atmospheric science:

- Speed of sound  
- Dynamic pressure  
- Mach number  
- Pressure altitude  
- Density altitude  

### Aerodynamic and Compressible‑Flow Physics
Additional ISA‑derived quantities for performance analysis:

- Dynamic viscosity (Sutherland’s law)  
- Kinematic viscosity  
- Reynolds number  
- Stagnation temperature  
- Stagnation pressure  
- Stagnation entropy  
- Prandtl–Glauert compressibility correction  
- Airspeed conversions (CAS → EAS, EAS ↔ TAS, Mach from TAS)

All functions are implemented in Rust and exposed directly to Python.

### Humidity and Thermodynamics
- Saturation vapor pressure  
- Vapor pressure  
- Mixing ratio  
- Dew point temperature  
- Virtual temperature  
- Potential temperature  
- Moist adiabatic lapse rate  
- Wet‑bulb temperature  

### Wind Models
- Log‑law wind profile  
- Power‑law wind profile  
- Displaced log‑law  
- Linear shear profile  
- Ekman‑type rotation  
- Gust factor  

### Flight‑Level Conversions
- Pressure altitude to flight level  
- Flight level to pressure altitude  
- Geometric altitude to flight level  
- Indicated altitude with QNH correction  

### Icing Conditions
- Liquid water content  
- Supercooled fraction  
- Icing severity index  
- Freezing fraction  

---

## New Capabilities

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

### ISA Deviation (ΔT, Δp, Δρ)

```python
isa.atm_deviation(h, dT=10)
isa.atm_deviation(h, dp=-500)
```

### Layer Introspection

```python
isa.layer_at(15000)
```

### Aerodynamic Quantities

```python
mu, nu = isa.viscosity(h)
Re = isa.reynolds(h, V, L)
T0, p0, s0 = isa.stagnation(h, V)
beta = isa.compressibility(h, V)
tas = isa.eas_to_tas(h, eas)
eas = isa.tas_to_eas(h, tas)
M = isa.mach_from_tas(h, tas)
```

---

## Installation

Editable mode with tests:

```
pip install -e .[test]
```

Standard installation:

```
pip install .
```

---

## Testing

```
pytest
```

---

## Basic usage

```python
import pyaisa

isa = pyaisa.ISA()
isa.atm(0)
isa.atm([0, 11000])
```

### Modifying parameters

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

### Temperature offset

```python
pyaisa.atm(0, 15)
pyaisa.atm(11000, 15)
```

---

## Extended physics examples

### Speed of sound and Mach number

```python
T, p, rho = isa.atm(11000)
a = isa.speed_of_sound(11000)
M = isa.mach(11000, V=250)
```

### Density altitude

```python
da = isa.density_altitude(1500)
```

### Humidity and dew point

```python
e = isa.vapor_pressure(0, rh=0.6)
td = isa.dew_point(0, rh=0.6)
```

### Wind profile

```python
u = isa.wind(z=50, z_ref=10, u_ref=5, z0=0.1)
```

### Aerodynamic quantities

```python
mu, nu = isa.viscosity(10000)
Re = isa.reynolds(10000, V=200, L=1.0)
T0, p0, s0 = isa.stagnation(10000, V=250)
beta = isa.compressibility(10000, V=250)
tas = isa.eas_to_tas(10000, eas=120)
```

---

## Background

Pyaisa originated from work done in the AeroPython group, where the ISA model served as an example for numerical methods and open‑source workflows.  
This repository continues that effort and keeps the model readable, configurable, and compact, with a Rust backend and an extended set of atmospheric utilities.
