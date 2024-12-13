use std::{io, time::Instant};

use magma::Magma;

// Example of key
// fffefdfcfbfaf9f8f7f6f5f4f3f2f1f000112233445566778899aabbccddeeff

fn main() {
    let mut input_text = String::new();

    println!("Enter text message in hex (for example, 1a3fbd):");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Couldn't read from stdin");

    let mut input_key = String::new();
    println!("Enter 256 bit key in hex:");
    io::stdin()
        .read_line(&mut input_key)
        .expect("Couldn't read from stdin");

    let start = Instant::now();

    let magma = Magma::new(input_text, input_key);
    let text_message = magma.text_mesage();
    let encrypted_text = magma.encrypt();
    let decrypted_text = magma.decrypt(encrypted_text);

    let time = start.elapsed();
    assert_eq!(text_message, decrypted_text);

    println!("Text message: 0x{:x}", u64::from_be_bytes(text_message));
    println!("Encrypted text: 0x{:x}", u64::from_be_bytes(encrypted_text));
    println!("Decrypted text: 0x{:x}", u64::from_be_bytes(decrypted_text));
    println!("Time: {} us", time.as_micros());
}
