use binrw::{args, binrw, BinRead, BinResult};
use modular_bitfield::{bitfield, specifiers::B9};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub enum NuNut {
    #[brw(magic = b"NTP3", big)]
    NTP3(NuNutData),
    #[brw(magic = b"NTWD", little)]
    NTWD(NuNutData),
    #[brw(magic = b"NTLX", little)]
    NTLX(NuNutData),
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
pub enum NuTextureType {
    DDS = 0,
    GXT = 1,
}

#[binrw]
#[brw(big, repr = u16)]
#[derive(Debug)]
pub enum NuPixelFormat {
    DXT1 = 0x00,
    DXT3 = 0x01,
    DXT5 = 0x02,
    RGB16 = 0x08,
    RGBA16 = 0x0C,
    RGBA = 0x0E,
    ABGR = 0x0F,
    RGBA2 = 0x10,
    RGBA1 = 0x11,
    RGBA0 = 0x15,
    CompressedRgRGTC2 = 0x16,
}

#[binrw]
#[br(assert(version == 2, "Unsupported version {}", version))]
#[derive(Debug)]
pub struct NuNutData {
    pub version: u16,
    #[br(temp)]
    #[bw(calc(textures.len() as u16))]
    count: u16,
    #[brw(align_before = 0x10)]
    #[br(count = count)]
    pub textures: Vec<NuNutTexture>,
    #[brw(align_before = 0x10)]
    #[br(parse_with = parse_surfaces, args(&textures))]
    pub surfaces: Vec<Vec<u8>>,
}

#[binrw::parser(reader, endian)]
fn parse_surfaces(textures: &Vec<NuNutTexture>) -> BinResult<Vec<Vec<u8>>> {
    let mut surfaces = Vec::with_capacity(textures.len());
    for texture in textures {
        surfaces.push(Vec::<u8>::read_options(
            reader,
            endian,
            args! {count: texture.data_size as usize},
        )?);
    }
    Ok(surfaces)
}

#[binrw]
#[br(assert(header_size == self.header_size()))]
#[br(assert(size == self.size()))]
#[derive(Debug)]
pub struct NuNutTexture {
    #[br(temp)]
    #[bw(calc(self.size()))]
    pub size: u64,
    pub data_size: u32, // TODO: dynamic writing
    #[brw(align_after = 0x10)]
    #[br(temp)]
    #[bw(calc(self.header_size()))]
    pub header_size: u16,

    #[brw(big)]
    #[br(temp, dbg)]
    #[bw(calc(mipmap_sizes.len() as u16))]
    pub mipmap_count: u16,
    pub pixel_format: NuPixelFormat,
    pub width: u16,
    pub height: u16,
    pub texture_type: NuTextureType,
    pub cubemap: NuNutCubemap,
    #[brw(align_before = 0x10)]
    pub data_offset: u32, // TODO: dynamic writing
    #[brw(if(cubemap.is_cubemap()), align_before = 0x10)]
    pub cubemap_sizes: [u16; 4],
    #[brw(align_before = 0x10)]
    #[br(count = mipmap_count)]
    pub mipmap_sizes: Vec<u32>, // TODO: dynamic writing

    #[brw(align_before = 0x10)]
    pub ext: NuNutExt,
    #[brw(align_before = 0x10, align_after = 0x10)]
    pub gidx: NuNutGidx,
}

impl NuNutTexture {
    fn header_size(&self) -> u16 {
        if self.cubemap.is_cubemap() {
            0x70
        } else {
            0x60
        }
    }

    fn size(&self) -> u64 {
        self.data_size as u64 + self.header_size() as u64
    }
}

#[binrw]
#[brw(magic = b"eXt\0")]
#[derive(Debug)]
pub struct NuNutExt {
    pub version: u32,
    pub version2: u32,
    pub unknown: u32,
}

#[binrw]
#[brw(magic = b"GIDX")]
#[derive(Debug)]
pub struct NuNutGidx {
    pub unknown2: u32,
    pub hash_id: u32,
}

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&x| Self::into_bytes(x))]
#[derive(Debug, Clone, Copy)]
pub struct NuNutCubemap {
    #[skip]
    __: B9,
    pub x_plus: bool,
    pub x_minus: bool,
    pub y_plus: bool,
    pub y_minus: bool,
    pub z_plus: bool,
    pub z_minus: bool,
    pub is_cubemap: bool,
}

impl NuNutCubemap {
    pub fn cubemap_count(&self) -> u8 {
        self.x_plus() as u8
            + self.x_minus() as u8
            + self.y_plus() as u8
            + self.y_minus() as u8
            + self.z_plus() as u8
            + self.z_minus() as u8
    }

    pub fn surface_count(&self) -> u8 {
        self.cubemap_count().min(1)
    }
}

#[binrw]
#[derive(Debug)]
pub struct NuNutSurface {}
