#![no_main]
use libfuzzer_sys::fuzz_target;
use self_encryption::bytes::Bytes;

fuzz_target!(|value: &[u8]| {
    let data = value.to_vec();
    let _ = Bytes::from(data);
});
