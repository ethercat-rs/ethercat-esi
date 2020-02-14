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
    pub id: HexDecValue, // TODO: use u32
    pub name: Option<String>,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub desc_url: Option<String>,
    pub image: Option<Image>,
}

/// Further slave descriptions.
#[derive(Debug, Clone)]
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
    pub name: String,
    pub comment: Option<String>,
    pub image: Option<Image>,
    // TODO: Optional 'VendorSpecific'
}

#[derive(Debug, Clone)]
pub struct Device {
    pub physics: Option<String>,
    pub name: String,
    pub desc: String,
    pub product_code: String, // TODO: use u32
    pub revision_no: String,  // TODO: use u32
    pub sm: Vec<Sm>,
    pub rx_pdo: Vec<Pdo>,
    pub tx_pdo: Vec<Pdo>,
}

/// Sync Manager (SM).
#[derive(Debug, Clone)]
pub struct Sm {
    pub enable: bool,
    pub start_address: HexDecValue, // TODO: use u16
    pub control_byte: HexDecValue,  // TODO: use u8
    pub default_size: Option<usize>,
}

/// Process Data Object (PDO).
#[derive(Debug, Clone)]
pub struct Pdo {
    pub sm: usize,
    pub fixed: bool,
    pub mandatory: bool,
    pub index: HexDecValue, // TODO: use u16
    pub name: Option<String>,
    pub entries: Vec<Entry>,
}

/// Service Data Object (SDO).
#[derive(Debug, Clone)]
pub struct Sdo {
    // TODO
}

/// Entry of an Object Dictionary.
#[derive(Debug, Clone)]
pub struct Entry {
    pub index: HexDecValue, // TODO: use u16
    pub sub_index: Option<usize>,
    pub bit_len: usize,
    pub name: Option<String>,
    pub data_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Module {
    // TODO
}

// Restrictions:
// "[+-]?[0-9]{1,}"
// "#x[0-9|a-f|A-F]{1,}"
/// Hex-encoded value.
#[derive(Debug, Clone, PartialEq)]
pub struct HexDecValue(pub String);

/// HexBinary represents arbitrary hex-encoded binary data.
///
/// More info: https://www.w3.org/TR/xmlschema-2/#hexBinary
#[derive(Debug, Clone, PartialEq)]
pub struct HexBinary(pub String);
