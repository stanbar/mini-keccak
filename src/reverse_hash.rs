use rayon::prelude::*;
use minikeccak::permutations;
use std::time::Instant;

const CHARS: &[u8] =
    b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#%^-_=+([{<)]}>";

fn main() {
    vec![
        (
            2,
            [
                0xCF, 0xEA, 0xCD, 0xDA, 0xA7, 0xB4, 0x9B, 0xC7, 0x43, 0x5C, 0x25, 0x64, 0x10, 0xDF,
                0x11, 0xED,
            ],
        ),
        (
            3,
            [
                0x46, 0xE1, 0x46, 0x69, 0x6C, 0x40, 0x8A, 0x28, 0xD1, 0xF6, 0xBB, 0xB1, 0x63, 0x5D,
                0xCA, 0xC0,
            ],
        ),
        (
            4,
            [
                0xCC, 0xC0, 0x96, 0x36, 0x70, 0xA4, 0xC1, 0x2F, 0x07, 0x45, 0x02, 0x8B, 0x26, 0x7F,
                0x4A, 0xE5,
            ],
        ),
        (
            5,
            [
                0xAE, 0xF5, 0xC7, 0xA3, 0x5A, 0x08, 0xAE, 0xE6, 0xBB, 0x1E, 0xA3, 0xA1, 0x86, 0x9A,
                0x73, 0xBD,
            ],
        ),
        (
            6,
            [
                0x47, 0x37, 0x90, 0xED, 0x28, 0x11, 0xDE, 0x71, 0x5B, 0x94, 0x3A, 0x69, 0x5C, 0x50,
                0x9A, 0x6F,
            ],
        ),
    ]
    .iter()
    .for_each(|x| brute_force(x.0, x.1));
}

fn brute_force(chars_count: usize, expected: [u8; 16]) {
    let start = Instant::now();
    let mut chars = [0u8; 79];
    chars.copy_from_slice(&CHARS[..]);
    chars.reverse();
    
    let p = permutations(&chars[..], chars_count).par_bridge().find_any(|p| {
        if expected == minikeccak::core::hash(p.clone()) {
            true
        } else {
            false
        }
    });
    match p {
        None => panic!("Did not find any input"),
        Some(x) => {
            let duration = start.elapsed();
            println!(
                "Found answer for {:?}, it is {:?}. Took {:?}",
                expected,
                String::from_utf8(x.clone()),
                duration
            );
        }
    }
}
