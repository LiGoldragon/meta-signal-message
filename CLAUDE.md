## Stack

Datom. `protos`, `datom-codec`, `ethos-zero`, `rkyv`, and the ordinary
`signal-message` contract. No `dotos`, no `signal-frame`, no `schema-rust`.

## Rules

`ethos/signal.ethos` is the only place a contract type is declared.
`src/generated/signal.rs` is generated output, committed and gated by
`build.rs`; never edit it by hand.

Every line of `examples/canonical.datom` is produced by the codec that reads
it back. Never spell a canonical line by hand.
