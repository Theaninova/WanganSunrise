use crate::windows_pe::WindowsPEFile;
use binrw::BinRead;
use serde::Serialize;

#[derive(Debug, BinRead, Serialize)]
pub struct EnmaLane {
    pub dist: f32,
    pub unk2: i32,
    pub unk3: i32,
    pub unk4: f32,
    pub left: [f32; 2],
    pub right: [f32; 2],
}

pub fn read_lane(
    file: &WindowsPEFile,
    mut address: u64,
    max_dist: f32,
) -> Result<Vec<EnmaLane>, std::io::Error> {
    let mut result = vec![];
    loop {
        let lane = file.read_addr::<EnmaLane>(address)?;
        let dist = lane.dist;
        result.push(lane);
        if dist >= max_dist {
            break;
        }
        address += 0x20;
    }

    Ok(result)
}
