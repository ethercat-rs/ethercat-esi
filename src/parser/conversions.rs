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
            .map(S::Device::from)
            .collect();

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

impl From<Device> for S::Device {
    fn from(d: Device) -> Self {
        S::Device {
            physics: d.Physics,
            name: d.Name,
            desc: d.Type.Description,
            product_code: d.Type.ProductCode,
            revision_no: d.Type.RevisionNo,
            sm: d.Sm.into_iter().map(S::Sm::from).collect(),
            rx_pdo: d
                .RxPdo
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(S::Pdo::from)
                .collect(),
            tx_pdo: d
                .TxPdo
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(S::Pdo::from)
                .collect(),
        }
    }
}

impl From<Sm> for S::Sm {
    fn from(sm: Sm) -> Self {
        S::Sm {
            start_address: S::HexDecValue(sm.StartAddress),
            control_byte: S::HexDecValue(sm.ControlByte),
            default_size: sm.DefaultSize,
            enable: sm.Enable == Some(1),
        }
    }
}

impl From<Pdo> for S::Pdo {
    fn from(pdo: Pdo) -> Self {
        S::Pdo {
            fixed: pdo.Fixed == 1,
            mandatory: pdo.Mandatory == 1,
            name: pdo
                .Name
                .and_then(|n| if n.is_empty() { None } else { Some(n) }),
            sm: pdo.Sm,
            index: S::HexDecValue(pdo.Index),
            entries: pdo.Entry.into_iter().map(S::Entry::from).collect(),
        }
    }
}

impl From<Entry> for S::Entry {
    fn from(e: Entry) -> Self {
        S::Entry {
            index: S::HexDecValue(e.Index),
            sub_index: e.SubIndex,
            bit_len: e.BitLen,
            name: e.Name,
            data_type: e.DataType,
        }
    }
}

impl TryFrom<Module> for S::Module {
    type Error = Error;
    fn try_from(_: Module) -> Result<Self> {
        Ok(S::Module {})
    }
}
