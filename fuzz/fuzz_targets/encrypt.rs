#![no_main]
use libfuzzer_sys::fuzz_target;
use self_encryption::{encrypt, bytes::Bytes};

fuzz_target!(|value: &[u8]| {
    if value.len() > 0 {
        let mut data = value.to_vec();

        if value.len() < 3072 {
            data = data.repeat(3072 / value.len() + 1);
        }

        let _ = encrypt(Bytes::from(data)).unwrap();
    }
});
