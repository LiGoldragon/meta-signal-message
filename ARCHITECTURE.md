# ARCHITECTURE — meta-signal-message

`meta-signal-message` is the meta relation for Message daemon configuration.
The ordinary contract owns the configuration type; this contract imports that
identity rather than redeclaring it.

## Source

The sole authored source is `ethos/signal.ethos`, a `Signal` root. Its import
header names `signal_message:[ MessageDaemonConfiguration ]`.

- Request: `Configure` carrying `MessageDaemonConfiguration`.
- Replies, in wire order: `ConfigurationRefused`, `ConfigurationApplied`,
  `OperationUnimplemented`.
- Local types: the rejection and unimplemented reason vocabularies, the
  configuration generation, and `RequestUnimplemented`.

`build.rs` generates from that source with `ethos-zero` and asserts equality
with the committed `src/generated/signal.rs`. Nothing in this repository
hand-writes a contract type or a codec implementation.

## Wire

Values travel as bare rkyv archives of `Query` and `Response`, framed by the
shared portable `Signal<T>` imported from `signal`. There is no envelope: each connection carries one request and one reply, so exchange
identity, lane and batch would be ceremony over a wire that never used them.
The Datom text projection is the same values rendered by `datom-codec`.

## Boundaries

This repository owns the meta relation's vocabulary. It owns no daemon
runtime, authentication mechanism, socket, actor, storage, process supervision,
or ordinary message traffic.

## Proof surfaces

- `tests/contract.rs` round-trips every request and reply root through
  received bytes, rejects malformed archives, and actualizes every line of
  `examples/canonical.datom`, asserting each re-renders to identical text.
- `build.rs` gates the committed generation against a fresh one.
