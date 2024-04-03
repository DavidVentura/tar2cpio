use std::fs::File;
use tar2cpio::convert;

pub fn main() -> std::io::Result<()> {
    let inf = File::open("outexport.tar")?;
    let outf = File::create("out.cpio")?;
    convert(inf, outf)
}
