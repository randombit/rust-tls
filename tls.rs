extern mod crypto;



enum ProtocolVersion {
    TlsVersion10 = 0x0301,
    TlsVersion11 = 0x0302,
    TlsVersion12 = 0x0303
}

struct Extension {

}

struct ClientHelloData {
    version: ProtocolVersion,
    random: ~[u8],
    session_id: ~[u8],
    ciphersuites: ~[u16],
    compression: ~[u8],
    extensions: ~[Extension]
}

impl ClientHelloData {

    static fn new(version: ProtocolVersion) -> ClientHelloData {
        let ch: ClientHelloData = ClientHelloData {
            version: version,
            random: crypto::rand::rand_bytes(16),
            session_id: ~[],
            ciphersuites: ~[],
            compression: ~[0u8]
        };

        ch
    }

    static fn deserialize(data: ~[u8]) -> ClientHelloData {
        let ch: ClientHelloData = ClientHelloData {
            version: TlsVersion12,
            random: crypto::rand::rand_bytes(16),
            session_id: ~[],
            ciphersuites: ~[],
            compression: ~[0u8]
        };

        ch

    }

    fn serialize(&self) -> ~[u8] {
        ~[]
    }
}

enum HandshakeMessage {
    ClientHello(ClientHelloData)
}

impl HandshakeMessage {

    fn serialize(&self) -> ~[u8] {
        match self {
            &ClientHello(ref data) => data.serialize()
        }
    }

    fn typecode(&self) -> u8 {
        match self {
            &ClientHello(_) => 1
        }
    }
}

fn main() {
    io::println("morning?");

    let ch = ClientHelloData::new(TlsVersion12);

    io::println(fmt!("%?", ch));

}
