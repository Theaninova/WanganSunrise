use binrw::BinRead;
use serde::Serialize;

use crate::dist_cell::EnmaDistCell;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaNonGuardCell {
    pub dist: f32,
    pub unk1: f32,
    // TODO: Is this a boolean?
    pub guard: i32,
}

impl EnmaDistCell for EnmaNonGuardCell {
    fn dist(&self) -> f32 {
        self.dist
    }

    fn size() -> u64 {
        0xc
    }
}
