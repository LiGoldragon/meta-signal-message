## Stack

Datom. `protos`, `datom-codec`, `ethos-zero`, `rkyv`, the shared `signal`
frame layer, and the ordinary `signal-message` contract. No `dotos`, no
`signal-frame`, no `schema-rust`.

The portable frame (`Signal`, `Signalizable`, `ByteViewable`, `Restorable`)
is imported from `signal`, never declared here: a second copy is a different
Rust type, which forks the wire.

## Rules

`ethos/signal.ethos` is the only place a contract type is declared.
`src/generated/signal.rs` is generated output, committed and gated by
`build.rs`; never edit it by hand.

Every line of `examples/canonical.datom` is produced by the codec that reads
it back. Never spell a canonical line by hand.
