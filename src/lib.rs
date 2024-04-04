use cpio::{newc, NewcBuilder};
use std::io::prelude::*;
use tar::Archive;

pub enum ModeFlags {
    None,
    Symlink,
}

impl From<ModeFlags> for u32 {
    fn from(m: ModeFlags) -> u32 {
        match m {
            ModeFlags::None => 0,
            ModeFlags::Symlink => 0o120000,
        }
    }
}
pub fn convert<R: Read, W: Write>(reader: R, mut writer: W) -> std::io::Result<()> {
    let mut ar = Archive::new(reader);
    let entries = ar.entries()?;

    let mut ino = 0;
    for entry in entries {
        let mut entry = entry?;
        let header = entry.header();
        let mode: u32 = if let Some(_) = entry.link_name()? {
            ModeFlags::Symlink
        } else {
            ModeFlags::None
        }
        .into();
        let builder = NewcBuilder::new(entry.path()?.to_str().unwrap())
            .uid(header.uid()? as u32)
            .gid(header.gid()? as u32)
            .mode(header.mode()? | mode)
            .mtime(header.mtime()? as u32)
            .ino(ino);

        if let Some(l) = entry.link_name()? {
            let target = l.as_os_str().as_encoded_bytes();
            let len = target.len() as u32;
            let mut fp = builder.write(&mut writer, len);
            fp.write(target)?;
            fp.finish()?;
        } else {
            let len = entry.size() as u32;
            let mut fp = builder.write(&mut writer, len);
            std::io::copy(&mut entry, &mut fp)?;
            fp.finish()?;
        }

        ino += 1;
    }
    newc::trailer(writer)?;
    Ok(())
}
