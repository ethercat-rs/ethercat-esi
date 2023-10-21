pub use ethercat_types::{PdoEntryIdx, PdoIdx, SmIdx, SubIdx};

/// EtherCAT Slave Information (ESI).
#[derive(Debug, Clone)]
pub struct EtherCatInfo {
    pub version: Option<String>,
    pub info_reference: Option<String>,
    pub vendor: Vendor,
    pub description: Description,
}

/// Vendor information.
#[derive(Debug, Clone)]
pub struct Vendor {
    pub file_version: Option<u32>,
    pub id: u32,
    pub name: Names,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub desc_url: Option<String>,
    pub image: Option<Image>,
}

/// A collection of human readable names, with attached language IDs.
pub type Names = Vec<(String, Option<u16>)>;

/// Further slave descriptions.
#[derive(Debug, Clone, Default)]
pub struct Description {
    pub groups: Vec<Group>,
    pub devices: Vec<Device>,
    pub modules: Vec<Module>,
}

/// Image data (BMP file format).
#[derive(Debug, Clone)]
pub enum Image {
    /// Obsolete
    Image16x14(String),
    ImageFile16x14(String),
    ImageData16x14(HexBinary),
}

#[derive(Debug, Clone)]
pub struct Group {
    pub sort_order: Option<i32>,
    pub parent_group: Option<String>,
    pub r#type: String,
    pub name: Names,
    pub comment: Option<String>,
    pub image: Option<Image>,
    // TODO: Optional 'VendorSpecific'
}

#[derive(Debug, Clone)]
pub struct Device {
    pub physics: Option<String>,
    pub name: Names,
    pub desc: String,
    pub product_code: Option<u32>,
    pub revision_no: Option<u32>,
    pub sm: Vec<Sm>,
    pub rx_pdo: Vec<Pdo>,
    pub tx_pdo: Vec<Pdo>,
}

/// Sync Manager (SM).
#[derive(Debug, Clone)]
pub struct Sm {
    pub enable: bool,
    pub start_address: u16,
    pub control_byte: Option<u8>,
    pub default_size: Option<usize>,
    pub r#virtual: bool,
}

/// Process Data Object (PDO).
#[derive(Debug, Clone)]
pub struct Pdo {
    pub sm: Option<SmIdx>,
    pub fixed: bool,
    pub mandatory: bool,
    pub idx: PdoIdx,
    pub name: Names,
    pub entries: Vec<PdoEntry>,
}

/// Service Data Object (SDO).
#[derive(Debug, Clone)]
pub struct Sdo {
    // TODO
}

/// PDO Entry.
#[derive(Debug, Clone)]
pub struct PdoEntry {
    pub entry_idx: PdoEntryIdx,
    pub bit_len: usize,
    pub name: Names,
    pub data_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub r#type: String,
    pub name: Names,
    pub tx_pdo: Vec<Pdo>,
    pub rx_pdo: Vec<Pdo>,
    pub mailbox: Option<Mailbox>,
    pub profile: Option<Profile>,
}

#[derive(Debug, Clone)]
pub struct Mailbox {
    // TODO
}

#[derive(Debug, Clone)]
pub struct Profile {
    // TODO
}

/// HexBinary represents arbitrary hex-encoded binary data.
///
/// More info: https://www.w3.org/TR/xmlschema-2/#hexBinary
#[derive(Debug, Clone, PartialEq)]
pub struct HexBinary(pub String);
