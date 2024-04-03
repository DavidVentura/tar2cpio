use cpio::{newc, NewcBuilder};
use std::io::prelude::*;
use tar::Archive;

pub fn convert<R: Read, W: Write>(reader: R, mut writer: W) -> std::io::Result<()> {
    let mut ar = Archive::new(reader);
    let entries = ar.entries()?;

    let mut ino = 0;
    for entry in entries {
        let mut entry = entry?;
        let header = entry.header();
        let builder = NewcBuilder::new(entry.path()?.to_str().unwrap())
            .uid(header.uid()? as u32)
            .gid(header.gid()? as u32)
            .mode(header.mode()?)
            .mtime(header.mtime()? as u32)
            .ino(ino);

        let len = entry.size() as u32;
        let mut fp = builder.write(&mut writer, len);
        std::io::copy(&mut entry, &mut fp)?;
        fp.finish()?;

        ino += 1;
    }
    newc::trailer(writer)?;
    Ok(())
}
