# meta-signal-message

The privileged Message Nexus Signal contract, spoken on Message's meta socket
by the owner and by flows whose aspect is in `MetaAspects`.

- `Configure.MessageConfiguration` — stored and applied: the Flow socket paths
  and `MetaAspects` take effect on the next call (`Applied`); a changed own
  socket path takes effect on restart (`NexusRestartRequired`).
- `Send.SendRequest` — the owner's Send, stamped `Owner`.
- `Redeliver.{ MessageId FlowId }` — the only way out of `Uncertain`.
- `MetaRefused.MetaRefusal` answers any other peer.

`SendRequest`, `Submission`, `SendRejection`, `Receipt` and `MessageRejection`
are imported from `signal-message` by identity.

`ethos/signal.ethos` is the sole authored source; `build.rs` asserts the
committed `src/generated/signal.rs` equals its generation. `tests/contract.rs`
round-trips a concrete datom of every record kind.

Run `nix flake check -L` for the complete proof matrix.
