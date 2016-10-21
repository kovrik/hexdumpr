use std::cmp;

pub fn print_hexdump(data: &[u8], offset: usize, display: char, bytes: usize) {
    let mut address = 0;
    while address <= data.len() {
        let end = cmp::min(address + 16, data.len());
        print_line(&data[address..end], address + offset, display, bytes);
        address = address + 16;
    }
}

fn print_line(line: &[u8], address: usize, display: char, bytes: usize) {
    // print address
    print!("\n{:08x}:", address);
    let words = if (line.len() % bytes) == 0 {
        line.len() / bytes
    } else {
        (line.len() / bytes) + 1
    };
    for b in 0..words {
        let word = match bytes {
            1 => line[b] as u16,
            _ => {
                if line.len() == bytes*b + 1 {
                    u16::from_be(((line[bytes * b] as u16) << 8) + 0)
                } else {
                    u16::from_be(((line[bytes * b] as u16) << 8) + (line[bytes * b + 1] as u16))
                }
            },
        };
        match display {
            'b' => print!(" {:03o}",  word),
            'c' => { match ((word as u8) as char).is_control() {
                        false => print!(" {:03}", (word as u8) as char),
                        _     => print!(" "),
                     }
                },
            'C' => print!(" {:02x}",  word),
            'x' => print!(" {:04x}",  word),
            'o' => print!(" {:06o} ", word),
            'd' => print!("  {:05} ", word),
            _   => print!(" {:04x}",  word),
        }
    }

    if display != 'c' {
        if (line.len() % 16) > 0 {
            // align
            let mut words_left = (16 - line.len()) / bytes;
            if (line.len() % 2) > 0 {
                words_left = words_left + 1;
            }
            let word_size = match display {
                'b' => 4,
                'c' => 4,
                'C' => 3,
                'x' => 5,
                'o' => 8,
                'd' => 8,
                _   => 5,
            };
            for _ in 0..word_size * words_left {
                print!(" ");
            }
        }

        print!("  ");
        for c in line {
            // replace all control chars with dots
            if (*c as char).is_control() { 
                print!(".");
            } else {
                print!("{}", (*c as char));
            }
        }
    }
}