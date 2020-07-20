use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use openssl::aes::{unwrap_key, AesKey};
use openssl::symm::{decrypt, Cipher};
use std::convert::TryInto;
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_5.txt").unwrap());

    let mut i = 0;

    // First 32 bytes: The 256-bit key encrypting key (KEK).
    let kek = &bytes[i..i + 32];
    i += 32;

    // Next 8 bytes: The 64-bit initialization vector (IV) for the wrapped key.
    let key_iv: [u8; 8] = (&bytes[i..i + 8]).try_into().unwrap();
    i += 8;

    // Next 40 bytes: The wrapped (encrypted) key. When decrypted, this will become the 256-bit encryption key.
    let key = &bytes[i..i + 40];
    i += 40;

    // Next 16 bytes: The 128-bit initialization vector (IV) for the encrypted payload.
    let payload_iv = &bytes[i..i + 16];
    i += 16;

    // All remaining bytes: The encrypted payload.
    let payload = &bytes[i..bytes.len()];

    // Initialise buffer and AES KEK, used to decrypt key
    let decrypted_key: &mut [u8] = &mut [0; 32];
    let aes_kek = AesKey::new_decrypt(kek).expect("Unable to create AesKey");
    // Decrypt KEK
    let _bytes_in_key = unwrap_key(&aes_kek, Some(key_iv), decrypted_key, key).unwrap();

    // Decrypt payload
    let decrypted_payload = decrypt(
        Cipher::aes_256_cbc(),
        &decrypted_key,
        Some(payload_iv),
        payload,
    )
    .unwrap();

    fs::write("output.txt", bytes_to_ascii(decrypted_payload)).expect("Unable to write file");
}
