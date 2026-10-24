# flux-cipher

AES-128, AES-256, CTR, and AES-256-GCM. No crates.io dependencies.

Flux Browser uses this for the Linux profile (`AES-256-GCM`) and for encrypted video samples (`AES-128-CTR`). HTTPS on Linux still uses the system OpenSSL. Windows still uses Schannel and DPAPI.

Put this directory next to the other Flux repositories:

```text
../flux-cipher
../flux-browser
../flux-video
```

```
cargo test
```

MPL-2.0. Copyright (c) 2026 Buntos.
