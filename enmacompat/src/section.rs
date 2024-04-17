use binrw::BinRead;
use serde::Serialize;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaSection {
    pub distance: f32,
    pub x: f32,
    pub y: f32,
    pub angle1: f32,
    pub angle2: f32,
    pub left: f32,
    pub right: f32,
    pub height: f32,
    pub unk1: f32,
    pub unk2: f32,
}
