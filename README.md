# meta-signal-message

The meta Message Signal contract: the privileged channel on which a manager
configures the Message daemon.

`MessageDaemonConfiguration` is owned by the ordinary `signal-message`
contract and imported here by identity, so startup configuration and live
reconfiguration cannot drift into parallel records.

`ethos/signal.ethos` is the sole authored source. `build.rs` regenerates it
with `ethos-zero` and asserts the result equals the committed
`src/generated/signal.rs`, so the checked-in projection can never drift from
its source. `src/lib.rs` adds the rkyv `Signal` / `Signalizable` /
`ByteViewable` / `Restorable` frame surface shared by every Signal contract.

The `datom` feature adds the Datom text projection. `examples/canonical.datom`
carries one line per contract head; `tests/contract.rs` actualizes every line
and re-renders it through the codec, so a wrong wire shape cannot sit unread.

Run `nix flake check -L` for the complete proof matrix.
