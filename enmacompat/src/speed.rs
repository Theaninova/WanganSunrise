use binrw::BinRead;
use serde::Serialize;

use crate::dist_cell::EnmaDistCell;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaSpeedCell {
    pub dist: f32,
    pub z: f32,
    pub norm: f32,
    pub handle_10: f32,
    pub pow_10: f32,
    pub ps_600: f32,
    pub handle_22: f32,
    pub pow_22: f32,
}

impl EnmaDistCell for EnmaSpeedCell {
    fn dist(&self) -> f32 {
        self.dist
    }

    fn size() -> u64 {
        0x20
    }
}
