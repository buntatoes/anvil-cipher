// Copyright (c) 2026 Buntos
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! AES-128, AES-256, CTR, and AES-256-GCM.

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const RCON: [u8; 11] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CipherError {
    TagMismatch,
}

impl std::fmt::Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TagMismatch => write!(f, "authentication tag does not match"),
        }
    }
}

impl std::error::Error for CipherError {}

/// AES-128 encrypt one block.
pub fn aes128_encrypt_block(key: &[u8; 16], block: &[u8; 16]) -> [u8; 16] {
    let round_keys = expand_key(key, 4, 10);
    encrypt_block(block, &round_keys, 10)
}

/// AES-256 encrypt one block.
pub fn aes256_encrypt_block(key: &[u8; 32], block: &[u8; 16]) -> [u8; 16] {
    let round_keys = expand_key(key, 8, 14);
    encrypt_block(block, &round_keys, 14)
}

/// AES-128 in CTR mode. `iv` is the initial 16-byte counter, incremented as a
/// big-endian integer once per block. Encrypt and decrypt are the same call.
pub fn aes128_ctr(key: &[u8; 16], iv: &[u8; 16], input: &[u8]) -> Vec<u8> {
    let round_keys = expand_key(key, 4, 10);
    let mut counter = *iv;
    let mut out = vec![0u8; input.len()];
    let mut offset = 0;
    while offset < input.len() {
        let stream = encrypt_block(&counter, &round_keys, 10);
        let n = (input.len() - offset).min(16);
        for i in 0..n {
            out[offset + i] = input[offset + i] ^ stream[i];
        }
        offset += n;
        inc_128(&mut counter);
    }
    out
}

/// AES-256-GCM seal. `iv` is 12 bytes. Returns ciphertext and a 16-byte tag.
pub fn aes256_gcm_seal(
    key: &[u8; 32],
    iv: &[u8; 12],
    aad: &[u8],
    plaintext: &[u8],
) -> (Vec<u8>, [u8; 16]) {
    let round_keys = expand_key(key, 8, 14);
    let h = encrypt_block(&[0u8; 16], &round_keys, 14);
    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(iv);
    j0[15] = 1;
    let mut counter = j0;
    inc_32(&mut counter);
    let mut body = vec![0u8; plaintext.len()];
    let mut offset = 0;
    while offset < plaintext.len() {
        let stream = encrypt_block(&counter, &round_keys, 14);
        let n = (plaintext.len() - offset).min(16);
        for i in 0..n {
            body[offset + i] = plaintext[offset + i] ^ stream[i];
        }
        offset += n;
        inc_32(&mut counter);
    }
    let tag = finish_tag(&round_keys, &h, &j0, aad, &body);
    (body, tag)
}

/// AES-256-GCM open. Fails if the tag does not match. On failure no plaintext is returned.
pub fn aes256_gcm_open(
    key: &[u8; 32],
    iv: &[u8; 12],
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8; 16],
) -> Result<Vec<u8>, CipherError> {
    let round_keys = expand_key(key, 8, 14);
    let h = encrypt_block(&[0u8; 16], &round_keys, 14);
    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(iv);
    j0[15] = 1;
    let expect = finish_tag(&round_keys, &h, &j0, aad, ciphertext);
    if !tags_equal(&expect, tag) {
        return Err(CipherError::TagMismatch);
    }
    let mut counter = j0;
    inc_32(&mut counter);
    let mut plain = vec![0u8; ciphertext.len()];
    let mut offset = 0;
    while offset < ciphertext.len() {
        let stream = encrypt_block(&counter, &round_keys, 14);
        let n = (ciphertext.len() - offset).min(16);
        for i in 0..n {
            plain[offset + i] = ciphertext[offset + i] ^ stream[i];
        }
        offset += n;
        inc_32(&mut counter);
    }
    Ok(plain)
}

fn finish_tag(round_keys: &[u8], h: &[u8; 16], j0: &[u8; 16], aad: &[u8], body: &[u8]) -> [u8; 16] {
    let s = ghash(h, aad, body);
    let ek = encrypt_block(j0, round_keys, 14);
    let mut tag = [0u8; 16];
    for i in 0..16 {
        tag[i] = s[i] ^ ek[i];
    }
    tag
}

fn tags_equal(a: &[u8; 16], b: &[u8; 16]) -> bool {
    let mut diff = 0u8;
    for i in 0..16 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

fn ghash(h: &[u8; 16], aad: &[u8], body: &[u8]) -> [u8; 16] {
    let mut y = [0u8; 16];
    ghash_blocks(&mut y, h, aad);
    ghash_blocks(&mut y, h, body);
    let mut len_block = [0u8; 16];
    len_block[..8].copy_from_slice(&(aad.len() as u64 * 8).to_be_bytes());
    len_block[8..].copy_from_slice(&(body.len() as u64 * 8).to_be_bytes());
    xor_block(&mut y, &len_block);
    gmul(&y, h)
}

fn ghash_blocks(y: &mut [u8; 16], h: &[u8; 16], data: &[u8]) {
    let mut offset = 0;
    while offset < data.len() {
        let mut block = [0u8; 16];
        let n = (data.len() - offset).min(16);
        block[..n].copy_from_slice(&data[offset..offset + n]);
        xor_block(y, &block);
        *y = gmul(y, h);
        offset += n;
    }
}

fn gmul(x: &[u8; 16], y: &[u8; 16]) -> [u8; 16] {
    let mut z = [0u8; 16];
    let mut v = *y;
    for byte_index in 0..16 {
        for bit in 0..8 {
            if (x[byte_index] >> (7 - bit)) & 1 == 1 {
                xor_block(&mut z, &v);
            }
            let lsb = v[15] & 1;
            shift_right(&mut v);
            if lsb == 1 {
                v[0] ^= 0xe1;
            }
        }
    }
    z
}

fn shift_right(v: &mut [u8; 16]) {
    let mut carry = 0u8;
    for byte in v.iter_mut() {
        let next = *byte & 1;
        *byte = (*byte >> 1) | (carry << 7);
        carry = next;
    }
}

fn xor_block(a: &mut [u8; 16], b: &[u8; 16]) {
    for i in 0..16 {
        a[i] ^= b[i];
    }
}

fn inc_32(counter: &mut [u8; 16]) {
    for i in (12..16).rev() {
        let (sum, overflow) = counter[i].overflowing_add(1);
        counter[i] = sum;
        if !overflow {
            break;
        }
    }
}

fn inc_128(counter: &mut [u8; 16]) {
    for i in (0..16).rev() {
        let (sum, overflow) = counter[i].overflowing_add(1);
        counter[i] = sum;
        if !overflow {
            break;
        }
    }
}

fn expand_key(key: &[u8], nk: usize, nr: usize) -> Vec<u8> {
    let total = 16 * (nr + 1);
    let mut words = vec![0u8; total];
    words[..key.len()].copy_from_slice(key);
    let mut generated = key.len();
    let mut rcon_i = 1usize;
    let mut prev = [0u8; 4];
    prev.copy_from_slice(&words[generated - 4..generated]);
    while generated < total {
        if generated % (nk * 4) == 0 {
            rot_word(&mut prev);
            sub_word(&mut prev);
            prev[0] ^= RCON[rcon_i];
            rcon_i += 1;
        } else if nk > 6 && generated % (nk * 4) == 16 {
            sub_word(&mut prev);
        }
        for j in 0..4 {
            prev[j] ^= words[generated - nk * 4 + j];
        }
        words[generated..generated + 4].copy_from_slice(&prev);
        generated += 4;
    }
    words
}

fn rot_word(word: &mut [u8; 4]) {
    let first = word[0];
    word[0] = word[1];
    word[1] = word[2];
    word[2] = word[3];
    word[3] = first;
}

fn sub_word(word: &mut [u8; 4]) {
    for b in word.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

fn encrypt_block(block: &[u8; 16], round_keys: &[u8], nr: usize) -> [u8; 16] {
    let mut state = *block;
    add_round_key(&mut state, &round_keys[0..16]);
    for round in 1..nr {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        let start = round * 16;
        add_round_key(&mut state, &round_keys[start..start + 16]);
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    let start = nr * 16;
    add_round_key(&mut state, &round_keys[start..start + 16]);
    state
}

fn add_round_key(state: &mut [u8; 16], key: &[u8]) {
    for i in 0..16 {
        state[i] ^= key[i];
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let s = *state;
    state[1] = s[5];
    state[5] = s[9];
    state[9] = s[13];
    state[13] = s[1];
    state[2] = s[10];
    state[6] = s[14];
    state[10] = s[2];
    state[14] = s[6];
    state[3] = s[15];
    state[7] = s[3];
    state[11] = s[7];
    state[15] = s[11];
}

fn mix_columns(state: &mut [u8; 16]) {
    for col in 0..4 {
        let i = col * 4;
        let a0 = state[i];
        let a1 = state[i + 1];
        let a2 = state[i + 2];
        let a3 = state[i + 3];
        state[i] = xtime(a0) ^ (xtime(a1) ^ a1) ^ a2 ^ a3;
        state[i + 1] = a0 ^ xtime(a1) ^ (xtime(a2) ^ a2) ^ a3;
        state[i + 2] = a0 ^ a1 ^ xtime(a2) ^ (xtime(a3) ^ a3);
        state[i + 3] = (xtime(a0) ^ a0) ^ a1 ^ a2 ^ xtime(a3);
    }
}

fn xtime(a: u8) -> u8 {
    let shifted = a << 1;
    if a & 0x80 == 0 {
        shifted
    } else {
        shifted ^ 0x1b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn aes128_block_matches_fips_197() {
        let key: [u8; 16] = hex("000102030405060708090a0b0c0d0e0f").try_into().unwrap();
        let block: [u8; 16] = hex("00112233445566778899aabbccddeeff").try_into().unwrap();
        let out = aes128_encrypt_block(&key, &block);
        assert_eq!(hex::encode_local(&out), "69c4e0d86a7b0430d8cdb78070b4c55a");
    }

    #[test]
    fn aes256_block_matches_fips_197() {
        let key: [u8; 32] = hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
            .try_into()
            .unwrap();
        let block: [u8; 16] = hex("00112233445566778899aabbccddeeff").try_into().unwrap();
        let out = aes256_encrypt_block(&key, &block);
        assert_eq!(hex::encode_local(&out), "8ea2b7ca516745bfeafc49904b496089");
    }

    #[test]
    fn aes128_ctr_matches_nist_sp800_38a() {
        let key: [u8; 16] = hex("2b7e151628aed2a6abf7158809cf4f3c").try_into().unwrap();
        let iv: [u8; 16] = hex("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").try_into().unwrap();
        let plain = hex("6bc1bee22e409f96e93d7e117393172a");
        let ct = aes128_ctr(&key, &iv, &plain);
        assert_eq!(hex::encode_local(&ct), "874d6191b620e3261bef6864990db6ce");
        assert_eq!(aes128_ctr(&key, &iv, &ct), plain);
    }

    #[test]
    fn aes256_gcm_round_trip_and_rejects_bad_tag() {
        let key = [0x11u8; 32];
        let iv = [0x22u8; 12];
        let aad = b"profile";
        let plain = b"bookmarks and cookies";
        let (body, tag) = aes256_gcm_seal(&key, &iv, aad, plain);
        let opened = aes256_gcm_open(&key, &iv, aad, &body, &tag).unwrap();
        assert_eq!(opened, plain);

        let mut bad = tag;
        bad[0] ^= 1;
        assert_eq!(
            aes256_gcm_open(&key, &iv, aad, &body, &bad),
            Err(CipherError::TagMismatch)
        );

        let empty = aes256_gcm_seal(&key, &iv, b"", b"");
        assert!(empty.0.is_empty());
        assert!(
            aes256_gcm_open(&key, &iv, b"", b"", &empty.1)
                .unwrap()
                .is_empty()
        );
        let mut bad_empty = empty.1;
        bad_empty[15] ^= 0xff;
        assert_eq!(
            aes256_gcm_open(&key, &iv, b"", b"", &bad_empty),
            Err(CipherError::TagMismatch)
        );
    }

    #[test]
    fn aes256_gcm_empty_matches_nist() {
        let (body, tag) = aes256_gcm_seal(&[0u8; 32], &[0u8; 12], b"", b"");
        assert!(body.is_empty());
        assert_eq!(hex::encode_local(&tag), "530f8afbc74536b9a963b4f1c4cb738b");
    }

    mod hex {
        pub fn encode_local(bytes: &[u8]) -> String {
            const HEX: &[u8; 16] = b"0123456789abcdef";
            let mut out = String::with_capacity(bytes.len() * 2);
            for &b in bytes {
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0xf) as usize] as char);
            }
            out
        }
    }
}
