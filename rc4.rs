// Copyright 2013 The Rust TLS Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This should probably ffi openssl instead

trait Rc4 {
    fn cipher(&mut self, &[mut u8]);
}

pub fn rc4(key: &[const u8]) -> Rc4 {

    struct Rc4State {
        S: ~[u8],
        x: u8,
        y: u8
    };

    impl Rc4State: Rc4 {

        fn cipher(&mut self, msg: &[mut u8]) {

            for uint::range(0, msg.len()) |i| {
                self.x += 1;
                let sx = self.S[self.x];
                self.y = (self.y + sx);
                let sy = self.S[self.y];

                self.S[self.x] = sy;
                self.S[self.y] = sx;

                msg[i] ^= self.S[sx + sy];
            }

        }

    }

    fn key_schedule(st: &mut Rc4State, key: &[const u8]) {

        for uint::range(0, 256) |i| {
            st.S.push(i as u8);
        }

        let mut idx: uint = 0;

        for uint::range(0, 256) |i| {
            idx = (idx + key[i % key.len()] as uint + st.S[i] as uint) % 256;

            let sx = st.S[i];
            let sy = st.S[idx];

            st.S[i] = sy;
            st.S[idx] = sx;
        }
    }

    let mut st = Rc4State {
        S: ~[],
        x: 0,
        y: 0
    };

    key_schedule(&mut st, key);

    return (st) as Rc4;
}

#[cfg(test)]
mod tests {

    use rc4;
    use hex::FromHex;
    use hex::ToHex;
    use std::time;

    #[test]
    pub fn testPerformance() {
        const trials : uint = 256;
        const bytes_per_trial : uint = 64*1024;

        let mut rc4 = rc4([1, 255, 1]);

        let start : u64 = std::time::precise_time_ns();

        let mut buf = vec::to_mut(vec::from_elem(bytes_per_trial, 0u8));

        let mut trial = 0;

        while trial < trials {
            rc4.cipher(buf);
            trial += 1;

        }

        let end : u64 = std::time::precise_time_ns();

        let duration_ns = ((end-start) as float);
        let duration_sec = duration_ns / 1000000000.0;

        let bytes_crypted = (trials * bytes_per_trial);

        let mbytes_crypted = (bytes_crypted as float) / (1024.0 * 1024.0);

        let ns_per_byte = duration_ns / (bytes_crypted as float);

        io::println(fmt!("RC4ing %? MiB took %? seconds, %? ns/byte",
                         mbytes_crypted, duration_sec, ns_per_byte));
    }

    fn testVector(keystr : &str, ctextstr : &str) {

        io::println(fmt!("RC4 test %s %s", keystr, ctextstr));

        let ctext = ctextstr.from_hex();

        let mut rc4 = rc4(keystr.from_hex());

        let mut ptext = vec::to_mut(vec::from_elem(ctext.len(), 0u8));

        rc4.cipher(ptext);

        assert ptext == ctext
    }

    #[test]
    pub fn test() {

        testVector("0000000000000000", "DE188941A3375D3A");

        testVector("0123456789ABCDEF", "7494C2E7104B0879");

        testVector("CC26F0F11FF7759081CC87B0296E46E1A29611FA042C0F09033F12FD06468624",
                   "F33562228D9339F23EFE694E45A6A5B4457F2865061384B064DD45321D399FD1DB1C3CCBE64B");
    }

}
