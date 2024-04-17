use serde::{Deserialize, Serialize};

use crate::{area::find_area_array_location, windows_pe::WindowsPEFile};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnmaMeta {
    pub area_array_addr: u64,
    pub area_array_count: i32,
}

impl EnmaMeta {
    /// Performs an analysis from a WindowsPE executable to find specific data locations
    pub fn from_analysis(windows_pe_file: &mut WindowsPEFile) -> Result<Self, std::io::Error> {
        println!("\x1b[34m\x1b[1mSearching for Area Array Location\x1b[0m");
        let area_array_addr = find_area_array_location(windows_pe_file)?.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find Area Array Location",
            )
        })?;

        let mut area_array_count = 1;
        loop {
            let area_array =
                windows_pe_file.read_addr::<u64>(area_array_addr + area_array_count as u64 * 8)?;
            if area_array == 0 {
                break;
            }

            let id = windows_pe_file.read_addr::<u64>(area_array)?;
            if id != area_array_count as u64 {
                break;
            }
            area_array_count += 1;
        }

        println!(
            "\x1b[32m\x1b[1mArea Array\x1b[0m: \x1b[4m{:#x} ({} Areas)\x1b[0m",
            area_array_addr,
            area_array_count - 1
        );

        Ok(Self {
            area_array_addr,
            area_array_count,
        })
    }
}
