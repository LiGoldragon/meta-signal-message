# ARCHITECTURE — meta-signal-message

`meta-signal-message` is the owner authority relation for Message daemon
configuration. The ordinary producer owns the configuration Type; this
Interface imports that same opaque identity so startup and live configuration
cannot drift into parallel records.

## Interface

The sole authored source is `ethos/interface.ethos`, a role-free
`Interface.{1 0 0}` document. Its import header names
`signal_message:lib.MessageDaemonConfiguration`. `build.rs` resolves the
producer-published Ethos directory, verifies that its text is exactly the
source compiled into the pinned producer dependency, and seats the imported
producer identity in the local catalog.

The local Types are ConfigurationGeneration, typed rejection and
unimplemented reasons, and the strict `OwnerRequest` / `OwnerReply` roots.
Their Rust coordinates and variants are encoded. Dotos uses the authority's
textual metadata to retain the domain spellings.

## Current behavior slice

Archive behavior, Dotos behavior, owner role routing, and the Signal frame
binding are handwritten in `src/schema/lib/behavior.rs` until Logos expresses
that slice. The request role has one route, Configure. The reply role has
ConfigurationApplied, ConfigurationRefused, and OperationUnimplemented. The
allocated frame contract is ID 2 at wire revision 2.

## Boundaries

This repository owns the owner relation vocabulary and frame legality. It owns
no daemon runtime, authentication mechanism, sockets, actors, storage, process
supervision, or ordinary message traffic.

## Proof surfaces

- `tests/interface_contract.rs` proves the exact producer identity import,
  empty Interface role lists, and strict local/imported Rust coordinates.
- `tests/round_trip.rs` proves every request and reply route through frame
  bytes.
- `tests/canonical_examples.rs` proves readable Dotos examples.
- `tests/dependency_boundary.rs` proves the corrected generator and runtime
  boundary, and fences historical source machinery at exact zero.
