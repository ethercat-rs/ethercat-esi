use super::*;
use crate::structs as S;
use std::{
    convert::{TryFrom, TryInto},
    io::{Error, ErrorKind, Result},
    num::ParseIntError,
    str::FromStr,
};

impl TryFrom<EtherCATInfo> for S::EtherCatInfo {
    type Error = Error;
    fn try_from(x: EtherCATInfo) -> Result<Self> {
        Ok(S::EtherCatInfo {
            version: x.Version,
            info_reference: x.InfoReference,
            vendor: x.Vendor.try_into()?,
            description: x.Descriptions.try_into()?,
        })
    }
}

impl TryFrom<Vendor> for S::Vendor {
    type Error = Error;
    fn try_from(v: Vendor) -> Result<Self> {
        let image = v.image()?;
        Ok(S::Vendor {
            file_version: v.FileVersion,
            id: u32_from_hex_dec_value(&v.Id)?,
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

impl Group {
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

impl TryFrom<Descriptions> for S::Description {
    type Error = Error;
    fn try_from(d: Descriptions) -> Result<Self> {
        let groups: Vec<_> = d
            .Groups
            .map(|groups| {
                groups
                    .items
                    .map(|items| items.into_iter().map(S::Group::try_from).collect())
                    .unwrap_or_else(|| Ok(vec![]))
            })
            .unwrap_or_else(|| Ok(vec![]))?;

        let devices: Vec<_> = d
            .Devices
            .items
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(S::Device::try_from)
            .collect::<Result<_>>()?;

        let modules: Vec<_> = d
            .Modules
            .map(|dev| {
                dev.items
                    .map(|items| items.into_iter().map(S::Module::try_from).collect())
                    .unwrap_or_else(|| Ok(vec![]))
            })
            .unwrap_or_else(|| Ok(vec![]))?;

        Ok(S::Description {
            groups,
            devices,
            modules,
        })
    }
}

impl TryFrom<Group> for S::Group {
    type Error = Error;
    fn try_from(g: Group) -> Result<Self> {
        let image = g.image()?;
        Ok(S::Group {
            sort_order: g.SortOrder,
            name: g.Name,
            comment: g.Comment,
            parent_group: g.ParentGroup,
            r#type: g.Type,
            image,
        })
    }
}

impl TryFrom<Device> for S::Device {
    type Error = Error;
    fn try_from(d: Device) -> Result<Self> {
        Ok(S::Device {
            physics: d.Physics,
            name: d.Name,
            desc: d.Type.Description,
            product_code: u32_from_hex_dec_value(&d.Type.ProductCode)?,
            revision_no: u32_from_hex_dec_value(&d.Type.RevisionNo)?,
            sm: d
                .Sm
                .into_iter()
                .map(S::Sm::try_from)
                .collect::<Result<_>>()?,
            rx_pdo: d
                .RxPdo
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(S::Pdo::try_from)
                .collect::<Result<_>>()?,
            tx_pdo: d
                .TxPdo
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(S::Pdo::try_from)
                .collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<Sm> for S::Sm {
    type Error = Error;
    fn try_from(sm: Sm) -> Result<Self> {
        Ok(S::Sm {
            start_address: u16_from_hex_dec_value(&sm.StartAddress)?,
            control_byte: u8_from_hex_dec_value(&sm.ControlByte)?,
            default_size: sm.DefaultSize,
            enable: sm.Enable == Some(1),
        })
    }
}

impl TryFrom<Pdo> for S::Pdo {
    type Error = Error;
    fn try_from(pdo: Pdo) -> Result<Self> {
        Ok(S::Pdo {
            fixed: pdo.Fixed == 1,
            mandatory: pdo.Mandatory == 1,
            name: pdo
                .Name
                .and_then(|n| if n.is_empty() { None } else { Some(n) }),
            sm: pdo.Sm,
            index: u16_from_hex_dec_value(&pdo.Index)?,
            entries: pdo
                .Entry
                .into_iter()
                .map(S::Entry::try_from)
                .collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<Entry> for S::Entry {
    type Error = Error;
    fn try_from(e: Entry) -> Result<Self> {
        Ok(S::Entry {
            index: u16_from_hex_dec_value(&e.Index)?,
            sub_index: e.SubIndex,
            bit_len: e.BitLen,
            name: e.Name,
            data_type: e.DataType,
        })
    }
}

impl TryFrom<Module> for S::Module {
    type Error = Error;
    fn try_from(_: Module) -> Result<Self> {
        Ok(S::Module {})
    }
}

fn u32_from_hex_dec_value(v: &str) -> Result<u32> {
    from_hex_dec_value(v, |x| u32::from_str_radix(x, 16))
}

fn u16_from_hex_dec_value(v: &str) -> Result<u16> {
    from_hex_dec_value(v, |x| u16::from_str_radix(x, 16))
}

fn u8_from_hex_dec_value(v: &str) -> Result<u8> {
    from_hex_dec_value(v, |x| u8::from_str_radix(x, 16))
}

fn from_hex_dec_value<T, F>(v: &str, parse_hex: F) -> Result<T>
where
    T: FromStr<Err = ParseIntError>,
    F: Fn(&str) -> std::result::Result<T, ParseIntError>,
{
    let mut chars = v.chars();
    match (chars.next(), chars.next(), chars.next()) {
        (Some('x'), _, _) | (Some('X'), _, _) => parse_hex(&v[1..]),
        (Some('#'), Some('x'), _)
        | (Some('#'), Some('X'), _)
        | (Some('0'), Some('x'), _)
        | (Some('0'), Some('X'), _) => parse_hex(&v[2..]),
        _ => FromStr::from_str(v),
    }
    .map_err(|e| Error::new(ErrorKind::Other, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u32_from_hex_dec_values() {
        assert_eq!(u32_from_hex_dec_value("0").unwrap(), 0);
        assert_eq!(u32_from_hex_dec_value("1").unwrap(), 1);
        assert_eq!(u32_from_hex_dec_value("0x1").unwrap(), 0x1);
        assert_eq!(u32_from_hex_dec_value("0X1").unwrap(), 0x1);
        assert_eq!(u32_from_hex_dec_value("#x1").unwrap(), 0x1);
        assert_eq!(u32_from_hex_dec_value("#X1").unwrap(), 0x1);
        assert_eq!(u32_from_hex_dec_value("#x005").unwrap(), 0x5);
        assert_eq!(u32_from_hex_dec_value("xF75").unwrap(), 0xf75);
        assert_eq!(u32_from_hex_dec_value("XF75").unwrap(), 0xf75);
    }

    #[test]
    fn parse_u16() {
        assert_eq!(u16_from_hex_dec_value("0").unwrap(), 0);
        assert_eq!(u16_from_hex_dec_value("1").unwrap(), 1);
        assert_eq!(u16_from_hex_dec_value("0x1").unwrap(), 0x1);
        assert_eq!(u16_from_hex_dec_value("0X1").unwrap(), 0x1);
        assert_eq!(u16_from_hex_dec_value("#x1").unwrap(), 0x1);
        assert_eq!(u16_from_hex_dec_value("#X1").unwrap(), 0x1);
        assert_eq!(u16_from_hex_dec_value("#x005").unwrap(), 0x5);
        assert_eq!(u16_from_hex_dec_value("xF75").unwrap(), 0xf75);
        assert_eq!(u16_from_hex_dec_value("XF75").unwrap(), 0xf75);
    }

    #[test]
    fn parse_u8() {
        assert_eq!(u8_from_hex_dec_value("0").unwrap(), 0);
        assert_eq!(u8_from_hex_dec_value("1").unwrap(), 1);
        assert_eq!(u8_from_hex_dec_value("0x1").unwrap(), 0x1);
        assert_eq!(u8_from_hex_dec_value("0X1").unwrap(), 0x1);
        assert_eq!(u8_from_hex_dec_value("#x1").unwrap(), 0x1);
        assert_eq!(u8_from_hex_dec_value("#X1").unwrap(), 0x1);
        assert_eq!(u8_from_hex_dec_value("#x005").unwrap(), 0x5);
        assert_eq!(u8_from_hex_dec_value("xF7").unwrap(), 0xf7);
        assert_eq!(u8_from_hex_dec_value("XF7").unwrap(), 0xf7);
    }
}
