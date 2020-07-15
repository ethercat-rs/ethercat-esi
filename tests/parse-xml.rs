use ethercat_esi::EtherCatInfo;
use ethercat_types as ec;
use std::{fs::File, io::prelude::*};

#[test]
fn parse_xml_crated_by_weidmueller() {
    let mut file = File::open("tests/fixtures/Weidmueller_UR20_FBC.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x0000_0230);
    assert_eq!(esi.description.devices.len(), 2);
    let dev_0 = &esi.description.devices[0];
    let dev_1 = &esi.description.devices[1];
    assert_eq!(dev_0.product_code, 0x4F911C30);
    assert_eq!(dev_1.product_code, 0x4F911C30);
    assert_eq!(dev_0.revision_no, 0x1);
    assert_eq!(dev_1.revision_no, 0x00011100);
    assert_eq!(dev_0.sm[0].start_address, 0x1000);
    assert_eq!(dev_0.sm[0].control_byte, 0x26);
    assert_eq!(dev_0.rx_pdo[0].index, ec::Idx::from(0x16FF));
    assert_eq!(dev_0.rx_pdo[0].entries[0].index, ec::Idx::from(0xF200));
}

#[test]
fn parse_xml_crated_by_weidmueller_module_information() {
    let mut file = File::open("tests/fixtures/Weidmueller_UR20_IO.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x0000_0230);
    assert_eq!(esi.description.modules.len(), 82);
    let m = &esi.description.modules[0];
    assert_eq!(m.tx_pdo.as_ref().unwrap().entries.len(), 6);
}

#[test]
fn parse_xml_crated_by_beckhoff() {
    let mut file = File::open("tests/fixtures/Beckhoff_EK11xx.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 2);
    assert_eq!(esi.description.devices.len(), 24);
}

#[test]
fn parse_xml_crated_by_igh() {
    // Parse file that was crated by `/opt/etherlab/bin/ethercat xml`
    let mut file = File::open("tests/fixtures/Weidmueller_UR20_FBC_from_IgH.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x230);
}
