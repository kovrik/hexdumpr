# hexdumpr
hexdump in Rust

Usage
---

Options:

```
-n length Interpret only length bytes of input.
-s offset Skip offset bytes from the beginning of the input.

-b        One-byte octal display.
-c        One-byte character display.
-C        Canonical hex display.
-d        Two-byte decimal display.
-o        Two-byte octal display.
-x        Two-byte hexadecimal display.
```

```
> hexdumpr
Usage: hexdumpr [-bcCdox][-s offset][-n length] file ...

> hexdumpr src/main.rs

src/main.rs:
00000000: 7573 6520 7374 643a 3a66 733a 3a46 696c  use std::fs::Fil
00000010: 653b 0d0a 7573 6520 7374 643a 3a65 6e76  e;..use std::env
00000020: 3b0d 0a75 7365 2073 7464 3a3a 696f 3a3a  ;..use std::io::
00000030: 7072 656c 7564 653a 3a2a 3b0d 0a0d 0a6d  prelude::*;....m
00000040: 6f64 206c 6962 3b0d 0a75 7365 206c 6962  od lib;..use lib
00000050: 3a3a 2a3b 0d0a 0d0a 2f2f 2054 4f44 4f20  ::*;....// TODO
00000060: 7573 6167 6520 616e 6420 6865 6c70 2069  usage and help i
00000070: 6e73 7472 7563 7469 6f6e 730d 0a2f 2f20  nstructions..//
00000080: 544f 444f 2064 6966 6665 7265 6e74 206f  TODO different o
00000090: 7074 696f 6e73 0d0a 666e 206d 6169 6e28  ptions..fn main(
000000a0: 2920 7b0d 0a20 2020 206c 6574 2066 696c  ) {..    let fil
000000b0: 656e 616d 6520 3d20 656e 763a 3a61 7267  ename = env::arg
000000c0: 7328 292e 6e74 6828 3129 2e75 6e77 7261  s().nth(1).unwra
000000d0: 705f 6f72 2822 7372 632f 6d61 696e 2e72  p_or("src/main.r
000000e0: 7322 2e74 6f5f 7374 7269 6e67 2829 293b  s".to_string());
000000f0: 0d0a 2020 2020 7072 696e 7421 2822 7b7d  ..    print!("{}
00000100: 3a22 2c20 6669 6c65 6e61 6d65 293b 0d0a  :", filename);..
00000110: 0d0a 2020 2020 6c65 7420 6d75 7420 6620  ..    let mut f
00000120: 3d20 4669 6c65 3a3a 6f70 656e 2826 6669  = File::open(&fi
00000130: 6c65 6e61 6d65 292e 6578 7065 6374 2822  lename).expect("
00000140: 556e 6162 6c65 2074 6f20 6f70 656e 2066  Unable to open f
00000150: 696c 6522 293b 0d0a 2020 2020 6c65 7420  ile");..    let
00000160: 6d75 7420 6461 7461 203d 2056 6563 3a3a  mut data = Vec::
00000170: 6e65 7728 293b 0d0a 2020 2020 662e 7265  new();..    f.re
00000180: 6164 5f74 6f5f 656e 6428 266d 7574 2064  ad_to_end(&mut d
00000190: 6174 6129 2e65 7870 6563 7428 2255 6e61  ata).expect("Una
000001a0: 626c 6520 746f 2072 6561 6420 6461 7461  ble to read data
000001b0: 2229 3b0d 0a20 2020 2070 7269 6e74 5f68  ");..    print_h
000001c0: 6578 6475 6d70 2826 6461 7461 293b 0d0a  exdump(&data);..
000001d0: 7d00                                     }.
```