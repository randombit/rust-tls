
struct Ciphersuite {
    code: u16,

    kex_algo: ~str,
    sig_algo: ~str,
    cipher: ~str,
    cipher_keylen: u8,
    mac: ~str
}

impl Ciphersuite {

    static fn new(suite: u16, kex_algo: ~str, sig_algo: ~str,
                  cipher: ~str, cipher_keylen: u8, mac: ~str) -> Ciphersuite {

        Ciphersuite {
            code: suite,
            kex_algo: kex_algo,
            sig_algo: sig_algo,
            cipher: cipher,
            cipher_keylen: cipher_keylen,
            mac: mac }
    }

    static fn from_code(suite: u16) -> Ciphersuite {
        match suite {
            // probably this should be a macro (or generated as in botan)
            0x002f => { Ciphersuite::new(0x002f, ~"RSA", ~"RSA", ~"AES", 16, ~"SHA1") },

            0x008A => { Ciphersuite::new(0x008A, ~"PSK", ~"PSK", ~"RC4", 16, ~"SHA1") },

            _ => { fail(~"No such ciphersuite") }
        }
    }
}

impl Ciphersuite: ToStr {
    pure fn to_str() -> ~str {
        let mut out: ~str = ~"TLS_";

        out += if self.kex_algo != ~"RSA" { self.kex_algo + ~"_" } else { ~"" };

        out += self.sig_algo + "_WITH_";

        out += match (self.cipher, self.cipher_keylen) {
            (~"AES", 16) => ~"AES_128",
            (~"AES", 32) => ~"AES_256",
            (~"RC4", 16) => ~"RC4_128",
            _ => fail ~"Unknown cipher"
        } + ~"_";

        out += match self.mac {
            ~"SHA1" => ~"SHA",
            ~"SHA256" => ~"SHA256",
            _ => fail ~"Unknown mac"
        };

        out


    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let psk = Ciphersuite::from_code(0x008A);

        io::println(fmt!("%?", psk));
    }

}