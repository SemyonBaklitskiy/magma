use std::io;

// Поскольку в тексте стандарта (по неведомой традиции) нулевой байт пишется
// в конце, а последний в начале, то для корректной работы программы строки
// таблицы необходимо записывать в обратном порядке, а не так, как изложено
// в стандарте
const PI: [[u8; 16]; 8] = [
    [12, 4, 6, 2, 10, 5, 11, 9, 14, 8, 13, 7, 0, 3, 15, 1],
    [6, 8, 2, 3, 9, 10, 5, 12, 1, 14, 4, 7, 11, 13, 0, 15],
    [11, 3, 5, 8, 2, 15, 10, 13, 14, 1, 7, 4, 12, 9, 6, 0],
    [12, 8, 2, 1, 13, 4, 15, 6, 7, 0, 10, 5, 3, 14, 9, 11],
    [7, 15, 5, 10, 8, 1, 6, 13, 0, 9, 3, 14, 11, 4, 2, 12],
    [5, 13, 15, 6, 9, 2, 12, 10, 11, 7, 8, 1, 4, 3, 14, 0],
    [8, 14, 2, 5, 6, 9, 1, 12, 15, 4, 11, 0, 13, 10, 3, 7],
    [1, 7, 14, 13, 0, 5, 8, 3, 4, 15, 10, 6, 9, 12, 11, 2],
];

const KEY: [u8; 32] = [
    0xff, 0xfe, 0xfd, 0xfc, 0xfb, 0xfa, 0xf9, 0xf8, 0xf7, 0xf6, 0xf5, 0xf4, 0xf3, 0xf2, 0xf1, 0xf0,
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
];

fn gost_magma_expand_key(key: [u8; 32]) -> [[u8; 4]; 32] {
    let mut iter_key: [[u8; 4]; 32] = [[0; 4]; 32];

    iter_key[7].copy_from_slice(&key[0..4]);
    iter_key[6].copy_from_slice(&key[4..8]);
    iter_key[5].copy_from_slice(&key[8..12]);
    iter_key[4].copy_from_slice(&key[12..16]);
    iter_key[3].copy_from_slice(&key[16..20]);
    iter_key[2].copy_from_slice(&key[20..24]);
    iter_key[1].copy_from_slice(&key[24..28]);
    iter_key[0].copy_from_slice(&key[28..32]);
    iter_key[15].copy_from_slice(&key[0..4]);
    iter_key[14].copy_from_slice(&key[4..8]);
    iter_key[13].copy_from_slice(&key[8..12]);
    iter_key[12].copy_from_slice(&key[12..16]);
    iter_key[11].copy_from_slice(&key[16..20]);
    iter_key[10].copy_from_slice(&key[20..24]);
    iter_key[9].copy_from_slice(&key[24..28]);
    iter_key[8].copy_from_slice(&key[28..32]);
    iter_key[23].copy_from_slice(&key[0..4]);
    iter_key[22].copy_from_slice(&key[4..8]);
    iter_key[21].copy_from_slice(&key[8..12]);
    iter_key[20].copy_from_slice(&key[12..16]);
    iter_key[19].copy_from_slice(&key[16..20]);
    iter_key[18].copy_from_slice(&key[20..24]);
    iter_key[17].copy_from_slice(&key[24..28]);
    iter_key[16].copy_from_slice(&key[28..32]);
    iter_key[31].copy_from_slice(&key[28..32]);
    iter_key[30].copy_from_slice(&key[24..28]);
    iter_key[29].copy_from_slice(&key[20..24]);
    iter_key[28].copy_from_slice(&key[16..20]);
    iter_key[27].copy_from_slice(&key[12..16]);
    iter_key[26].copy_from_slice(&key[8..12]);
    iter_key[25].copy_from_slice(&key[4..8]);
    iter_key[24].copy_from_slice(&key[0..4]);

    iter_key
}

fn gost_magma_t(in_data: [u8; 4], out_data: &mut [u8; 4]) {
    let mut first_part_byte: u8;
    let mut sec_part_byte: u8;

    for i in 0..4 {
        first_part_byte = in_data[i] & 0x0f;
        sec_part_byte = (in_data[i] & 0xf0) >> 4;
        first_part_byte = PI[i * 2][first_part_byte as usize];
        sec_part_byte = PI[i * 2 + 1][sec_part_byte as usize];
        out_data[i] = (sec_part_byte << 4) | first_part_byte;
    }
}

fn gost_magma_add(a: &[u8; 4], b: [u8; 4], c: &mut [u8; 4]) {
    for i in 0..4 {
        c[i] = a[i] ^ b[i];
    }
}

fn gost_magma_add_32(a: &[u8; 4], b: &[u8; 4], c: &mut [u8; 4]) {
    let mut internal: u32 = 0;
    for i in 0..4 {
        internal = a[i] as u32 + b[i] as u32 + (internal >> 8);
        c[i] = (internal & 0xff) as u8;
    }
}

fn gost_magma_g_help(k: &[u8; 4], a: &[u8; 4], out_data: &mut [u8; 4]) {
    let mut internal: [u8; 4] = [0; 4];
    let mut out_data_32: u32;
    gost_magma_add_32(a, k, &mut internal);
    gost_magma_t(internal, &mut internal);
    out_data_32 = internal[3] as u32;
    out_data_32 = (out_data_32 << 8) + internal[2] as u32;
    out_data_32 = (out_data_32 << 8) + internal[1] as u32;
    out_data_32 = (out_data_32 << 8) + internal[0] as u32;
    out_data_32 = (out_data_32 << 11) | (out_data_32 >> 21);

    out_data[0] = out_data_32 as u8;
    out_data[1] = (out_data_32 >> 8) as u8;
    out_data[2] = (out_data_32 >> 16) as u8;
    out_data[3] = (out_data_32 >> 24) as u8;
}

fn gost_magma_g(k: &[u8; 4], a: &[u8; 8], mut out_data: [u8; 8]) -> [u8; 8] {
    let mut a_0: [u8; 4] = [0; 4];
    let mut a_1: [u8; 4] = [0; 4];
    let mut g: [u8; 4] = [0; 4];

    for i in 0..4 {
        a_1[i] = a[4 + i];
        a_0[i] = a[i];
    }
    gost_magma_g_help(k, &a_0, &mut g);
    gost_magma_add(&a_1, g, &mut g);

    for i in 0..4 {
        a_1[i] = a_0[i];
        a_0[i] = g[i];
    }

    for i in 0..4 {
        out_data[i] = a_0[i];
        out_data[4 + i] = a_1[i];
    }

    out_data
}

fn gost_magma_g_fin(k: &[u8; 4], a: &[u8; 8], mut out_data: [u8; 8]) -> [u8; 8] {
    let mut a_0: [u8; 4] = [0; 4];
    let mut a_1: [u8; 4] = [0; 4];
    let mut g: [u8; 4] = [0; 4];

    for i in 0..4 {
        a_1[i] = a[4 + i];
        a_0[i] = a[i];
    }
    gost_magma_g_help(k, &a_0, &mut g);
    gost_magma_add(&a_1, g, &mut g);

    for i in 0..4 {
        a_1[i] = g[i];
    }

    for i in 0..4 {
        out_data[i] = a_0[i];
        out_data[4 + i] = a_1[i];
    }

    out_data
}

fn gost_magma_encrypt(text_message: &[u8; 8], iter_key: &[[u8; 4]; 32]) -> [u8; 8] {
    let mut out_data = [0; 8];
    out_data = gost_magma_g(&iter_key[0], &text_message, out_data);

    for i in 1..31 {
        out_data = gost_magma_g(&iter_key[i], &out_data, out_data);
    }

    gost_magma_g_fin(&iter_key[31], &out_data, out_data)
}

fn gost_magma_decrypt(encrypted_message: &[u8; 8], iter_key: &[[u8; 4]; 32]) -> [u8; 8] {
    let mut out_data = [0; 8];
    out_data = gost_magma_g(&iter_key[31], &encrypted_message, out_data);

    for i in 1..=30 {
        out_data = gost_magma_g(&iter_key[31 - i], &out_data, out_data);
    }

    gost_magma_g_fin(&iter_key[0], &out_data, out_data)
}

fn main() {
    let mut input = String::new();

    println!("Enter text message in hex (for example, 1a3fbd): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read from stdin");

    let text_message = u64::from_str_radix(input.trim(), 16)
        .expect("Wrong text message")
        .to_be_bytes();

    println!("Text message: 0x{:x}", u64::from_be_bytes(text_message));
    let iter_key = gost_magma_expand_key(KEY);
    let encrypted_text = gost_magma_encrypt(&text_message, &iter_key);
    println!("Encrypted text: 0x{:x}", u64::from_be_bytes(encrypted_text));

    let decrypted_text = gost_magma_decrypt(&encrypted_text, &iter_key);
    println!("Decrypted text: 0x{:x}", u64::from_be_bytes(decrypted_text));

    assert_eq!(text_message, decrypted_text);
}
