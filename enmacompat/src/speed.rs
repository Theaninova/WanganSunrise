use crate::windows_pe::WindowsPEFile;
use binrw::BinRead;
use serde::Serialize;

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

pub fn read_speed_cells(
    file: &WindowsPEFile,
    mut address: u64,
    max_dist: f32,
) -> Result<Vec<EnmaSpeedCell>, std::io::Error> {
    let mut result = vec![];
    loop {
        let speed = file.read_addr::<EnmaSpeedCell>(address)?;
        let dist = speed.dist;
        result.push(speed);
        if dist >= max_dist {
            break;
        }
        address += 0x20;
    }

    Ok(result)
}
