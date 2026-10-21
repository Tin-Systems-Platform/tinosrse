#![allow(dead_code)]

use core::ptr;

#[repr(C)]
pub struct BootloaderInfoResponse {
    pub revision: u64,
    pub name: *const u8,
    pub version: *const u8,
}

#[repr(C)]
pub struct HhdmResponse {
    pub revision: u64,
    pub offset: u64,
}

#[repr(C)]
pub struct FramebufferResponse {
    pub revision: u64,
    pub framebuffer_count: u64,
    pub framebuffers: *const *const Framebuffer,
}

#[repr(C)]
pub struct Framebuffer {
    pub address: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
    pub edid_size: u64,
    pub edid: *mut u8,
    pub mode_count: u64,
    pub modes: *const *const VideoMode,
}

#[repr(C)]
pub struct VideoMode {
    pub pitch: u64,
    pub width: u64,
    pub height: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
}

#[repr(C)]
pub struct MemmapResponse {
    pub revision: u64,
    pub entry_count: u64,
    pub entries: *const *const MemmapEntry,
}

#[repr(C)]
pub struct MemmapEntry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u64,
}

#[repr(C)]
pub struct File {
    pub revision: u64,
    pub address: *const u8,
    pub size: u64,
    pub path: *const u8,
    pub string: *const u8,
    pub media_type: u32,
    pub unused: u32,
    pub tftp_ipv4: [u8; 4],
    pub tftp_port: u32,
    pub partition_index: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: [u8; 16],
    pub gpt_part_uuid: [u8; 16],
    pub part_uuid: [u8; 16],
}

#[repr(C)]
pub struct ExecutableFileResponse {
    pub revision: u64,
    pub executable_file: *const File,
}

#[repr(C)]
pub struct RResponse {
    pub revision: u64,
    pub address: u64,
}

#[repr(C)]
pub struct SmbiosResponse {
    pub revision: u64,
    pub entry_32: *const u8,
    pub entry_64: *const u8,
}

#[repr(C)]
pub struct FirmwareResponse {
    pub revision: u64,
    pub firmware_type: u64,
}

#[repr(C)]
pub struct DateResponse {
    pub revision: u64,
    pub timestamp: i64,
}

#[repr(C)]
struct Request {
    id: [u64; 4],
    revision: u64,
    response: *const (),
}

macro_rules! request {
    ($name:ident, $response:ty, [$($id:expr),*]) => {
        #[unsafe(link_section = ".limine_requests")]
        #[used]
        static mut $name: Request = Request {
            id: [$($id),*],
            revision: 0,
            response: ptr::null(),
        };
    };
}

#[unsafe(link_section = ".limine_requests_start_marker")]
#[used]
static LIMINE_REQUESTS_START: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

#[unsafe(link_section = ".limine_requests")]
#[used]
static mut LIMINE_BASE_REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 6];

const COMMON_MAGIC: u64 = 0xc7b1dd30df4c8b88;
const COMMON_MAGIC_2: u64 = 0x0a82e883a194f07b;

request!(
    BOOTLOADER_INFO_REQUEST,
    BootloaderInfoResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0xf55038d8e2a1202f,
        0x279426fcf5f59740
    ]
);
request!(
    HHDM_REQUEST,
    HhdmResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x48dcf1cb8ad2b852,
        0x63984e959a98244b
    ]
);
request!(
    FRAMEBUFFER_REQUEST,
    FramebufferResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x9d5827dcd881dd75,
        0xa3148604f6fab11b
    ]
);
request!(
    MEMMAP_REQUEST,
    MemmapResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x67cf3d9d378a806f,
        0xe304acdfc50c3c62
    ]
);
request!(
    EXECUTABLE_FILE_REQUEST,
    ExecutableFileResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0xad97e90e83f1ed67,
        0x31eb5d1c5ff23b69
    ]
);
request!(
    FIRMWARE_REQUEST,
    FirmwareResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x8c2f75d90bef28a8,
        0x7045a4688eac00c3
    ]
);
request!(
    RSDP_REQUEST,
    RResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0xc5e77b6b397e7b43,
        0x27637845accdcf3c
    ]
);
request!(
    SMBIOS_REQUEST,
    SmbiosResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x9e9046f11e095391,
        0xaa4a520fefbde5ee
    ]
);
request!(
    EFI_SYSTEM_TABLE_REQUEST,
    RResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x5ceba5163eaaf6d6,
        0x0a6981610cf65fcc
    ]
);
request!(
    DATE_REQUEST,
    DateResponse,
    [
        COMMON_MAGIC,
        COMMON_MAGIC_2,
        0x502746e184c088aa,
        0xfbc5ec83e6327893
    ]
);

#[unsafe(link_section = ".limine_requests_end_marker")]
#[used]
static LIMINE_REQUESTS_END: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];

pub fn base_revision_supported() -> bool {
    unsafe { LIMINE_BASE_REVISION[2] == 0 }
}

pub fn hhdm_offset() -> Option<usize> {
    unsafe {
        let response = HHDM_REQUEST.response as *const HhdmResponse;
        (!response.is_null()).then_some((*response).offset as usize)
    }
}

pub fn framebuffer() -> Option<&'static Framebuffer> {
    unsafe {
        let response = FRAMEBUFFER_REQUEST.response as *const FramebufferResponse;
        if response.is_null() || (*response).framebuffer_count == 0 {
            return None;
        }
        let framebuffers = (*response).framebuffers;
        if framebuffers.is_null() || (*framebuffers).is_null() {
            return None;
        }
        Some(&*(*framebuffers))
    }
}
