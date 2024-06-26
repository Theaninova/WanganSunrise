use binrw::BinRead;
use serde::Serialize;

use crate::dist_cell::EnmaDistCell;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaLaneCell {
    pub dist: f32,
    pub unk2: i32,
    pub unk3: i32,
    pub unk4: f32,
    pub left: [f32; 2],
    pub right: [f32; 2],
}

impl EnmaDistCell for EnmaLaneCell {
    fn dist(&self) -> f32 {
        self.dist
    }

    fn size() -> u64 {
        0x20
    }
}
