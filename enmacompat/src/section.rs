use binrw::BinRead;
use serde::Serialize;

use crate::windows_pe::WindowsPEFile;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaSection {
    /// The distance along the path
    pub distance: f32,
    /// The position of the node in world space
    pub position: [f32; 2],
    /// The normal of the node
    pub normal: [f32; 2],
    /// The left wall, where the wall point is `position + normal * left`
    pub left: f32,
    /// The right wall, where the wall point is `position + normal * right`
    pub right: f32,
    /// The z component of the position
    pub height: f32,
    pub unk1: f32,
    pub unk2: f32,
}

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaBankCell {
    /// The index of the section this bank belongs to
    pub section: i32,
    /// The width of the bank (it's unclear wether this is horizontal or diagonal)
    pub width: f32,
    /// The bank (it's unclear if this is a rotation in radians or a factor)
    pub bank: f32,
    /// If false then this bank cell is not connected to the previous one
    #[br(map = |x: u32| x == 1)]
    pub connect: bool,
}

pub fn read_bank_cell(
    file: &WindowsPEFile,
    mut address: u64,
    section_count: i32,
) -> Result<Vec<EnmaBankCell>, std::io::Error> {
    let mut result = vec![];
    loop {
        let bank = file.read_addr::<EnmaBankCell>(address)?;
        if bank.section >= section_count {
            break;
        }
        result.push(bank);
        address += 0x10;
    }

    Ok(result)
}
