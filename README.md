# flux-cipher

AES-128, AES-256, CTR, and AES-256-GCM. Version 0.1.0.

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
