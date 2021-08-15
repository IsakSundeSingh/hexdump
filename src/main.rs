use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
  let filename = std::env::args().nth(1).expect("expected filename");

  let mut file = std::fs::File::open(filename)?;
  let mut buffer = [0; BYTES_PER_LINE];

  let mut pos = 0;
  while file.read_exact(&mut buffer).is_ok() {
    print!("[0x{:08x}] ", pos);
    for byte in &buffer {
      match *byte {
        0x00 => print!(".  "),
        0xff => print!("## "),
        _ => print!("{:02x}", byte),
      }
    }
    println!();
    pos += BYTES_PER_LINE;
  }

  Ok(())
}
