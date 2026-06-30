# meta-signal-message

Meta signal contract for privileged message daemon configuration.

The meta-only wire contract for `message` — the second leg of the two-contract
pair (`signal-message` ordinary + `meta-signal-message` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `message`'s `*DaemonConfiguration` (the same record that is the daemon's
binary startup message), with `Configured` / `ConfigurationRejected` /
`RequestUnimplemented` replies.

Default builds stay binary/rkyv-only; enable `nota-text` for CLI/debug
projection. See `ARCHITECTURE.md`.
