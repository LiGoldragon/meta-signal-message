# INTENT — meta-signal-message

*The meta-only wire contract for privileged `message` daemon configuration.
Companion to `Cargo.toml` and the ordinary `signal-message` contract.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is for the `meta-signal-message`
contract. Workspace-shape intent stays in `primary/INTENT.md`; the component
daemon intent stays in `message/INTENT.md`; ordinary message submission and
inbox traffic stays in `signal-message/INTENT.md`.

## Why this repo exists

Every Persona component has exactly two contracts: `signal-<component>`
(ordinary) and `meta-signal-<component>` (meta). `meta-signal-message` is the
second leg for `message` — the authority surface that configures the
`message-daemon`, including the ingress socket mode and engine-owner origin
policy the stamp-and-forward boundary binds. Before this repo, `message` had
only its ordinary contract; this completes the pair.

## The channel shape

The meta plane's baseline content is daemon configuration. The channel carries
a single `Configure` operation whose payload is the typed
`MessageDaemonConfiguration` imported from `signal-message` — the same record
that is the daemon's binary startup message. Reconfiguration arrives over this
meta plane as the same typed record, never as flags.

- **Request:** `Configure(MessageDaemonConfiguration)`.
- **Replies:** `Configured`, `ConfigurationRejected` (typed reason),
  `RequestUnimplemented`.

Engine-owner registration and ingress origin policy are daemon configuration
and so live inside the `Configure` payload rather than as bespoke operations.
