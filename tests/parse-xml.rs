use ethercat_esi::{EtherCatInfo, HexDecValue};
use std::{fs::File, io::prelude::*};

#[test]
fn parse_xml_crated_by_weidmueller() {
    let mut file = File::open("tests/fixtures/Weidmueller_UR20_FBC.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, HexDecValue("#x00000230".to_string()));
    assert_eq!(esi.description.devices.len(), 2);
    let dev_0 = &esi.description.devices[0];
    let dev_1 = &esi.description.devices[1];
    assert_eq!(dev_0.product_code, "#x4F911C30");
    assert_eq!(dev_1.product_code, "#x4F911C30");
    assert_eq!(dev_0.revision_no, "#x00000001");
    assert_eq!(dev_1.revision_no, "#x00011100");
    assert_eq!(dev_0.sm[0].start_address, HexDecValue("#x1000".into()));
    assert_eq!(dev_0.sm[0].control_byte, HexDecValue("#x26".into()));
    assert_eq!(dev_0.rx_pdo[0].index, HexDecValue("#x16FF".into()));
    assert_eq!(
        dev_0.rx_pdo[0].entries[0].index,
        HexDecValue("#xF200".into())
    );
}

#[test]
fn parse_xml_crated_by_igh() {
    // Parse file that was crated by `/opt/etherlab/bin/ethercat xml`
    let mut file = File::open("tests/fixtures/Weidmueller_UR20_FBC_from_IgH.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let esi = EtherCatInfo::from_xml_str(&xml_string).unwrap();
    assert_eq!(esi.vendor.id, HexDecValue("560".to_string()));
}
