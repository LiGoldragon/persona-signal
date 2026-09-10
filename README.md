# signal-persona

The ordinary lifecycle Interface for Persona. Together,
`signal-persona` and `meta-signal-persona` form Persona's working and privileged
contract pair.

The Interface carries component announcement, readiness and health queries,
graceful stop, and the typed spawn envelope. Its canonical component-owned
source is `ethos/signal.ethos`; its generated Rust Types use encoded
identities, while Dotos renders human-facing names. Lifecycle request/reply
roles remain handwritten during the current bootstrap stage.

Regenerate the checked source and Rust projection with:

```console
SIGNAL_PERSONA_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features
```

Persona owns process supervision, sockets, persistence, and privileged policy.
