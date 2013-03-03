extern mod crypto;

use crypto::hash;

pub fn hmac(key: &[u8]) {
    let h = hash::Hasher(hash::SHA1);
}

fn main() {
    io::println("Hi?");
}
