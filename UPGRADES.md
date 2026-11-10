## 4.0.1: signal-message 2999252

signal-message moves to 2999252, which pins flow-ethos 5c61d666 (0.54.0) and
signal-flow 15790be. The contract and every datom form are unchanged.

# UPGRADES

## 3.0.1 → 4.0.0 — signal-message 22, ethos-zero 22, on build-base 404cf8d

Pins signal-message 22.0.0 (`2fed750`), signal 11.0.0 (`6d4eb74`),
datom-codec `09a0526`, protos `c93b598` and generator ethos-zero 22.0.0
(`1976e45`); signal-flow `6881724` and flow-ethos `d1effd4b` come through
signal-message. The authored Ethos, the generated contract, its digest and
every datom text form are unchanged.

- Configuration's fields are signal-message 22's newtypes `Ordinary`, `Meta`
  and `Flow` over String: a consumer wraps a value with the constructor and
  reads it with `.0`.
- The flake is the build-base form with a committed Cargo.nix; the third-party
  crates sit at build-base's version set.

## 3.0.0 → 3.0.1 — signal-message 21.0.1

Pins signal-message 21.0.1 (`998c824`), whose imported Flow types come from
signal-flow 57.0.0 (`3d0a4cd`). The meta contract and its wire are
unchanged. Repin Message with the ordinary contract in one dependency graph.

## 2.0.0 → 3.0.0 — signal-message 21

Pins signal-message 21.0.0, whose imported Flow types come from signal-flow
56.0.0. The meta contract remains Configure-only and its authored wire is
unchanged. Repin Message with the ordinary contract in one dependency graph.

## 1.0.0 → 2.0.0 — signal-message 20.0.0 and the 0.33 codec stack

Repins signal-message `2c5a007` (Flow `9f159d9` vocabulary: `Address` carries a
`Model`; see its UPGRADES), signal `7afb68b`, datom-codec `eabead4`, protos
`3c7ade3` and generator ethos-zero `5a5daca`. The authored Ethos and the archive
layout of this contract are unchanged; the projection regenerates with one added
`clippy::type_complexity` allowance. Repin `message` to this release together
with signal-message 20.0.0.

## 0.17.0 → 0.17.1 — simplified Flow peer

Pins signal-message 18.0.1, whose imported Message wire types are
unchanged. The meta contract remains Configure-only. Repin and rebuild
Message alongside Flow 0.39.0.

## 0.16.0 → 0.17.0 — signal-message 18.0.0

Pins signal-message 18.0.0 (`8db91d8`), which adds the accepted-send and
resumable delivery-observation wire types. This meta contract remains
Configure-only; repin Message with it.

## 0.15.0 → 0.16.0 — signal-message 17.0.0

Pins signal-message 17.0.0 (`10e0901`), which adds a typed oversized Send
refusal. The meta wire is unchanged; repin Message with it.

## 0.14.0 → 0.15.0 — signal-message 16.0.0

Pins signal-message 16.0.0 (signal-flow 25.0.0). The wire is unchanged.
Repin `message` with it.

## 0.11.0 → 0.12.0 — configuration only

The Meta Signal now carries only `Configure`. Privileged `Send`,
`Redeliver`, receipt, and meta-refusal variants are removed. Send uses
the ordinary typed Message contract; old peers cannot greet this one.

Repin `signal-message` 12.0.0 and `signal` e0e3c055 together. Rebuild
Message and restart it with the matching Flow and Signal packages.

## 0.9.0 → 0.10.0 — signal 8.0.0, signal-message 10.0.0, signal-flow 10.0.0, meta-signal-flow 14.0.0

### What breaks

- The contract depends on `signal` 8.0.0 (f35460de), `signal-message`
  10.0.0 (b94d907c), `signal-flow` 10.0.0 (f95034de) and `meta-signal-flow`
  14.0.0 (54eb5618). The imported types and the `signal::Contracted` that
  `Query` implements are now those releases'; a crate holding the older
  ones alongside sees two sets.
- The `datom` feature also enables `signal/datom`: signal 8.0.0 binds the
  same datom-codec and protos 0.32.2, so the graph holds one codec.
- The build reads the ethos with ethos-zero 16.0.0 at c2653dd8. The
  generated module is byte-identical (the build script asserts it), so
  `ETHOS`, the contract digest and every archive are unchanged from 0.9.0.

### Deploy

In message: repin meta-signal-message with the four contracts above and
signal 8.0.0 in one change. The wire is unchanged; a 0.9.0 peer still
greets this one.

## 0.8.0 → 0.9.0 — ethos-zero 16.0.0, signal 7.0.0, protos and datom-codec 0.32.2

### What breaks

- Repins `signal` 7.0.0 (66e7b153), `signal-message` 9.0.0 (0f5c0f0),
  `signal-flow` 8.0.0 (c297d987) and `meta-signal-flow` 12.0.0 (69f9c146).
  The re-exported frame types are signal 7.0.0's; the imported Send shapes
  are signal-message 9.0.0's.
- `Query` implements `signal::Contracted` over `ETHOS`; the meta socket's
  wire identity is the digest of `ethos/signal.ethos`, which differs from
  the ordinary contract's, so an ordinary greeting is refused here.
- The `datom` feature pins protos and datom-codec 0.32.2 (15b41da8,
  4dff16b4) and no longer enables `signal/datom`.
- One-line datom text is `Compactable::compact`; protos 0.32's `textualize`
  prints vertically.
- Build dependency ethos-zero 16.0.0 (0edfc0c3). The ethos source and the
  generated Rust are unchanged.

### Deploying

No runtime here. Repin message's `meta-signal-message` with
`signal-message` 9.0.0 in the same pass (one `signal` per graph), rebuild
the Message Nexus and both CLIs, and restart them together with flow 0.19.0.

## 0.7.1 → 0.8.0 — a refusal names a Retired or Exited flow

Repins `meta-signal-flow` 11.0.0 (2ac045c) and `signal-message` 8.0.0
(d574200), whose `DeliveryRejection` gains `FlowRetired` and `FlowExited`.
Nothing of this contract's own vocabulary changes; a reader of the owner's
Send reply pinned to the older contracts cannot decode the new refusals.

## 0.6.0 → 0.7.0 — configuration applied; the owner's Send; Redeliver

`Configure` now carries `MessageConfiguration.{ OrdinarySocketPath
MetaSocketPath FlowSocketPath FlowMetaSocketPath MetaAspects }` and answers
`Configured.{ MessageConfiguration Activation }` or `ConfigureRejected`.
`Send` (stamped Owner) and `Redeliver` are added; `MetaRefused` answers a peer
the gate refuses. `ConfigurationApplied`, `ConfigurationRefused` and
`OperationUnimplemented` are gone, and `MessageDaemonConfiguration` is no
longer imported. Depends on `signal-message` 6.0.0, `signal-flow` 7.0.0 and
`meta-signal-flow` 9.0.0.


## 0.3.1 → 0.6.0 — rewritten onto the Datom stack

0.3.1 was not on the Datom stack at all: it was a `schema-rust` Interface
projection over `signal-frame`, `dotos`, `sema-translator`, `core-ethos`,
`name-table` and `rust-logos`, whose Rust type names were name-table hashes.

### What breaks

1. **Every type name.** The contract is now generated by `ethos-zero` from
   `ethos/signal.ethos` and carries its real names. The vocabulary and the
   position of every field and variant are preserved from 0.3.1; only the
   projection changed.
2. **The wire.** The `signal-frame` envelope is gone. A value travels as the
   bare rkyv archive of `Query` or `Response`.
3. **`links = "meta-signal-message"` is dropped.** Nothing needed the
   single-copy key, and a `links` key fails at *resolution*, masking every
   compile error behind it.
4. **The `dotos-text` feature is gone**, replaced by `datom`.
5. **Newtypes collapsed to aliases.** `Configured`, `Generation`,
   `ConfigurationGeneration`, `Reason` and `UnimplementedOperationKind` are
   plain type aliases, so `::new(..)` and `.payload()` are gone. Note the
   consequence for Datom text: `ConfigurationApplied.3`, not
   `ConfigurationApplied.{ 3 }`, because `Configured` is an alias to an
   integer and not a single-field struct.
6. **`EngineRefusal` and `EngineRefusalReason` are deleted.** They were
   hand-written, outside the ethos, and had no consumer.
7. **The portable frame comes from `signal` 5.0.0**, not from this crate, and
   the generated projection derives `Composing` rather than `Compositional`.

### Deploying

Nothing to deploy: this crate is a contract with no runtime. Its only
consumer is `message`, which moves with it. Pin `signal` as
`https://github.com/LiGoldragon/signal` with no `.git` suffix — `signal`
declares `links = "signal"`, and cargo source identity is the pin string
rather than the commit.
