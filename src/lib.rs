// FIXME byte order

pub fn print_hexdump(data: &[u8], offset: usize, end: usize, display: char, bytes: usize) {
    let mut lines = 0;
    let mut line = Vec::new();
    let data = &data[offset..end];
    for b in data {
        if (line.len() % 16) == 0 {
            // print offset
            print!("\n{:08x}:", lines * 16 + offset);
            lines = lines + 1;
            line.clear();
        }
        line.push(*b);
        if (line.len() % 16) == 0 {
            print_line(&line, display, bytes);
        }
    }
    if (line.len() % 16) != 0 {
        if line.len() % 2 == 1 {
            line.push(0);
        }
        print_line(&line, display, bytes);
    }
}

fn print_line(line: &Vec<u8>, display: char, bytes: usize) {
    for b in 0..line.len() / bytes {
        let word = match bytes {
            1 => line[b] as u16,
            _ => ((line[2*b] as u16) << 8) + (line[2*b + 1] as u16),
        };
        match display {
            'x' => print!(" {:04x}",  word),
            'o' => print!(" {:06o} ", word),
            'd' => print!("  {:05} ", word),
            'b' => print!(" {:03o}",  word),
            'C' => print!(" {:02x}",  word),
            'c' => { match word < 32 {
                        false => print!(" {:03}",  (word as u8) as char),
                        _     => print!(" "),
                     }
                },
            _   => print!(" {:04x}", word),
        }
    }

    // align
    if display != 'c' {
        let padding = match display {
            'b' => ((16 - line.len())) * 4,
            'c' => ((16 - line.len())) * 4,
            'C' => ((16 - line.len())) * 3,
            'x' => ((16 - line.len())  / 2) * 5,
            'o' => ((16 - line.len())  / 2) * 8,
            'd' => ((16 - line.len())  / 2) * 8,
            _   => ((16 - line.len())  / 2) * 5,
        };
        for _ in 0..padding {
            print!(" ");
        }

        print!("  ");
        for c in line {
            // replace all chars less than SPACE with dots
            // FIXME Utf8?
            if *c < 32 {
                print!(".");
            } else {
                print!("{}", *c as char);
            }
        }
    }
}