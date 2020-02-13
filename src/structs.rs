/// EtherCAT Slave Information (ESI).
#[derive(Debug, Clone)]
pub struct EtherCatInfo {
    pub version: Option<String>,
    pub info_reference: Option<String>,
    pub vendor: Vendor,
    pub decriptions: Descriptions,
}

/// Vendor information.
#[derive(Debug, Clone)]
pub struct Vendor {
    pub file_version: u32,
    pub id: HexDecValue,
    pub name: String,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub desc_url: Option<String>,
    pub image: Option<Image>,
}

/// Further Slave descriptions.
#[derive(Debug, Clone)]
pub struct Descriptions {
    pub groups: Vec<Group>,
    pub devices: Vec<Device>,
    pub modules: Vec<Module>,
}

/// Data image.
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
    // TODO
}

#[derive(Debug, Clone)]
pub struct Module {
    // TODO
}

// Restrictions:
// "[+-]?[0-9]{1,}"
// "#x[0-9|a-f|A-F]{1,}"
/// Hex-encoded value.
#[derive(Debug, Clone)]
pub struct HexDecValue(pub(crate) String);

/// HexBinary represents arbitrary hex-encoded binary data.
///
/// More info: https://www.w3.org/TR/xmlschema-2/#hexBinary
#[derive(Debug, Clone)]
pub struct HexBinary(pub(crate) String);
