// Copyright 2013 The Rust Crypto Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod std;

pub trait ToHex {
    pure fn to_hex() -> ~str;
}

impl &[u8]: ToHex {
    pure fn to_hex() -> ~str {

        let chars = str::chars(~"0123456789ABCDEF");

        let mut s = ~"";

        for uint::range(0, self.len()) |i| {

            let x = self[i];

            let xhi = (x >> 4) & 0x0F;
            let xlo = (x     ) & 0x0F;

            unsafe {
                str::push_char(&mut s, chars[xhi]);
                str::push_char(&mut s, chars[xlo]);
            }
        }

        s
    }
}

pub trait FromHex {
    pure fn from_hex() -> ~[u8];
}

impl &str: FromHex {
    pure fn from_hex() -> ~[u8] {
        let mut vec = vec::with_capacity(self.len() / 2);

        for str::each_chari(self) |i,c| {

            let nibble =
                if c >= '0' && c <= '9' { (c as u8) - 0x30 }
                else if c >= 'a' && c <= 'f' { (c as u8) - (0x61 - 10) }
                else if c >= 'A' && c <= 'F' { (c as u8) - (0x41 - 10) }
                else { fail ~"bad hex character"; };

            if i % 2 == 0 {
                unsafe {
                    vec::push(&mut vec, nibble << 4);
                }
            }
            else {
                vec[i/2] |= nibble;
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test() {

        assert [05u8, 0xffu8, 0x00u8, 0x59u8].to_hex() == ~"05FF0059";

        assert "00FFA9D1F5".from_hex() == ~[0, 0xff, 0xa9, 0xd1, 0xf5];

        assert "00FFA9D1F5".from_hex().to_hex() == ~"00FFA9D1F5";
    }


}