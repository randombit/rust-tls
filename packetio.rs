
pub fn make_u16(x0: u8, x1: u8) -> u16 {
    ((x0 as u16) << 8) | (x1 as u16)
}

pub fn make_u32(x0: u8, x1: u8, x2: u8, x3: u8) -> u32 {
    ((x0 as u32) << 24) |
    ((x1 as u32) << 16) |
    ((x2 as u32) << 8) |
     (x3 as u32)
}

pub struct Reader {
    buf: ~[u8],
    mut offset: uint
}

impl Reader {

    static fn new(buf: ~[u8]) -> Reader {
        Reader { buf: buf, offset: 0 }
    }

    fn remaining(&self) -> uint {
        self.buf.len() - self.offset
    }

    fn assert_available(&self, n: uint) {
        assert n <= self.remaining();
    }

    fn get_u8(&self) -> u8 {
        self.assert_available(1);
        let val = self.buf[self.offset];
        self.offset += 1;
        val
    }

    fn get_u16(&self) -> u16 {
        self.assert_available(2);
        let val = make_u16(self.buf[self.offset],
                           self.buf[self.offset+1]);
        self.offset += 2;
        val
    }

    fn get_u32(&self) -> u32 {
        self.assert_available(4);
        let val = make_u32(self.buf[self.offset],
                           self.buf[self.offset+1],
                           self.buf[self.offset+2],
                           self.buf[self.offset+3]);
        self.offset += 4;
        val
    }

    fn get_u8_fixed(&self, n: uint) -> ~[u8] {
        self.assert_available(n);
        let val = self.buf.slice(self.offset, self.offset + n);
        self.offset += n;
        val
    }

    priv fn get_length_field(&self, tagsize: uint) -> uint {
        match tagsize {
            1 => self.get_u8() as uint,
            2 => self.get_u16() as uint,
            _ => fail ~"Bad length field"
        }
    }

    priv fn get_elems(&self, tagsize: uint, valsize: uint, min_elems: uint, max_elems: uint) -> uint {
        let byte_len = self.get_length_field(tagsize);

        assert byte_len % valsize == 0;

        let num_elems = byte_len / valsize;

        assert num_elems >= min_elems;
        assert num_elems <= max_elems;

        num_elems
    }

    fn get_u8_range(&self, tagsize: uint, min_elems: uint, max_elems: uint) -> ~[u8] {
        let elems = self.get_elems(tagsize, 1, min_elems, max_elems);

        self.assert_available(elems);

        self.buf.slice(self.offset, self.offset + elems)
    }

    fn get_u16_range(&self, tagsize: uint, min_elems: uint, max_elems: uint) -> ~[u16] {
        let elems = self.get_elems(tagsize, 2, min_elems, max_elems);

        self.assert_available(2*elems);

        let bits = self.buf.slice(self.offset, self.offset + 2*elems);

        /*
        This seems so awkward but I can't find a good way to express
        it in vec:: operations
        */

        let res : ~[mut u16] = vec::to_mut(vec::from_elem(elems, 0u16));

        uint::range(0, bits.len(), |i|{
            res[i/2] |= (bits[i] as u16) << (8-8*(i % 2));
            true
        });

        self.offset += 2*elems;

        vec::from_mut(res)
    }
}

pub struct Writer {
    mut buf: ~[u8],
}

impl Writer {

    static fn new() -> Writer {
        Writer { buf: ~[] }
    }

    fn put_u8(&self, x: u8) {
        self.buf.push(x);
    }

    fn put_u16(&self, x: u16) {
        self.buf.push((x >> 8) as u8);
        self.buf.push((x & 0xFF) as u8);
    }

    fn put_u8_fixed(&self, input: &[const u8], sz: uint) {
        assert input.len() == sz;
        self.buf.push_all(input);
    }

    priv fn put_tag(tag_size: uint, bytes: uint) {
        match tag_size {
            1 => self.put_u8(bytes as u8),
            2 => self.put_u16(bytes as u16),
            _ => fail ~"Unknown tag size"
        }
    }

    fn put_u8_range(&self, input: &[const u8], tag_size: uint) {
        self.put_tag(tag_size, input.len());

        self.buf.push_all(input);
    }

    fn put_u16_range(&self, input: &[const u16], tag_size: uint) {
        self.put_tag(tag_size, 2*input.len());

        // this seems unrustish
        uint::range(0, input.len(), |x| { self.put_u16(input[x]); true });
    }

    fn result(&self) -> ~[u8] { copy self.buf }

}
