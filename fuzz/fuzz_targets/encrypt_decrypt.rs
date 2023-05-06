#![no_main]
use libfuzzer_sys::fuzz_target;
use self_encryption::{encrypt, bytes::Bytes, decrypt_full_set};

fuzz_target!(|value: &[u8]| {
    if value.len() > 0 {
        let mut data = value.to_vec();

        if value.len() < 3072 {
            data = data.repeat(3072 / value.len() + 1);
        }

        let data_bytes = Bytes::from(data);

        let (dmap, chunks) = encrypt(data_bytes.clone()).unwrap();
        let dec = decrypt_full_set(&dmap, chunks.as_ref()).unwrap();
        assert_eq!(dec, data_bytes);
    }
});
