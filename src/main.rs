/// A short program that solves the puzzle posed by an [ASI job posting](
/// https://jobs.ashbyhq.com/airspace-intelligence.com/6462d3d1-443c-4a5a-8667-c1d0472fa32d).
use std::str::Utf8Error;

// The hiring manager's email was encoded in this byte array by the following process:
//
//     1. Each character was converted into its Unicode integer representation
//     2. Each integer was XORed with the secret key
//
// It's not specified whether the Unicode integer representation is UTF-8 or UTF-16, but it's safe
// to assume UTF-8 because UTF-16 characters are always exactly 2 or 4 bytes (even numbers) and
// this byte array has a length of 31 (odd number).
const ENCODED_EMAIL: &[u8] = &[
    33, 56, 46, 44, 62, 13, 44, 36, 63, 62, 61, 44, 46, 40, 96, 36, 35, 57, 40, 33, 33, 36, 42, 40,
    35, 46, 40, 99, 46, 34, 32,
];

const N: usize = ENCODED_EMAIL.len();

// Per the challenge, "...the key will be the squawk code used to signal a generic emergency, with
// any leading or trailing 0’s trimmed out."
//
// Per [Wikipedia](https://w.wiki/M9LV), ICAO specifies 7700 as the squawk code for generic
// emergencies. Worth noting is that squawk codes are four digit base 8 (octal) numbers. 0o77
// (77 in base-8) is actually 0b0011_1111 in binary or 63 in base 10. However, the intent of
// the puzzle seems to be to interpret the trimmed squawk code as a base 10 number, which is
// 0b0100_1101 in binary.
const SECRET_KEY: u8 = 77;

// That the original email contained only ASCII characters can be inferred by way of proof by
// contradiction.
//
// Fact 1: The secret key's binary representation is 0b0100_1101. Its highest set bit is in the 2^6
//   position.
//
// Fact 2: The binary representation of the highest value seen in the encoded email (99) is
//   0b0110_0011. Its highest set bit is also in the 2^6 position.
//
// Fact 3: The UTF-8 binary representations of all non-ASCII characters (i.e. one of the first 128
//   Unicode characters) have at least one bit set in the 2^7 position or higher.
//
// Hypothesis: At least one character in the original email was NOT an ASCII character (i.e. one of
//   the first 128 Unicode characters).
//
// Lemma: XORing a non-ASCII character, which has at least one bit set in the 2^7 position or
//   higher, with the secret key, which has no bits set in the 2^7 position or higher, would yield
//   at least one integer in the given array with at least one bit set in the 2^7 position or
//   higher.
//
// Contradiction: None of the given integers have any bits set in the 2^7 position or higher.
//   Therefore, the hypothesis is false and we know that all characters in the original email were
//   ASCII characters.
//
// Then, because XOR is its own inverse, the decoded email is obtained by XORing each byte with the
// secret key and interpreting the final byte array as a UTF-8 string.
const DECODED_EMAIL: [u8; N] = {
    let mut buffer = [0; N];
    let mut i = 0;

    while i < N {
        buffer[i] = ENCODED_EMAIL[i] ^ SECRET_KEY;
        i += 1;
    }

    buffer
};

fn main() -> Result<(), Utf8Error> {
    println!("{}", std::str::from_utf8(&DECODED_EMAIL)?);
    Ok(())
}
