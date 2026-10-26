# Security

Report problems in private:

https://github.com/buntatoes/anvil-cipher/security/advisories/new

The browser repository takes the same kind of report:

https://github.com/buntatoes/anvil-browser/security/advisories/new

Do not put the details in a public issue.

## What is checked

- AES-256-GCM open fails when the tag does not match, including an empty message.
- The tag compare does not stop at the first differing byte.
- `sha256`, `hmac_sha256`, and `hkdf_expand_label` are deterministic and validated against NIST FIPS 180-4, RFC 2104, RFC 5869, and RFC 8446 test vectors.

This crate does not store keys and does not talk to the operating system.

Version 0.2.1 is current.
