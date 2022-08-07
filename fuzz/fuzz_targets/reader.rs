#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data = std::io::Cursor::new(data);
    if let Ok(mut reader) = audrey::Reader::new(data) {
        let _ = reader.description();
        for _ in reader.samples::<f32>() {}
    }
});
