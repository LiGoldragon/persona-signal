# skills — signal-persona

Work from the encoded Interface transaction outward.

- Edit `ethos/signal.ethos` for structural Type changes.
- Mint and record new identities explicitly in `src/bootstrap_manifest.rs`.
- Keep request/reply role behavior in `src/schema/lib/behavior.rs` during this
  bootstrap stage.
- Regenerate the checked source and Rust projections with
  `SIGNAL_PERSONA_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features`.
- Use encoded Rust coordinates in assembly code. Human-visible names belong to
  Ethos metadata and Dotos projection.
- Run the repository Cargo suite and exact Nix flake check before publication.

The ordinary channel owns lifecycle traffic. Privileged engine policy belongs
to `meta-signal-persona`.
