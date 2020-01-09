// font-kit/src/id.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A globally-unique identifier for fonts.

use byteorder::{BigEndian, ReadBytesExt};
use crc::crc32;
use std::fmt::{self, Debug, Display, Formatter};

pub(crate) const OPENTYPE_TABLE_TAG_HEAD: u32 = 0x68656164;    // 'head'

#[derive(Clone)]
pub struct FontId {
    /// A name describing the font. This is usually the PostScript name, but if the font does not
    /// have a PostScript name it may be some other kind of name.
    pub name: String,
    /// The revision number per the OpenType specification.
    pub revision: FontRevision,
    /// A CRC-32C (Castagnoli polynomial) hash of the `head` table.
    pub hash: u32,
    /// Various flags.
    pub flags: FontIdFlags,
}

impl FontId {
    pub(crate) fn from_opentype_head_table(name: String,
                                           head_table_data: &[u8],
                                           name_is_postscript: bool)
                                           -> FontId {
        let mut flags = FontIdFlags::IS_OPENTYPE;
        if name_is_postscript {
            flags.insert(FontIdFlags::HAS_POSTSCRIPT_NAME);
        }

        let revision = match (&head_table_data[4..]).read_i32::<BigEndian>() {
            Ok(revision) => FontRevision(revision),
            Err(_) => FontRevision(0),
        };

        let hash = crc32::checksum_castagnoli(head_table_data);

        FontId { name, revision, hash, flags }
    }
}

impl Debug for FontId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}/{:08x}", self.name, self.revision, self.hash)
    }
}

impl Display for FontId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// The revision number of a font, as specified in the `head` table of OpenType.
#[derive(Clone, Copy, PartialEq)]
pub struct FontRevision(pub i32);

impl FontRevision {
    /// Returns the major version component.
    #[inline]
    pub fn major(self) -> i16 {
        (self.0 >> 16) as i16
    }

    /// Returns the minor version component.
    #[inline]
    pub fn minor(self) -> i16 {
        self.0 as i16
    }
}

impl Debug for FontRevision {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.major())?;
        if self.minor() != 0 {
            write!(f, ".{}", self.minor())?
        }
        Ok(())
    }
}

impl Display for FontRevision {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

bitflags! {
    /// Flags that describe how the font ID fields were obtained.
    pub struct FontIdFlags: u8 {
        const HAS_POSTSCRIPT_NAME = 0x01;
        const IS_OPENTYPE = 0x02;
    }
}
