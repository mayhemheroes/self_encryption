#![no_main]
use libfuzzer_sys::fuzz_target;
use self_encryption::{encrypt, bytes::Bytes};

fuzz_target!(|value: &[u8]| {
    if value.len() > 3072 {
        let data = value.to_vec();
        let _ = encrypt(Bytes::from(data)).unwrap();
    }
});
