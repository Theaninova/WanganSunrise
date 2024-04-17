use binrw::BinRead;
use serde::Serialize;

use crate::dist_cell::EnmaDistCell;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaZebraCell {
    pub dist: f32,
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: i32,
}

impl EnmaDistCell for EnmaZebraCell {
    fn dist(&self) -> f32 {
        self.dist
    }

    fn size() -> u64 {
        0x10
    }
}
