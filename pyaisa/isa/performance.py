from pyaisa.pyaisa_core import (
    climb_rate,
    drag_force,
    drag_polar,
    service_ceiling,
    thrust_lapse,
)


class Performance:
    def drag_polar(self, cd0: float, k: float, cl: float) -> float:
        return drag_polar(cd0, k, cl)

    def thrust_lapse(self, thrust_sl: float, T: float, p: float) -> float:
        return thrust_lapse(thrust_sl, T, p)

    def drag_force(self, q: float, S: float, CD: float) -> float:
        return drag_force(q, S, CD)

    def climb_rate(self, thrust: float, drag: float, V: float, weight: float) -> float:
        return climb_rate(thrust, drag, V, weight)

    def service_ceiling(self, rc: float) -> bool:
        return service_ceiling(rc)
