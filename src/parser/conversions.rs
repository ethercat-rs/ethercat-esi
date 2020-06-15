use super::*;
use crate::structs as S;
use std::{convert::TryFrom, num::ParseIntError, str::FromStr};

impl TryFrom<EtherCATInfo> for S::EtherCatInfo {
    type Error = Error;
    fn try_from(x: EtherCATInfo) -> Result<Self> {
        let mut description = match x.Descriptions {
            Some(d) => d.try_into()?,
            None => S::Description::default(),
        };

        if let Some(Modules {
            items: Some(modules),
        }) = x.Modules
        {
            modules
                .into_iter()
                .map(S::Module::try_from)
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .for_each(|m| {
                    description.modules.push(m);
                })
        }

        Ok(S::EtherCatInfo {
            version: x.Version,
            info_reference: x.InfoReference,
            vendor: x.Vendor.try_into()?,
            description,
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
        let img = self.items.iter().filter(|p| match p {
            GroupProperty::Image16x14(_)
            | GroupProperty::ImageFile16x14(_)
            | GroupProperty::ImageData16x14(_) => true,
            _ => false,
        });
        if img.clone().count() > 1 {
            return Err(Error::new(ErrorKind::Other, "Multiple images found"));
        }
        for p in img {
            match p {
                GroupProperty::Image16x14(img) => {
                    return Ok(Some(S::Image::Image16x14(img.clone())))
                }
                GroupProperty::ImageFile16x14(img) => {
                    return Ok(Some(S::Image::ImageFile16x14(img.clone())))
                }
                GroupProperty::ImageData16x14(img) => {
                    return Ok(Some(S::Image::ImageData16x14(S::HexBinary(img.clone()))))
                }
                _ => {}
            }
        }
        Ok(None)
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
        let comment = g
            .items
            .iter()
            .filter_map(|p| {
                if let GroupProperty::Comment(c) = p {
                    Some(c)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0);

        let props = g.items.iter();
        let name = props
            .clone()
            .filter_map(|p| {
                if let GroupProperty::Name(Name { value, .. }) = p {
                    Some(value)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Mandatory group name not found"))?;

        let r#type = props
            .filter_map(|p| {
                if let GroupProperty::Type(t) = p {
                    Some(t)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Mandatory group type not found"))?;

        Ok(S::Group {
            sort_order: g.SortOrder,
            parent_group: g.ParentGroup,
            name,
            comment,
            r#type,
            image,
        })
    }
}

impl TryFrom<Device> for S::Device {
    type Error = Error;
    fn try_from(d: Device) -> Result<Self> {
        let props = d.items.iter();
        let name = props
            .clone()
            .filter_map(|p| {
                if let DeviceProperty::Name(Name { value, .. }) = p {
                    Some(value)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Mandatory device name not found"))?;
        let d_type = props
            .clone()
            .filter_map(|p| {
                if let DeviceProperty::Type(t) = p {
                    Some(t)
                } else {
                    None
                }
            })
            .nth(0)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Mandatory device type not found"))?;

        let product_code = u32_from_hex_dec_value(&d_type.ProductCode)?;
        let revision_no = u32_from_hex_dec_value(&d_type.RevisionNo)?;
        let desc = d_type.Description.to_owned();

        let sm = props
            .clone()
            .filter_map(|p| {
                if let DeviceProperty::Sm(sm) = p {
                    Some(sm)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(S::Sm::try_from)
            .collect::<Result<_>>()?;

        let rx_pdo = props
            .clone()
            .filter_map(|p| {
                if let DeviceProperty::RxPdo(pdo) = p {
                    Some(pdo)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(S::Pdo::try_from)
            .collect::<Result<_>>()?;

        let tx_pdo = props
            .clone()
            .filter_map(|p| {
                if let DeviceProperty::TxPdo(pdo) = p {
                    Some(pdo)
                } else {
                    None
                }
            })
            .cloned()
            .nth(0)
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(S::Pdo::try_from)
            .collect::<Result<_>>()?;

        Ok(S::Device {
            physics: d.Physics,
            name,
            desc,
            product_code,
            revision_no,
            sm,
            rx_pdo,
            tx_pdo,
        })
    }
}

impl TryFrom<Sm> for S::Sm {
    type Error = Error;
    fn try_from(sm: Sm) -> Result<Self> {
        Ok(S::Sm {
            start_address: u16_from_hex_dec_value(&sm.StartAddress)?,
            control_byte: u8_from_hex_dec_value(&sm.ControlByte)?,
            default_size: if let Some(x) = sm.DefaultSize {
                let n = u32_from_hex_dec_value(&x)?;
                Some(n as usize)
            } else {
                None
            },
            enable: sm.Enable == Some(1),
        })
    }
}

impl TryFrom<Pdo> for S::Pdo {
    type Error = Error;
    fn try_from(pdo: Pdo) -> Result<Self> {
        Ok(S::Pdo {
            fixed: pdo.Fixed == Some(1),
            mandatory: pdo.Mandatory == Some(1),
            name: pdo
                .Name
                .and_then(|n| if n.is_empty() { None } else { Some(n) }),
            sm: pdo.Sm,
            index: u16_from_hex_dec_value(&pdo.Index.value)?,
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
            index: u16_from_hex_dec_value(&e.Index.value)?,
            sub_index: match e.SubIndex {
                Some(idx_string) => Some(u32_from_hex_dec_value(&idx_string)?),
                None => None,
            },
            bit_len: e.BitLen,
            name: e.Name,
            data_type: e.DataType,
        })
    }
}

impl TryFrom<Module> for S::Module {
    type Error = Error;
    fn try_from(m: Module) -> Result<Self> {
        let rx_pdo = match m.RxPdo {
            Some(pdo) => {
                let pdo = S::Pdo::try_from(pdo)?;
                Some(pdo)
            }
            None => None,
        };

        let tx_pdo = match m.TxPdo {
            Some(pdo) => {
                let pdo = S::Pdo::try_from(pdo)?;
                Some(pdo)
            }
            None => None,
        };

        Ok(S::Module {
            name: m.Name,
            r#type: m.Type,
            rx_pdo,
            tx_pdo,
            mailbox: S::Mailbox {},
            profile: S::Profile {},
        })
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
