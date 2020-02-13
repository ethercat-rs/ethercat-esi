//! # EtherCAT Slave Information (ESI).
//!
//! The EtherCAT Slave Information (ESI) file is an XML file that is used by
//! some EtherCAT master stacks to configure the slaves and generate network
//! description files.
//! However, it's main purpose is to describe how data is shared with the
//! slave, including what sync managers it uses, and what PDOs are in
//! each sync manager.
//!
//! The official XML schema can be found in the
//! *[EtherCAT Slave Information (ESI) Schema](https://www.ethercat.org/en/downloads/downloads_981F0A9A81044A878CE329DC8818F495.htm)*
//! (see `EtherCATInfo.xsd`).
//!
//! ## Example
//!
//! ```rust
//! use ethercat_esi::EtherCatInfo;
//! use std::{
//!     env,
//!     fs::File,
//!     io::{self, prelude::*},
//! };
//!
//! fn main() -> io::Result<()> {
//!     match env::args().nth(1) {
//!         None => {
//!             eprintln!("Missing filename");
//!         }
//!         Some(file_name) => {
//!             let mut xml_file = File::open(file_name)?;
//!             let mut xml_string = String::new();
//!             xml_file.read_to_string(&mut xml_string)?;
//!             let info = EtherCatInfo::from_xml_str(&xml_string)?;
//!             println!("{:#?}", info);
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use std::io;

mod parser;
mod structs;

pub use structs::*;

impl EtherCatInfo {
    pub fn from_xml_str(xml: &str) -> io::Result<Self> {
        parser::from_xml_str(xml)
    }
}
