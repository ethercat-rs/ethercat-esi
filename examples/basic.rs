use ethercat_esi::EtherCatInfo;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
};

fn main() -> io::Result<()> {
    match env::args().nth(1) {
        None => {
            eprintln!("Missing filename");
        }
        Some(file_name) => {
            let mut xml_file = File::open(file_name)?;
            let mut xml_string = String::new();
            xml_file.read_to_string(&mut xml_string)?;
            let info = EtherCatInfo::from_xml_str(&xml_string)?;
            println!("{:#?}", info);
        }
    }
    Ok(())
}
