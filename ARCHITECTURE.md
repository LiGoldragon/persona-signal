# signal-persona — Architecture

`signal-persona` owns the ordinary Persona lifecycle Interface: announce,
readiness, health, graceful stop, and the typed spawn envelope used between the
engine manager and supervised components.

## Structural center

`ethos/signal.ethos` is the component-owned textual projection of one
authority-approved encoded Interface transaction. `src/bootstrap_manifest.rs`
holds its already-minted identity seats and canonical order. The build boundary
assembles that exact transaction, revalidates it through Core Nomos, lowers it
to Whole Logos, and asks Rust Logos for the checked Rust projection in
`src/generated/signal.rs`.

```text
ethos/signal.ethos + bootstrap_manifest
                    │
                    ▼
       verified Interface transaction
                    │
                    ▼
              Whole Logos
                    │
                    ▼
   encoded Rust Types in generated.rs
```

The generated Rust surface uses encoded identities. Dotos supplies the human
textual names carried by the Interface metadata.

## Current-stage behavior

The bootstrap Interface describes Types. `src/schema/lib/behavior.rs`
handwrites the lifecycle request/reply role seating, structural runtime traits,
and Signal frame boundary. That separation is deliberate: executable role
behavior gains its schema home when the language train reaches the Logos
behavior slice.

The ordinary and privileged Persona channels remain a pair:

| Repository | Responsibility |
|---|---|
| `signal-persona` | supervised-component lifecycle traffic |
| `meta-signal-persona` | engine-owner policy operations |

## Code map

- `ethos/signal.ethos` — canonical role-free Interface text.
- `src/bootstrap_manifest.rs` — explicit authority identity state.
- `build.rs` — verified generation and `ethos-source-dir` publication.
- `src/generated/signal.rs` — checked encoded Rust Types projection.
- `src/schema/lib/behavior.rs` — handwritten lifecycle roles and wire behavior.
- `tests/interface_contract.rs` — strict Interface/projection witness.
- `tests/dependency_boundary.rs` — dependency and historical-surface fence.
- `tests/lifecycle_roles.rs` — handwritten role seating witness.
- `tests/round_trip.rs` — Signal frame and Dotos role round trips.
- `tests/spawn_envelope.rs` — standalone archive and Dotos envelope witness.

## Boundaries

The crate carries typed Interface data and current-stage projection behavior.
Persona owns supervision, process spawning, persistence, sockets, and policy.
Request payloads carry domain data; transport infrastructure owns caller
identity, timestamps, and authorization proof.
