use binrw::binrw;

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
pub enum NuXmdListCounts {
    LmbTexturesResources = 1,
    PosLenId = 3,
}

#[binrw]
#[brw(little, magic = b"XMD\0001\0")]
#[derive(Debug)]
pub struct NuXmd {
    pub layout: NuXmdListCounts,
    pub count: u32,
    #[br(count = count)]
    #[brw(align_after = 0x10)]
    pub positions: Vec<u32>,
    #[br(count = count)]
    #[brw(align_after = 0x10)]
    pub lengths: Vec<u32>,
    #[br(count = count)]
    #[brw(align_after = 0x10)]
    pub ids: Vec<u32>,
}
