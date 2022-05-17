pub trait JsonExtender {
    fn extend_json_str(&mut self, s: &str);
}

static HEX: &[u8; 16] = br#"0123456789abcdef"#;

fn extend_hex_16(buf: &mut Vec<u8>, x: u16) {
    buf.push(HEX[(x>>12&0xF) as usize]);
    buf.push(HEX[(x>>8&0xF) as usize]);
    buf.push(HEX[(x>>4&0xF) as usize]);
    buf.push(HEX[(x>>0&0xF) as usize]);
}

impl JsonExtender for Vec<u8> {
    fn extend_json_str(&mut self, s: &str) {
        let mut tmp = [0u8; 10];
        self.push(b'"');
        for c in s.chars() {
            if c < ' ' {
                self.push(b'\\');
                match c {
                    '\n'=> self.push(b'n'),
                    '\r' => self.push(b'r'),
                    '\t'=> self.push(b'r'),
                    _ => {
                        self.push(b'u');
                        extend_hex_16(self, c as u16);
                    }
                }
            } else if c == '>' || c == '<' || c == '&' {
                self.push(b'\\');
                self.push(b'u');
                extend_hex_16(self, c as u16);
            } else if c == '\\' {
                self.push(b'\\');
                self.push(b'\\');
            } else if c == '"' {
                self.push(b'"');
            } else {
                self.extend(c.encode_utf8(&mut tmp).as_bytes());
            }
        }
        self.push(b'"');
    }
}
