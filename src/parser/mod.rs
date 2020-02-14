use serde::Deserialize;
use std::{
    convert::TryInto,
    io::{Error, ErrorKind, Result},
};

mod conversions;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct EtherCATInfo {
    Version: Option<String>,
    InfoReference: Option<String>,
    Vendor: Vendor,
    Descriptions: Descriptions,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct Vendor {
    FileVersion: Option<u32>,
    Id: String,
    Name: Option<String>,
    Comment: Option<String>,
    URL: Option<String>,
    DescriptionURL: Option<String>,
    Image16x14: Option<String>,
    ImageFile16x14: Option<String>,
    ImageData16x14: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Descriptions {
    Groups: Option<Groups>,
    Devices: Devices,
    Modules: Option<Modules>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Groups {
    #[serde(rename = "$value")]
    items: Option<Vec<Group>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Devices {
    #[serde(rename = "$value")]
    items: Option<Vec<Device>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Modules {
    #[serde(rename = "$value")]
    items: Option<Vec<Module>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Group {
    SortOrder: Option<i32>,
    ParentGroup: Option<String>,
    Type: String,
    Name: String,
    Comment: Option<String>,
    Image16x14: Option<String>,
    ImageFile16x14: Option<String>,
    ImageData16x14: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Device {
    Physics: Option<String>,
    Type: DeviceType,
    Name: String,
    RxPdo: Option<Vec<RxPdo>>,
    TxPdo: Option<Vec<TxPdo>>,
    Sm: Vec<Sm>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct DeviceType {
    ModulePdoGroup: Option<String>,
    ProductCode: String,
    RevisionNo: String,
    #[serde(rename = "$value")]
    Description: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Sm {
    Enable: Option<u8>,
    StartAddress: String,
    ControlByte: String,
    DefaultSize: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Entry {
    Index: String,
    SubIndex: Option<usize>,
    BitLen: usize,
    Name: Option<String>,
    DataType: Option<String>,
}

pub type RxPdo = Pdo;
pub type TxPdo = Pdo;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Pdo {
    Sm: usize,
    Fixed: u8,
    Mandatory: u8,
    Index: String,
    Name: Option<String>,
    Entry: Vec<Entry>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Module {
    // TODO
}

pub(crate) fn from_xml_str(xml: &str) -> Result<super::EtherCatInfo> {
    let raw_info: EtherCATInfo = serde_xml_rs::from_reader(xml.as_bytes())
        .map_err(|e| Error::new(ErrorKind::Other, e.description()))?;
    raw_info.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn ethercat_info() {
        let s = r##"
        <EtherCATInfo Version="1.11" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="EtherCATInfo.xsd">
            <InfoReference>FooBar.xml</InfoReference>
            <Vendor FileVersion="0099">
                <Id>#x00000000</Id>
                <Name>Vendor Foo</Name>
                <ImageData16x14>7D</ImageData16x14>
            </Vendor>
            <Descriptions>
                <Groups/>
                <Devices/>
            </Descriptions>
        </EtherCATInfo>
        "##;
        let info: EtherCATInfo = from_str(s).unwrap();

        assert_eq!(
            info,
            EtherCATInfo {
                Version: Some("1.11".to_string()),
                InfoReference: Some("FooBar.xml".to_string()),
                Vendor: Vendor {
                    FileVersion: Some(99),
                    Id: "#x00000000".to_string(),
                    Name: Some("Vendor Foo".to_string()),
                    Comment: None,
                    URL: None,
                    DescriptionURL: None,
                    Image16x14: None,
                    ImageFile16x14: None,
                    ImageData16x14: Some("7D".to_string()),
                },
                Descriptions: Descriptions {
                    Groups: Some(Groups { items: None }),
                    Devices: Devices { items: None },
                    Modules: None,
                }
            }
        );
    }

    #[test]
    fn vendor() {
        let s = r##"
 		<Vendor FileVersion="0045">
 			<Id>#x00000999</Id>
 			<Name>Vendor Name</Name>
 			<ImageData16x14>7D7D7D7</ImageData16x14>
 		</Vendor>"##;
        let vendor: Vendor = from_str(s).unwrap();

        assert_eq!(
            vendor,
            Vendor {
                FileVersion: Some(45),
                Id: "#x00000999".to_string(),
                Name: Some("Vendor Name".to_string()),
                Comment: None,
                URL: None,
                DescriptionURL: None,
                Image16x14: None,
                ImageFile16x14: None,
                ImageData16x14: Some("7D7D7D7".to_string()),
            }
        )
    }

    #[test]
    fn descriptions() {
        let s = r##"
			<Descriptions>
				<Groups>
					<Group SortOrder="0">
						<Type>Coupler</Type>
						<Name>Coupler</Name>
						<ImageData16x14>44</ImageData16x14>
					</Group>
				</Groups>
				<Devices></Devices>
			</Descriptions>"##;
        let descriptions: Descriptions = from_str(s).unwrap();
        assert_eq!(
            descriptions,
            Descriptions {
                Groups: Some(Groups {
                    items: Some(vec![Group {
                        SortOrder: Some(0),
                        ParentGroup: None,
                        Type: "Coupler".to_string(),
                        Name: "Coupler".to_string(),
                        Comment: None,
                        Image16x14: None,
                        ImageFile16x14: None,
                        ImageData16x14: Some("44".to_string()),
                    }]),
                }),
                Devices: Devices { items: None },
                Modules: None,
            }
        );
    }

    #[test]
    fn entry() {
        let s = r##"
          <Entry>
            <Index>#xf200</Index>
            <SubIndex>2</SubIndex>
            <BitLen>1</BitLen>
            <Name></Name>
            <DataType>BOOL</DataType>
          </Entry>"##;
        let entry: Entry = from_str(s).unwrap();
        assert_eq!(
            entry,
            Entry {
                Index: "#xf200".to_string(),
                SubIndex: Some(2),
                BitLen: 1,
                Name: Some("".to_string()),
                DataType: Some("BOOL".to_string()),
            }
        );
    }

    #[test]
    fn rx_pdo() {
        let s = r##"
        <RxPdo Sm="2" Fixed="1" Mandatory="1">
          <Index>#x16ff</Index>
          <Name></Name>
          <Entry>
            <Index>#xf200</Index>
            <SubIndex>3</SubIndex>
            <BitLen>1</BitLen>
            <Name></Name>
            <DataType>BOOL</DataType>
          </Entry>
        </RxPdo>"##;
        let pdo: RxPdo = from_str(s).unwrap();
        assert_eq!(
            pdo,
            RxPdo {
                Sm: 2,
                Fixed: 1,
                Mandatory: 1,
                Index: "#x16ff".to_string(),
                Name: Some("".to_string()),
                Entry: vec![Entry {
                    Index: "#xf200".to_string(),
                    SubIndex: Some(3),
                    BitLen: 1,
                    Name: Some("".to_string()),
                    DataType: Some("BOOL".to_string()),
                }]
            }
        );
    }

    #[test]
    fn device() {
        let s = r##"
        <Device>
          <Type ProductCode="#x45" RevisionNo="#x001">Foo</Type>
          <Name>Bar</Name>
          <Sm Enable="1" StartAddress="#x1000" ControlByte="#x26" DefaultSize="512" />
          <Sm Enable="1" StartAddress="#x1400" ControlByte="#x22" DefaultSize="512" />
          <Sm            StartAddress="#x1800" ControlByte="#x64"                 />
          <Sm Enable="0" StartAddress="#x2400" ControlByte="#x20" DefaultSize="0" />
        </Device>"##;
        let device: Device = from_str(s).unwrap();
        assert_eq!(
            device,
            Device {
                Physics: None,
                RxPdo: None,
                TxPdo: None,
                Sm: vec![
                    Sm {
                        Enable: Some(1),
                        StartAddress: "#x1000".to_string(),
                        ControlByte: "#x26".to_string(),
                        DefaultSize: Some(512),
                    },
                    Sm {
                        Enable: Some(1),
                        StartAddress: "#x1400".to_string(),
                        ControlByte: "#x22".to_string(),
                        DefaultSize: Some(512),
                    },
                    Sm {
                        Enable: None,
                        StartAddress: "#x1800".to_string(),
                        ControlByte: "#x64".to_string(),
                        DefaultSize: None,
                    },
                    Sm {
                        Enable: Some(0),
                        StartAddress: "#x2400".to_string(),
                        ControlByte: "#x20".to_string(),
                        DefaultSize: Some(0),
                    }
                ],
                Name: "Bar".to_string(),
                Type: DeviceType {
                    Description: "Foo".to_string(),
                    ModulePdoGroup: None,
                    ProductCode: "#x45".to_string(),
                    RevisionNo: "#x001".to_string(),
                }
            }
        );
    }
}
