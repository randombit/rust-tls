
use to_bytes::ToBytes;

pub fn hello_random() -> ~[u8] {
    let time32 = std::time::get_time().sec as u32;

    let time_bytes = time32.to_bytes(false);

    let random = crypto::rand::rand_bytes(28);

    time_bytes + random
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hello_random() {
        let hr = hello_random();

        io::println(fmt!("%?", hr));
    }
}
