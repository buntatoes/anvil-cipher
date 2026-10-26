<div align="center">
  <h1>anvil-cipher</h1>
  <p>AES-128, AES-256, CTR, AES-256-GCM, SHA-256, HMAC-SHA256, and HKDF-SHA256.</p>
  <p>
    <img src="https://img.shields.io/badge/release-v.02-e8c37a?style=flat-square" alt="v.02" />
    <img src="https://img.shields.io/badge/status-Proof_of_Concept-d4a359?style=flat-square" alt="Proof of Concept" />
    <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square" alt="MPL-2.0" /></a>
    <a href="./SECURITY.md"><img src="https://img.shields.io/badge/security-policy-2ea44f?style=flat-square" alt="Security" /></a>
    <img src="https://img.shields.io/badge/rust-2024-orange?style=flat-square" alt="Rust 2024" />
  </p>
</div>

Version 0.2.0.

The browser uses AES-256-GCM for the Linux profile and SHA-256 / HKDF-SHA256 (`hkdf_extract`, `hkdf_expand_label`) for TLS 1.3 hybrid key-schedule derivation with `anvil-pqc`. [anvil-video](https://github.com/buntatoes/anvil-video) uses AES-128-CTR for encrypted samples. HTTPS socket handshakes use the system OpenSSL on Linux and Schannel on Windows.

## What it does

- `aes128_encrypt_block` and `aes256_encrypt_block` encrypt one 16-byte block.
- `aes128_ctr` takes a 16-byte initial counter and increments it as a big-endian integer. Encrypt and decrypt are the same call.
- `aes256_gcm_seal` and `aes256_gcm_open` take a 12-byte IV. A bad tag returns an error and no plaintext.
- `sha256`, `hmac_sha256`, `hkdf_extract`, `hkdf_expand`, and `hkdf_expand_label` implement FIPS 180-4 SHA-256, RFC 2104 HMAC-SHA256, RFC 5869 HKDF-SHA256, and RFC 8446 Section 7.1 TLS 1.3 `HKDF-Expand-Label`.

## What it does not do

- It does not do TLS record framing, X.509 certificates, or OS key storage.
- It does not link OpenSSL.

## Build

```
cargo test
```

Rust 2024 edition, Rust 1.99.1 or newer. No crates.io dependencies.

## The other repositories

The browser is [anvil-browser](https://github.com/buntatoes/anvil-browser) (Apache-2.0). The page engine is [anvil-engine](https://github.com/buntatoes/anvil-engine). Its source is public to read and audit, but it's proprietary, not MPL-2.0, and can't be redistributed on its own. These nine libraries sit next to both, and all of them are MPL-2.0. None of them come from crates.io.

| Repository | What it is |
| --- | --- |
| [anvil-cipher](https://github.com/buntatoes/anvil-cipher) | AES-128, AES-256, CTR, AES-256-GCM, SHA-256, HMAC-SHA256, HKDF-SHA256 |
| [anvil-compress](https://github.com/buntatoes/anvil-compress) | DEFLATE, zlib, gzip |
| [anvil-hpack](https://github.com/buntatoes/anvil-hpack) | HPACK header compression for HTTP/2 |
| [anvil-image](https://github.com/buntatoes/anvil-image) | PNG, JPEG, GIF, WebP (`VP8L`), ICO, BMP, PPM |
| [anvil-json](https://github.com/buntatoes/anvil-json) | JSON |
| [anvil-pqc](https://github.com/buntatoes/anvil-pqc) | ML-KEM-768, X25519, X25519MLKEM768, SHA-3 |
| [anvil-ttf](https://github.com/buntatoes/anvil-ttf) | TrueType and OpenType (`glyf`, `CFF `, `kern`, `GPOS`) |
| [anvil-gpu](https://github.com/buntatoes/anvil-gpu) | Compositor passes for boxes, shadows, gradients, and NV12, on the CPU or a GPU |
| [anvil-video](https://github.com/buntatoes/anvil-video) | CPU frames and PCM/ADPCM audio in a small MP4 container |

## License

MPL-2.0. Copyright (c) 2026 Buntato.
