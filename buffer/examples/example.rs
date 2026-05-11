extern crate libtw2_buffer;

use libtw2_buffer::ReadBuffer;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let filename = env::temp_dir().join("rust_buffer_test.txt");
    {
        let mut f = File::create(&filename)?;
        f.write_all(&[1; 256])?;
    }
    {
        let mut contents = Vec::with_capacity(1);
        let mut f = File::open(&filename)?;
        loop {
            let len = f.read_buffer(&mut contents)?.len();
            println!("{:3} {:3}/{:3}", len, contents.len(), contents.capacity());
            if len == 0 {
                break;
            }
            if contents.len() == contents.capacity() {
                let len = contents.len();
                contents.reserve(len);
            }
        }
    }
    Ok(())
}
