<div align="center">
  <h1>flux-cipher</h1>
  <p>AES-128, AES-256, CTR, and AES-256-GCM.</p>
  <p>
    <img src="https://img.shields.io/badge/release-v.01-e8c37a?style=flat-square" alt="v.01" />
    <img src="https://img.shields.io/badge/status-Proof_of_Concept-d4a359?style=flat-square" alt="Proof of Concept" />
    <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square" alt="MPL-2.0" /></a>
    <a href="./SECURITY.md"><img src="https://img.shields.io/badge/security-policy-2ea44f?style=flat-square" alt="Security" /></a>
    <img src="https://img.shields.io/badge/rust-2024-orange?style=flat-square" alt="Rust 2024" />
  </p>
</div>

Version 0.1.0.

The browser uses AES-256-GCM for the Linux profile. [flux-video](https://github.com/buntatoes/flux-video) uses AES-128-CTR for encrypted samples. HTTPS on Linux still uses the system OpenSSL. Windows still uses Schannel and DPAPI.

## What it does

- `aes128_encrypt_block` and `aes256_encrypt_block` encrypt one 16-byte block.
- `aes128_ctr` takes a 16-byte initial counter and increments it as a big-endian integer. Encrypt and decrypt are the same call.
- `aes256_gcm_seal` and `aes256_gcm_open` take a 12-byte IV. A bad tag returns an error and no plaintext.

## What it does not do

- It does not do TLS, certificates, or key storage.
- It does not link OpenSSL.

## Build

```
cargo test
```

Rust 2024. No crates.io dependencies.

## The other repositories

The browser is [flux-browser](https://github.com/buntatoes/flux-browser). These libraries sit next to it. None of them come from crates.io.

| Repository | What it is |
| --- | --- |
| [flux-cipher](https://github.com/buntatoes/flux-cipher) | AES-128, AES-256, CTR, AES-256-GCM |
| [flux-compress](https://github.com/buntatoes/flux-compress) | DEFLATE, zlib, gzip |
| [flux-image](https://github.com/buntatoes/flux-image) | PNG, JPEG, GIF, WebP, ICO, BMP, PPM |
| [flux-json](https://github.com/buntatoes/flux-json) | JSON |
| [flux-ttf](https://github.com/buntatoes/flux-ttf) | TrueType and OpenType |
| [flux-gpu](https://github.com/buntatoes/flux-gpu) | CPU drawing of boxes, shadows, and NV12 |
| [flux-video](https://github.com/buntatoes/flux-video) | CPU frames in a small MP4 container |

## License

MPL-2.0. Copyright (c) 2026 Buntos.
