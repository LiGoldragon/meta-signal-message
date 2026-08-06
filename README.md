# meta-signal-message

The owner Message configuration Interface. It imports
`MessageDaemonConfiguration` from the ordinary `signal-message` producer by
identity and adds the owner-only Configure request and its three reply shapes.

`ethos/interface.ethos` is the sole authored Interface projection. The build
resolves the exact producer source published by `signal-message`, assembles the
owner Interface under its recorded authority, and freshness-checks the strict
encoded Rust projection. Dotos remains optional and presents the readable
Configure, ConfigurationApplied, ConfigurationRefused, and
OperationUnimplemented names.

Run `nix --option substituters https://cache.nixos.org flake check
--print-build-logs` for the complete proof matrix.
