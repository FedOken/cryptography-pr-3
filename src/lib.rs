// Substitution table for S block for encoding
const S_BLOCK_ENCODE: [[u8; 4]; 4] = [
    [10, 04, 03, 12],
    [06, 09, 01, 15],
    [14, 02, 13, 07],
    [00, 08, 05, 11],
];

// Substitution table for S block for decoding
const S_BLOCK_DECODE: [[u8; 4]; 4] = [
    [12, 06, 09, 02],
    [01, 14, 04, 11],
    [13, 05, 00, 15],
    [03, 10, 08, 07],
];

// Encode bits vector with 'key'
pub fn encode(bits: &Vec<u8>, key: u128) -> Vec<u8> {
    p_block_encryption(&s_block_encryption(&bits, S_BLOCK_ENCODE), key)
}

// Decode bits vector with 'key'
pub fn decode(bits: &Vec<u8>, key: u128) -> Vec<u8> {
    s_block_encryption(&p_block_encryption(&bits, key), S_BLOCK_DECODE)
}

// Transform ASCII string to bits vector (rust => [0,1,1,1,0,0,1,0,0,1,1,1,0,1,0,1...])
pub fn string_to_bits(string: &str) -> Vec<u8> {
    let mut bits = Vec::new();

    for c in string.chars() {
        let ascii_value = c as u8;
        for i in (0..8).rev() {
            let bit = (ascii_value >> i) & 1;
            bits.push(bit);
        }
    }

    bits
}

// Transform bits vector to string ([0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1] => 00000000 11111111)
pub fn bits_to_bits_string(bits: &Vec<u8>) -> String {
    let mut result = String::new();

    for chunk in bits.chunks(8) {
        for bit in chunk {
            result.push_str(&bit.to_string());
        }
        result.push(' ');
    }

    result.trim().to_string()
}

// Transform bits vector to ASCII string ([0,1,1,1,0,0,1,0,0,1,1,1,0,1,0,1...] => rust)
pub fn bits_to_ascii_string(bits: &[u8]) -> String {
    let mut result = String::new();

    for chunk in bits.chunks(8) {
        let mut ascii_value = 0;
        for &bit in chunk {
            ascii_value = (ascii_value << 1) | bit;
        }

        if let Some(c) = char::from_u32(u32::from(ascii_value)) {
            result.push(c);
        } else {
            panic!("Invalid ASCII value: {}", ascii_value);
        }
    }

    result
}

// Encrypt or decrypt bytes vector based on substitution table
fn s_block_encryption(bits: &Vec<u8>, table: [[u8; 4]; 4]) -> Vec<u8> {
    let mut enoded_bits: Vec<u8> = Vec::new();

    for chunk in bits.chunks_exact(4) {
        if let [bit_1, bit_2, bit_3, bit_4] = chunk {

            let row_index = bits_to_number(&vec![*bit_1, *bit_2]) as usize;
            let column_index = bits_to_number(&vec![*bit_3, *bit_4]) as usize;

            let mut encoded_value = number_to_bits(table[row_index][column_index], 4);

            enoded_bits.append(&mut encoded_value);
        }
    }

    enoded_bits
}

// Encrypt or decrypt bytes vector based on 'key'
fn p_block_encryption(bits: &Vec<u8>, key: u128) -> Vec<u8> {
    let mut p_bits: Vec<u8> = Vec::new();
    
    for chunk in bits.chunks(8) {
        let num = bits_to_number(&chunk.to_vec());
        let entropy = (key % 256) as u8;

        let mut p_bit = number_to_bits(num ^ entropy, 8);

        p_bits.append(&mut p_bit);
    }

    p_bits
}

fn bits_to_number(bits: &Vec<u8>) -> u8 {
    let mut result = 0;
    for &bit in bits {
        result <<= 1;
        result |= bit & 0x01;
    }
    result
}

fn number_to_bits(number: u8, num_bits: usize) -> Vec<u8> {
    (0..num_bits).map(|bit| (number >> bit) & 1).collect::<Vec<u8>>().into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_from_string() {
        let test_string = "Hello";
        let key: u128 = 998;

        let test_bits = string_to_bits(&test_string); // Convert string to ACII representation '01001000 01100101 01101100 01101100 01101111'
        let bits_encoded = encode(&test_bits, key); // Encode bits representation with 'key'
        let bits_decoded = decode(&bits_encoded, key); // Decode encoded representation with 'key'

        println!("{}", bits_to_bits_string(&bits_encoded));

        assert_eq!(bits_to_bits_string(&test_bits), "01001000 01100101 01101100 01101100 01101111");
        assert_ne!(bits_to_bits_string(&bits_encoded), "01001000 01100101 01101100 01101100 01101111");
        assert_eq!(bits_to_bits_string(&bits_decoded), "01001000 01100101 01101100 01101100 01101110");
        assert_eq!(bits_to_ascii_string(&bits_decoded), test_string);
    }

    #[test]
    fn test_encryption_from_bytes() {
        let key: u128 = 9999999999;

        let test_bits = vec![0,1,0,0,1,0,0,0];
        let bits_encoded = encode(&test_bits, key);
        let bits_decoded = decode(&bits_encoded, key);

        assert_eq!(bits_to_bits_string(&test_bits), "01001000");
        assert_ne!(bits_to_bits_string(&bits_encoded), "01001000");
        assert_eq!(bits_to_bits_string(&bits_decoded), "01001000");
        assert_eq!(bits_to_ascii_string(&bits_decoded), "H");
    }

    #[test]
    fn should_return_invalid_bytes_with_invalid_key() {
        let test_string = "Hello";
        let key: u128 = 1;

        let test_bits = string_to_bits(&test_string);
        let bits_encoded = encode(&test_bits, key);
        let bits_decoded = decode(&bits_encoded, 2);

        assert_ne!(bits_to_ascii_string(&bits_decoded), test_string);
    }
}
