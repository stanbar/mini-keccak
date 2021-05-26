use crate::matrix::Matrix;
use std::convert::TryInto;

fn to_array(hash: Vec<u16>) -> [u8; 16] {
    let flattened: Vec<u8> = hash
        .iter()
        .map(|x| vec![((x & 0xFF00) >> 8) as u8, (x & 0x00FF) as u8])
        .flatten()
        .collect();
    flattened.try_into().expect("Could not map vec to array")
}

pub const BLOCK_SIZE: usize = 20;
pub const LENGTH_SIZE: usize = 1;

fn add_padding(input: &mut Vec<u8>) {
    let payload_size = input.len();
    let blocks = (payload_size + LENGTH_SIZE - 1) / BLOCK_SIZE + 1;
    let bytes = blocks * BLOCK_SIZE;
    input.resize(bytes, 0u8);
    input[payload_size] = 0x80;
}

pub fn hash(mut input: Vec<u8>) -> [u8; 16] {
    add_padding(&mut input);
    let mut state = Matrix::zeros();

    for input in input.chunks(20) {
        let input: Vec<u16> = input
            .chunks(2)
            .map(|x| (x[0] as u16) << 8 | (x[1] as u16))
            .collect();

        for i in 0..5 {
            state.0[0][i] ^= input[i];
            state.0[1][i] ^= input[i + 5];
        }
        rounds(&mut state);
    }

    let mut partial_result = [0u16; 5];
    partial_result.copy_from_slice(&state.0[0][..]);
    rounds(&mut state);
    let result: Vec<u16> = partial_result
        .iter()
        .map(|x| x.clone())
        .chain(state.0[0][0..=2].iter().map(|x| x.clone()))
        .collect();

    to_array(result)
}

fn rounds(a: &mut Matrix) {
    for round in 0..10 {
        // Theta
        let mut c = [0u16; 5];
        for i in 0..5 {
            c[i] = a.0[i][0] ^ a.0[i][1] ^ a.0[i][2] ^ a.0[i][3] ^ a.0[i][4];
        }
        let mut d = [0u16; 5];
        for i in 0..5 {
            d[i] = c[(5 + i - 1).rem_euclid(5)] ^ (c[(i + 1).rem_euclid(5)].rotate_left(1))
        }
        for i in 0..5 {
            for j in 0..5 {
                a.0[i][j] = a.0[i][j] ^ d[i];
            }
        }

        // pri
        for i in 0..5 {
            for j in 0..5 {
                let rotate_by = (7 * i + j) as u32;
                a.0[i][j] = a.0[i][j].rotate_left(rotate_by % 16);
            }
        }

        // pi
        let mut b = Matrix::zeros();
        for i in 0..5 {
            for j in 0..5 {
                b.0[(3 * i + 2 * j) % 5][i] = a.0[i][j];
            }
        }

        // xi
        for i in 0..5 {
            for j in 0..5 {
                a.0[i][j] =
                    b.0[i][j] ^ ((!b.0[(i + 1).rem_euclid(5)][j]) & b.0[(i + 2).rem_euclid(5)][j])
            }
        }

        // i
        a.0[0][0] = a.0[0][0] ^ R[round];
    }
}

const R: [u16; 10] = [
    0x3EC2, 0x738D, 0xB119, 0xC5E7, 0x86C6, 0xDC1B, 0x57D6, 0xDA3A, 0x7710, 0x9200,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keccak() {
        [
            (
                "",
                [
                    0xE2, 0x25, 0x5B, 0xFB, 0xD3, 0xCF, 0x86, 0xE0, 0xDB, 0xE5, 0x2A, 0xA9, 0x67,
                    0x82, 0xEB, 0x8D,
                ],
            ),
            (
                "AbCxYz",
                [
                    0x5A, 0x0F, 0xB1, 0xF1, 0xF0, 0x14, 0x98, 0x27, 0xC5, 0x36, 0x28, 0x0F, 0xEA,
                    0xD1, 0x67, 0xD1,
                ],
            ),
            (
                "1234567890",
                [
                    0x37, 0x46, 0x68, 0x9D, 0x2E, 0xD8, 0x04, 0x06, 0xEB, 0xE2, 0x03, 0x8B, 0x5F,
                    0xDD, 0xF9, 0xD5,
                ],
            ),
            (
                "Ala ma kota, kot ma ale.",
                [
                    0xD6, 0x62, 0xF8, 0xE0, 0x32, 0x8D, 0x46, 0xCB, 0x53, 0xCC, 0xB8, 0x9D, 0x21,
                    0x9A, 0x94, 0x85,
                ],
            ),
            (
                "Ty, ktory wchodzisz, zegnaj sie z nadzieja.",
                [
                    0xB5, 0x34, 0xF7, 0xEF, 0xF7, 0x14, 0x8C, 0x43, 0x20, 0x57, 0xDF, 0xD6, 0x11,
                    0x38, 0x7A, 0x30,
                ],
            ),
            (
                "a".repeat(48000).as_str(),
                [
                    0x07, 0x2F, 0xB0, 0x3B, 0xC3, 0xC9, 0x96, 0x50, 0x66, 0x3B, 0x2B, 0x89, 0xA6,
                    0xE9, 0x9F, 0x74,
                ],
            ),
            (
                "a".repeat(48479).as_str(),
                [
                    0xAA, 0x64, 0x8B, 0xAE, 0xF6, 0x95, 0x48, 0x33, 0xF9, 0x55, 0x5D, 0x55, 0xA7,
                    0x97, 0xD2, 0xCB,
                ],
            ),
            (
                "a".repeat(48958).as_str(),
                [
                    0x9A, 0x9C, 0x15, 0x4F, 0x81, 0x7A, 0x48, 0xE4, 0xE2, 0x8D, 0x8A, 0x8C, 0x68,
                    0x7A, 0xCD, 0x60,
                ],
            ),
        ]
        .iter()
        .for_each(|x| {
            let hash = hash(x.0.as_bytes().to_vec());
            assert_eq!(hash, x.1, "for message {}", x.0);
        });
    }
}
