use std::fs;

use ethercat_esi::EtherCatInfo;
use ethercat_types as ec;

#[test]
fn parse_xml_crated_by_weidmueller() {
    let xml_string = fs::read_to_string("tests/fixtures/Weidmueller_UR20_FBC.xml").unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x0000_0230);
    assert_eq!(esi.description.devices.len(), 2);
    let dev_0 = &esi.description.devices[0];
    let dev_1 = &esi.description.devices[1];
    assert_eq!(dev_0.product_code, Some(0x4F911C30));
    assert_eq!(dev_1.product_code, Some(0x4F911C30));
    assert_eq!(dev_0.revision_no, Some(0x1));
    assert_eq!(dev_1.revision_no, Some(0x00011100));
    assert_eq!(dev_0.sm[0].start_address, 0x1000);
    assert_eq!(dev_0.sm[0].control_byte, Some(0x26));
    assert_eq!(dev_0.rx_pdo[0].idx, ec::PdoIdx::from(0x16FF));
    assert_eq!(
        dev_0.rx_pdo[0].entries[0].entry_idx.idx,
        ec::Idx::from(0xF200)
    );
    assert_eq!(
        dev_0.rx_pdo[0].entries[0].entry_idx.sub_idx,
        ec::SubIdx::from(1)
    );
}

#[test]
fn parse_xml_crated_by_weidmueller_module_information() {
    let xml_string = fs::read_to_string("tests/fixtures/Weidmueller_UR20_IO.xml").unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x0000_0230);
    assert_eq!(esi.description.modules.len(), 82);
    let m = &esi.description.modules[0];
    assert_eq!(m.tx_pdo[0].entries.len(), 6);
}

#[test]
fn parse_xml_crated_by_beckhoff() {
    let xml_string = fs::read_to_string("tests/fixtures/Beckhoff_EK11xx.xml").unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 2);
    assert_eq!(esi.description.devices.len(), 24);
}

#[test]
fn parse_xml_crated_by_igh() {
    // Parse file that was crated by `/opt/etherlab/bin/ethercat xml`
    let xml_string =
        fs::read_to_string("tests/fixtures/Weidmueller_UR20_FBC_from_IgH.xml").unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, 0x230);
}

#[test]
#[ignore]
fn parse_xml_crated_by_nanotec_electronic() {
    let xml_string = fs::read_to_string("tests/fixtures/C5-E-2-21.xml").unwrap();
    EtherCatInfo::from_xml_str(&xml_string).unwrap();
}
