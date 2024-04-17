use std::{fs::File, io::Seek};

use binrw::{io::BufReader, BinRead, BinReaderExt, Endian};

use crate::util::{find_in_data, is_exactly};

#[derive(BinRead)]
#[br(little, magic = b"MZ")]
pub struct WindowsPEHeader {
    pub last_page_size: u16,
    pub pages_in_file: u16,
    pub relocations: u16,
    pub header_size: u16,
    pub min_memory: u16,
    pub max_memory: u16,
    pub initial_ss: u16,
    pub initial_sp: u16,
    pub checksum: u16,
    pub initial_ip: u16,
    pub initial_cs: u16,
    pub relocations_offset: u16,
    pub overlay_number: u16,
    pub reserved: [u16; 4],
    pub oem_id: u16,
    pub oem_info: u16,
    pub reserved2: [u16; 10],
    pub pe_offset: u32,
    #[br(pad_before = pe_offset - 0x40)]
    pub image_nt_headers: WindowsPEImageNTHeaders,
}

#[derive(BinRead)]
#[br(little, magic = b"PE\0\0")]
pub struct WindowsPEImageNTHeaders {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
    #[br(pad_size_to = size_of_optional_header)]
    pub optional_header: WindowsPEOptionalHeader,
    #[br(count = number_of_sections)]
    pub sections: Vec<WindowsPEImageSectionHeader>,
}

#[derive(BinRead)]
#[br(little, magic = b"\x0b\x02")]
pub struct WindowsPEOptionalHeader {
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
}

#[derive(BinRead)]
#[br(little)]
pub struct WindowsPEImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

pub struct WindowsPEFile {
    pub file: File,
    pub header: WindowsPEHeader,
}

impl WindowsPEFile {
    pub fn get_file_address(&self, memory_address: u64) -> Option<u64> {
        let image_base = self.header.image_nt_headers.optional_header.image_base;
        let section = self
            .header
            .image_nt_headers
            .sections
            .iter()
            .find(|section| {
                let virtual_address = section.virtual_address as u64 + image_base;
                memory_address >= virtual_address
                    && memory_address < virtual_address + section.virtual_size as u64
            })?;
        let offset = memory_address - section.virtual_address as u64 - image_base as u64
            + section.pointer_to_raw_data as u64;
        Some(offset)
    }

    pub fn get_memory_address(&self, file_address: u64) -> Option<u64> {
        let image_base = self.header.image_nt_headers.optional_header.image_base;
        let section = self
            .header
            .image_nt_headers
            .sections
            .iter()
            .find(|section| {
                file_address >= section.pointer_to_raw_data as u64
                    && file_address
                        < section.pointer_to_raw_data as u64 + section.size_of_raw_data as u64
            })?;
        let offset = file_address - section.pointer_to_raw_data as u64
            + image_base as u64
            + section.virtual_address as u64;
        Some(offset)
    }

    pub fn find_memory_address_of(&self, target: &[u8]) -> Result<Vec<u64>, std::io::Error> {
        let mut reader = BufReader::new(&self.file);
        reader.seek(std::io::SeekFrom::Start(0))?;
        let file_addr = find_in_data(&mut reader, target)?;
        Ok(file_addr
            .iter()
            .map(|file_addr| self.get_memory_address(*file_addr as u64).unwrap())
            .collect())
    }

    pub fn find_references(&self, address: u64) -> Result<Vec<u64>, std::io::Error> {
        self.find_memory_address_of(&address.to_le_bytes())
    }

    pub fn is_exactly(&self, address: u64, target: &[u8]) -> Result<bool, std::io::Error> {
        let mut reader = BufReader::new(&self.file);
        self.get_file_address(address)
            .map(|file_addr| is_exactly(&mut reader, file_addr as usize, target))
            .unwrap_or(Ok(false))
    }

    pub fn read_addr<'a, T>(&self, address: u64) -> Result<T, std::io::Error>
    where
        T: BinRead,
        T::Args<'a>: Default,
    {
        let mut reader = BufReader::new(&self.file);
        if let Some(file_addr) = self.get_file_address(address) {
            reader.seek(std::io::SeekFrom::Start(file_addr as u64))?;
            reader
                .read_type::<T>(Endian::Little)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Address not found",
            ))
        }
    }
}

impl From<std::fs::File> for WindowsPEFile {
    fn from(file: std::fs::File) -> Self {
        let mut reader = binrw::io::BufReader::new(&file);
        let header = reader.read_ne().unwrap();
        Self { file, header }
    }
}
