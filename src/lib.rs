// FIXME byte order

pub fn print_hexdump(data: &[u8], offset: usize, end: usize) {
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
            print_line(&line);
        }
    }
    if (line.len() % 16) != 0 {
        if line.len() % 2 == 1 {
            line.push(0);
        }
        print_line(&line);
    }
}

fn print_line(line: &Vec<u8>) {
    for b in 0..line.len() / 2 {
        let word = ((line[2*b] as u16) << 8) + (line[2*b + 1] as u16);
        print!(" {:04x}", word);
    }

    // align
    let padding = (16 - line.len()) / 2;
    for _ in 0..padding {
        print!("     ");
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