# Artificial Culture

Artificial Culture (AC) is the cognition and social-simulation layer for non-player characters in **Gala of the Hidden**. The active implementation is being migrated to **C++20 and Flecs** so it can share the game's native ECS world and production toolchain.

## Current status

The C++ implementation is an integration foundation, not a complete port of the former simulation. It currently models normalized needs, Big Five personality traits, rumor beliefs and propagation, observed resource memory, prioritized action proposals, and confirmed action results. The complete Rust/Bevy source and commit history are preserved on the `archive/rust-bevy` branch; the old feature branch tips are retained as archive tags.

The active module does not own movement, collision, resource inventory, or world state. It consumes observations and confirmed social contacts supplied by the authoritative GotH server, remembers resource IDs and their last observed facts, proposes action intents, and updates cognition from the server's confirmed results. Trust, richer memory, learning rules, and the remaining legacy behaviors still need to be ported.

## Build

Requirements: CMake 3.24+, a C++20 compiler, Git, and network access on the first configure (CMake fetches Flecs v4.1.6).

```powershell
cmake --preset windows-msvc-release
cmake --build --preset windows-msvc-release
ctest --preset windows-msvc-release
```

Run the local adapter demonstration:

```powershell
./out/build/windows-msvc-release/Release/ac_demo.exe
```

The Windows preset uses Visual Studio 2026 x64 and writes the executable to `out/build/windows-msvc-release/Release/ac_demo.exe`.

## Integration contract

The versioned host adapter contract is documented in [`Docs/ADAPTER_CONTRACT.md`](./Docs/ADAPTER_CONTRACT.md). `CognitionModule` receives GotH's `flecs::world`, accepts perception and agent-specific event accounts through `observe` and `observe_event`, and exposes confirmed contacts through `report_social_contact`. GotH calls `step(delta_seconds)`, drains intents with `take_intents`, validates and executes them in the authoritative world, reports each outcome with `report_result`, and removes departed NPCs with `remove_agent`. Rumor beliefs remain internal to AC; `take_rumor_events` exposes telemetry. The demo adapter in `src/main.cpp` illustrates this boundary; it is not the live game integration.

Follow the step-by-step path to a usable module in [`Docs/ROADMAP_FLECS.md`](./Docs/ROADMAP_FLECS.md), and check shipped milestones in [`Docs/IMPLEMENTATION_STATUS.md`](./Docs/IMPLEMENTATION_STATUS.md). See [`Docs/ARCHITECTURE.md`](./Docs/ARCHITECTURE.md) for the component model and authority boundary, and [`Docs/CULTURAL_SCENARIOS.md`](./Docs/CULTURAL_SCENARIOS.md) for the social acceptance scenarios. The historical design and research files remain under [`Docs/`](./Docs/).

## License

The repository currently carries CC BY-NC-SA 4.0. Review the license and obtain any required commercial permission before incorporating derived work into a commercial release.
