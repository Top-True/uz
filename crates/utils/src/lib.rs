pub mod reflect;

pub const fn hash128(s: &str) -> u128 {
    let bytes = s.as_bytes();
    let mut hash: u128 = 0;
    let mut i = 0;
    while i < bytes.len() {
        hash = hash.wrapping_mul(131);
        hash = hash.wrapping_add(bytes[i] as u128);
        i += 1;
    }
    hash
}
