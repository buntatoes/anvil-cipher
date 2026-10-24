# Security

Report problems in private:

https://github.com/buntatoes/flux-cipher/security/advisories/new

The browser repository takes the same kind of report:

https://github.com/buntatoes/flux-browser/security/advisories/new

Do not put the details in a public issue.

## What is checked

- AES-256-GCM open fails when the tag does not match, including an empty message.
- The tag compare does not stop at the first differing byte.

This crate does not store keys and does not talk to the operating system.

Version 0.2.0 is current.
