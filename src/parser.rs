use serde::Deserialize;
use std::{
    convert::TryInto,
    io::{Error, ErrorKind, Result},
};

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
pub struct Descriptions {/* TODO */}

pub(crate) fn from_xml_str(xml: &str) -> Result<super::EtherCatInfo> {
    let raw_info: EtherCATInfo = serde_xml_rs::from_reader(xml.as_bytes())
        .map_err(|e| Error::new(ErrorKind::Other, e.description()))?;
    raw_info.try_into()
}

mod conversions {
    use super::*;
    use crate::structs as S;
    use std::{
        convert::{TryFrom, TryInto},
        io::{Error, ErrorKind, Result},
    };

    impl TryFrom<EtherCATInfo> for S::EtherCatInfo {
        type Error = Error;
        fn try_from(x: EtherCATInfo) -> Result<Self> {
            Ok(S::EtherCatInfo {
                version: x.Version,
                info_reference: x.InfoReference,
                vendor: x.Vendor.try_into()?,
                decriptions: S::Descriptions {},
            })
        }
    }

    impl TryFrom<Vendor> for S::Vendor {
        type Error = Error;
        fn try_from(v: Vendor) -> Result<Self> {
            let image = v.image()?;
            Ok(S::Vendor {
                file_version: v.FileVersion,
                id: S::HexDecValue(v.Id),
                name: v.Name,
                comment: v.Comment,
                url: v.URL,
                desc_url: v.DescriptionURL,
                image,
            })
        }
    }

    impl Vendor {
        fn image(&self) -> Result<Option<S::Image>> {
            match (&self.Image16x14, &self.ImageFile16x14, &self.ImageData16x14) {
                (None, None, None) => Ok(None),
                (Some(img), None, None) => Ok(Some(S::Image::Image16x14(img.clone()))),
                (None, Some(img), None) => Ok(Some(S::Image::ImageFile16x14(img.clone()))),
                (None, None, Some(img)) => {
                    Ok(Some(S::Image::ImageData16x14(S::HexBinary(img.clone()))))
                }
                _ => Err(Error::new(ErrorKind::Other, "Multiple images found")),
            }
        }
    }
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
            <Descriptions />
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
                Descriptions: Descriptions {}
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
}
