rust-tls
===========

The intent of this package is to eventually provide a fully native
Rust implementation of the TLS protocol. It uses rust-crypto
(https://github.com/erickt/rustcrypto/) to access OpenSSL's
implementation of basic primitives like AES, SHA-1, and RSA.

The reasoning is that while implementing the entire TLS protocol in a
memory unsafe language is quite difficult, OpenSSL's implementations
of the basic crypto operations are very well optimized and often
constant time.
