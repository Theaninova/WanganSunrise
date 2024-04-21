use std::io::{BufReader, Cursor, Read, Seek};

use binrw::{
    binrw,
    meta::{EndianKind, ReadEndian},
    BinRead, BinResult, Endian,
};
use flate2::bufread::GzDecoder;
use formats::{nut::NuNut, xmd::NuXmd};

pub mod formats;

#[derive(Debug)]
pub enum NuFile {
    Plain(Nu),
    Gz(Nu),
}

impl ReadEndian for NuFile {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl BinRead for NuFile {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut signature = [0u8; 2];
        reader.read_exact(&mut signature)?;
        reader.seek(std::io::SeekFrom::Start(0))?;
        if signature == [0x1F, 0x8B] {
            let mut reader = BufReader::new(reader);
            let mut decoder = GzDecoder::new(&mut reader);
            let mut buf = Vec::new();
            decoder.read_to_end(&mut buf)?;
            Ok(NuFile::Gz(Nu::read_options(
                &mut Cursor::new(buf),
                endian,
                args,
            )?))
        } else {
            Ok(NuFile::Plain(Nu::read_options(reader, endian, args)?))
        }
    }
}

#[binrw]
#[derive(Debug)]
pub enum Nu {
    Nut(NuNut),
    Xmd(NuXmd),
}
