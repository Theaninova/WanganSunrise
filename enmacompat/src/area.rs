use crate::{
    dist_cell::read_dist_cells,
    lane::EnmaLaneCell,
    section::{read_bank_cell, EnmaBankCell, EnmaSection},
    speed::EnmaSpeedCell,
    windows_pe::WindowsPEFile,
    zebra::EnmaZebraCell,
};
use binrw::{BinRead, NullString};
use serde::Serialize;

pub fn find_area_array_location(file: &mut WindowsPEFile) -> Result<Option<u64>, std::io::Error> {
    for area_name_addr in file.find_memory_address_of(b"OsDojima")? {
        println!("Possible Area Name: {:#x}", area_name_addr);

        for area_addr in file.find_references(area_name_addr)? {
            let area_addr = area_addr - 8;
            if !file.is_exactly(area_addr, &1_u64.to_le_bytes())? {
                continue;
            }

            println!("Possible Area: {:#x}", area_addr);

            for area_array_addr in file.find_references(area_addr)? {
                let area_array_addr = area_array_addr - 8;
                if !file
                    .is_exactly(area_array_addr, &0_u64.to_le_bytes())
                    .unwrap_or(false)
                {
                    continue;
                }

                println!("Possible Area Array: {:#x}", area_array_addr);

                if (1..10).all(|i| {
                    let reference = area_array_addr + i * 8;
                    file.read_addr::<u64>(reference)
                        .map(|addr| {
                            println!("Checking Array Member {} ({:#x})", i, addr);
                            file.is_exactly(addr, &(i as u64).to_le_bytes())
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                }) {
                    return Ok(Some(area_array_addr));
                }
            }
        }
    }

    Ok(None)
}

pub fn read_area(file: &WindowsPEFile, address: u64) -> Result<EnmaArea, std::io::Error> {
    let area = file.read_addr::<EnmaAreaRaw>(address)?;
    Ok(EnmaArea {
        id: area.id,
        name: file.read_addr::<NullString>(area.name_addr)?.to_string(),
        stage_id: area.stage_id,
        related_area_addr: area.related_area_addr,
        max_dist: area.max_dist,
        sections: (0..area.section_count)
            .map(|i| file.read_addr::<EnmaSection>(area.sections_addr + i as u64 * 0x28))
            .collect::<Result<_, _>>()?,
        bank_left: read_bank_cell(file, area.bank_left_addr, area.section_count)?,
        bank_right: read_bank_cell(file, area.bank_right_addr, area.section_count)?,
        zebra_left: read_dist_cells(file, area.zebra_left_addr, area.max_dist, false)?,
        zebra_right: read_dist_cells(file, area.zebra_left_addr, area.max_dist, false)?,
        gaps: read_dist_cells(file, area.gaps_addr, area.max_dist, false)?,
        speed: read_dist_cells(file, area.speed_addr, area.max_dist, true)?,
        lane: read_dist_cells(file, area.lane_addr, area.max_dist, true)?,
    })
}

#[derive(Debug, Serialize)]
pub struct EnmaArea {
    /// A unique identifier for the area
    pub id: u64,
    /// The name of the area
    pub name: String,
    pub stage_id: u64,
    pub related_area_addr: [u64; 4],
    pub max_dist: f32,
    pub sections: Vec<EnmaSection>,
    pub bank_left: Vec<EnmaBankCell>,
    pub bank_right: Vec<EnmaBankCell>,
    pub zebra_left: Vec<EnmaZebraCell>,
    pub zebra_right: Vec<EnmaZebraCell>,
    pub gaps: Vec<f32>,
    // TODO: Non Guard Left
    // TODO: Non Guard Right
    pub speed: Vec<EnmaSpeedCell>,
    pub lane: Vec<EnmaLaneCell>,
}

#[derive(BinRead, Debug)]
#[br(little)]
pub struct EnmaAreaRaw {
    pub id: u64,
    pub name_addr: u64,
    pub stage_id: u64,

    pub related_area_addr: [u64; 4],

    #[br(pad_before = 8, pad_after = 4)]
    pub max_dist: f32,
    pub sections_addr: u64,
    pub section_count: i32,

    pub unk1: [f32; 3],

    pub bank_left_addr: u64,
    #[br(pad_before = 8)]
    pub bank_right_addr: u64,
    #[br(pad_before = 8)]
    pub zebra_left_addr: u64,
    #[br(pad_before = 8)]
    pub zebra_right_addr: u64,
    #[br(pad_before = 8)]
    pub gaps_addr: u64,
    #[br(pad_before = 8)]
    pub non_guard_left_addr: u64,
    #[br(pad_before = 8)]
    pub non_guard_right_addr: u64,
    #[br(pad_before = 8)]
    pub speed_addr: u64,
    #[br(pad_before = 8)]
    pub lane_addr: u64,
    #[br(pad_before = 8)]
    pub other_addr: u64,
    #[br(pad_before = 8)]
    pub non_lane_change_addr: u64,
    #[br(pad_before = 8)]
    pub signs_addr: u64,
    #[br(pad_before = 8)]
    pub notices_addr: u64,
    #[br(pad_before = 8)]
    pub watches_addr: u64,
    #[br(pad_before = 8)]
    pub on_comers_addr: u64,
    #[br(pad_before = 8)]
    pub pillers_addr: u64,
}
