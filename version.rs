
use core::cmp::Eq;

enum NamedProtocolVersion {
    SSL_V3,
    TLS_V10,
    TLS_V11,
    TLS_V12
}

pub struct ProtocolVersion {
    major: u8,
    minor: u8
}

impl ProtocolVersion {

    static fn named(version: NamedProtocolVersion) -> ProtocolVersion {
        match version {
            SSL_V3  => ProtocolVersion { major: 3, minor: 0 },
            TLS_V10 => ProtocolVersion { major: 3, minor: 1 },
            TLS_V11 => ProtocolVersion { major: 3, minor: 2 },
            TLS_V12 => ProtocolVersion { major: 3, minor: 3 },
        }
    }

    static fn latest() -> ProtocolVersion {
        ProtocolVersion::named(TLS_V12)
    }

    fn known(&self) -> bool {
        match (self.major, self.minor) {
            (3,0) => true,
            (3,1) => true,
            (3,2) => true,
            (3,3) => true,
            _ => false
        }
    }

    fn best_match(&self) -> ProtocolVersion {
        if self.known() { *self } else { ProtocolVersion::latest() }
    }
}

impl ProtocolVersion: ToStr {

    pure fn to_str() -> ~str {
        match (self.major, self.minor) {
            (3,0) => ~"SSL v3",
            (3,1) => ~"TLS v1.0",
            (3,2) => ~"TLS v1.1",
            (3,3) => ~"TLS v1.2",
            (maj,min) => fmt!("Unknown TLS version %?.%?", maj, min)
        }
    }
}

impl ProtocolVersion: Eq {

    pure fn eq(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major && self.minor == other.minor
    }

    pure fn ne(&self, other: &ProtocolVersion) -> bool { !self.eq(other) }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_version() {
        let version = ProtocolVersion::latest();

        assert version.known();

        io::println(fmt!("latest %s %?", version.to_str(), version.known()));

        let ssl_v3 = ProtocolVersion::named(SSL_V3);

        assert ssl_v3.to_str() == ~"SSL v3";

        assert ssl_v3.major == 3;
        assert ssl_v3.minor == 0;

        let late_version = ProtocolVersion { major: 3, minor: 9 };

        assert late_version.known() == false;

        assert late_version.best_match() == ProtocolVersion::latest();
    }
}