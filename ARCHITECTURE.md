# meta-signal-message - architecture

`meta-signal-message` is the meta-only wire contract for privileged
`message-daemon` configuration. It is the authority/configuration companion to
the ordinary `signal-message` contract.

## Direction

This repo is the second leg of the message contract pair. Every Persona
component has exactly two contracts: the ordinary `signal-<component>` and the
meta `meta-signal-<component>`. `meta-signal-message` is the authority surface
that configures the `message-daemon`, including the ingress socket mode and the
engine-owner origin policy the stamp-and-forward boundary binds; before it,
`message` had only its ordinary contract.

Engine-owner registration and ingress origin policy are daemon configuration, so
they live inside the `Configure` payload rather than as bespoke operations.
Reconfiguration arrives over the meta plane as the same typed record, never as
flags.

## Surface

The crate defines the meta channel that lets the owning authority configure a
message daemon. The baseline operation is `Configure`, carrying
`signal_message::MessageDaemonConfiguration`, the same typed startup record the
daemon decodes from its binary startup file.

```text
MetaMessageOperation                        MetaMessageReply
└─ Configure(MessageDaemonConfiguration)    ├─ Configured(ConfigurationGeneration)
                                            ├─ ConfigurationRejected(reason)
                                            └─ RequestUnimplemented
```

## Owned

- Meta authority wire vocabulary for message.
- The `Configure(MessageDaemonConfiguration)` operation.
- Configuration replies: `Configured` (carries the applied
  `ConfigurationGeneration`), `ConfigurationRejected` (typed reason:
  `ManagerAuthorityRequired`, `MalformedConfiguration`,
  `UnsupportedConfiguration`), and `RequestUnimplemented` (typed
  `NotBuiltYet` / `DependencyNotReady` reason).
- Optional NOTA projection behind the `nota-text` feature.

## Not Owned

- Ordinary message submission and inbox traffic lives in `signal-message`.
- The shared `MessageDaemonConfiguration` record lives in `signal-message`; this
  contract imports it and exposes the owner/meta authority verb.
- Message daemon state, sockets, actors, the stamp-and-forward boundary, and
  storage live in `message`.
- Schema generation machinery lives in `schema-rust-next`.

## Code Map

- `schema/lib.schema` is the source of the meta wire vocabulary; it cross-imports
  `MessageDaemonConfiguration` from `signal-message` with the single-colon path
  form so startup and meta reconfiguration share one type identity.
- `build.rs` runs `schema-rust-next` against the dependency schema and checks the
  checked-in artifacts for freshness.
- `src/schema/lib.rs` is the checked-in generated artifact.
- `src/lib.rs` re-exports the generated nouns and keeps only tiny handwritten
  accessors and component aliases (`MetaMessageOperation`, `MetaMessageReply`).
- `tests/round_trip.rs` proves the meta channel round-trips through signal
  frames; `tests/canonical_examples.rs` exercises the NOTA projection under
  `nota-text`.
- `Cargo.toml` keeps `nota-text` optional and pins the rkyv feature set.
- `flake.nix` builds, tests, and checks the contract, including the
  `nota-text` canonical-examples path.

## Invariants

- The crate is wire-only: no daemon runtime, no actors, no storage, no tokio.
- Default builds are NOTA-free; `nota-text` is the explicit text-codec opt-in.
- The meta contract reuses `signal_message::MessageDaemonConfiguration`; it does
  not mirror the daemon configuration record.
- The implementation is schema-derived `WireContract`; there is no parallel
  handwritten channel surface.
