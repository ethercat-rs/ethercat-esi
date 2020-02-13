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
    FileVersion: u32,
    Id: String,
    Name: String,
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
    Groups: Groups,
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
    // TODO
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
                    FileVersion: 99,
                    Id: "#x00000000".to_string(),
                    Name: "Vendor Foo".to_string(),
                    Comment: None,
                    URL: None,
                    DescriptionURL: None,
                    Image16x14: None,
                    ImageFile16x14: None,
                    ImageData16x14: Some("7D".to_string()),
                },
                Descriptions: Descriptions {
                    Groups: Groups { items: None },
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
                FileVersion: 45,
                Id: "#x00000999".to_string(),
                Name: "Vendor Name".to_string(),
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
                Groups: Groups {
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
                },
                Devices: Devices { items: None },
                Modules: None,
            }
        );
    }
}
