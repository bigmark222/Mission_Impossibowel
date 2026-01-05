# Lessons from the Toy Clone

Focusing on a stripped-down `vision_runtime` surfaced a few design laws:

1) **Own or swap, avoid locks when you can**  
   Swapping ownership of the detector into the async task avoided mutexes. The real crate uses a mutex to share the model; for throughput, per-task ownership or per-thread detectors are cleaner.

2) **Keep state small and explicit**  
   Capture state, inference state, and overlay state as simple resources make the data flow obvious. Adding more fields (fallback, thresholds, mode) should stay explicit rather than hidden behind globals.

3) **Debounce/queue as first-class policy**  
   Deciding how many in-flight inference tasks you allow is a policy knob. Making it explicit (debounce interval, max pending) keeps the runtime predictable.

4) **Trait objects at the edge**  
   Using trait objects for the detector keeps the runtime agnostic to implementation (heuristic vs. Burn). Retain this boundary; implementations can be swapped without touching systems.

5) **Bite-size modules map to responsibilities**  
   Three tiny modules (capture, inference, overlay) were enough to cover the core responsibilities. The real crate’s structure matches this and should stay disciplined as features grow.

6) **Observability should be baked in**  
   Even in the toy, capturing latency/confidence in `OverlayState` shows how little state can provide useful telemetry. The real crate should add counters/logging for fallbacks and errors.
