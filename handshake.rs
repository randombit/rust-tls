
use version::*;

struct ClientHello {
    version: ProtocolVersion,
    random: ~[u8],
    session_id: ~[u8],
    ciphersuites: ~[u16],
    compression: ~[u8],
}

struct ServerHello {
    version: ProtocolVersion,
    random: ~[u8],
    session_id: ~[u8],
    ciphersuite: u16,
    compression: u8,
}

enum HandshakeMessage {
    ClientHello(ClientHello),
    ServerHello(ServerHello),
}

impl HandshakeMessage {

    static fn deserialize(typecode: u8, buf: ~[u8]) -> HandshakeMessage {

        match typecode {
            1 => ClientHello(ClientHello::deserialize(buf)),
            2 => ServerHello(ServerHello::deserialize(buf)),
            _ => fail fmt!("Unknown message %u", typecode as uint)

        }
    }

}

impl ClientHello {

    static fn new(version: NamedProtocolVersion) -> ClientHello {
        let ch: ClientHello = ClientHello {
            version: ProtocolVersion::named(version),
            random: util::hello_random(),
            session_id: ~[],
            ciphersuites: ~[0xfffeu16, 0x1234u16],
            compression: ~[0u8]
        };

        ch
    }

    static fn deserialize(buf: ~[u8]) -> ClientHello {
        let r = packetio::Reader::new(buf);

        ClientHello {
            version: ProtocolVersion { major: r.get_u8(), minor: r.get_u8() },
            random: r.get_u8_fixed(32),
            session_id: r.get_u8_range(1, 0, 32),
            ciphersuites: r.get_u16_range(2, 1, 32767),
            compression: r.get_u8_range(1, 1, 255)
        }
    }

    fn serialize(&self) -> ~[u8] {
        let w = packetio::Writer::new();

        w.put_u8(self.version.major);
        w.put_u8(self.version.minor);
        w.put_u8_fixed(self.random, 32);
        w.put_u8_range(self.session_id, 1);
        w.put_u16_range(self.ciphersuites, 2);
        w.put_u8_range(self.compression, 1);

        w.result()
    }
}

impl ServerHello {

    static fn new(ch: ClientHello, version: ProtocolVersion) -> ServerHello {
        let sh: ServerHello = ServerHello {
            version: version,
            random: util::hello_random(),
            session_id: ~[],
            ciphersuite: 0x008Au16,
            compression: 0x00u8
        };

        sh
    }

    static fn deserialize(buf: ~[u8]) -> ServerHello {
        let r = packetio::Reader::new(buf);

        ServerHello {
            version: ProtocolVersion { major: r.get_u8(), minor: r.get_u8() },
            random: r.get_u8_fixed(32),
            session_id: r.get_u8_range(1, 0, 32),
            ciphersuite: r.get_u16(),
            compression: r.get_u8()
        }
    }

    fn serialize(&self) -> ~[u8] {
        let w = packetio::Writer::new();

        w.put_u8(self.version.major);
        w.put_u8(self.version.minor);
        w.put_u8_fixed(self.random, 32);
        w.put_u8_range(self.session_id, 1);
        w.put_u16(self.ciphersuite);
        w.put_u8(self.compression);

        w.result()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {

        use crypto::hex::ToHex;

        let ch = ClientHello::new(TLS_V12);

        io::println(fmt!("%?", ch));

        let bytes = ch.serialize();

        io::println(fmt!("%s", bytes.to_hex()));

        let ch2 = ClientHello::deserialize(bytes);

        io::println(fmt!("%?", ch2));

    }

}
