# Artificial Culture

Artificial Culture (AC) is the cognition and social-simulation layer for non-player characters in **Gala of the Hidden**. The active implementation is being migrated to **C++20 and Flecs** so it can share the game's native ECS world and production toolchain.

## Current status

The C++ implementation is an integration foundation, not a complete port of the former simulation. It currently models normalized needs, legacy need-decay rates, perception snapshots, prioritized action proposals, and confirmed action results. The prior Rust/Bevy implementation and tests are preserved under [`legacy/rust-bevy/`](./legacy/rust-bevy/) as migration reference.

The active module does not own movement, collision, resource inventory, or world state. It consumes observations supplied by the authoritative GotH server, proposes an action intent, and updates cognition from the server's confirmed result. Rumor propagation, personality, memory, trust, and the remaining legacy behaviors still need to be ported.

## Build

Requirements: CMake 3.24+, a C++20 compiler, Git, and network access on the first configure (CMake fetches Flecs v4.1.6).

```powershell
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
```

Run the local adapter demonstration:

```powershell
./build/ac_demo
```

On multi-configuration generators such as Visual Studio, the executable is usually under `build/Release/`.

## Integration contract

`CognitionModule` receives the GotH `flecs::world`, registers its cognition components, and accepts observations through `observe`. The game calls `step(delta_seconds)`, drains the outbox with `take_intents`, validates and executes those proposals in the authoritative world, then reports each outcome with `report_result`. The demo adapter in `src/main.cpp` illustrates this boundary; it is not the live game integration.

See [`Docs/ARCHITECTURE.md`](./Docs/ARCHITECTURE.md) for the component model, authority boundary, migration phases, and known gaps. The historical design and research files remain under [`Docs/`](./Docs/).

## License

The repository currently carries CC BY-NC-SA 4.0. Review the license and obtain any required commercial permission before incorporating derived work into a commercial release.
