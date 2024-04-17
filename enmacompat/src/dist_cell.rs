use crate::windows_pe::WindowsPEFile;
use binrw::BinRead;
use serde::Serialize;

pub trait EnmaDistCell {
    fn dist(&self) -> f32;
    fn size() -> u64;
}

impl EnmaDistCell for f32 {
    fn dist(&self) -> f32 {
        *self
    }

    fn size() -> u64 {
        4
    }
}

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaDistTableCell {
    pub dist: f32,
    pub unk1: f32,
    pub flag: i32,
}

impl EnmaDistCell for EnmaDistTableCell {
    fn dist(&self) -> f32 {
        self.dist
    }

    fn size() -> u64 {
        0xc
    }
}

pub fn read_dist_cells<'a, T>(
    file: &WindowsPEFile,
    mut address: u64,
    max_dist: f32,
    include_last: bool,
) -> Result<Vec<T>, std::io::Error>
where
    T: BinRead + EnmaDistCell,
    T::Args<'a>: Default,
{
    let mut result = vec![];
    loop {
        let cell = file.read_addr::<T>(address)?;
        if cell.dist() >= max_dist {
            if include_last {
                result.push(cell);
            }
            break;
        }
        result.push(cell);
        address += T::size();
    }

    Ok(result)
}
