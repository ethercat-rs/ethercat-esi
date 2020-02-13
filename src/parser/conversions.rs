
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
            decriptions: x.Descriptions.try_into()?,
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

impl TryFrom<Descriptions> for S::Descriptions {
    type Error = Error;
    fn try_from(d: Descriptions) -> Result<Self> {
        let groups: Vec<_> = d
            .Groups
            .items
            .map(|items| items.into_iter().map(S::Group::try_from).collect())
            .unwrap_or_else(|| Ok(vec![]))?;
        let devices: Vec<_> = d
            .Devices
            .items
            .map(|items| items.into_iter().map(S::Device::try_from).collect())
            .unwrap_or_else(|| Ok(vec![]))?;

        let modules: Vec<_> = d
            .Modules
            .map(|dev| {
                dev.items
                    .map(|items| items.into_iter().map(S::Module::try_from).collect())
                    .unwrap_or_else(|| Ok(vec![]))
            })
            .unwrap_or_else(|| Ok(vec![]))?;

        Ok(S::Descriptions {
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
    fn try_from(_: Device) -> Result<Self> {
        Ok(S::Device {})
    }
}

impl TryFrom<Module> for S::Module {
    type Error = Error;
    fn try_from(_: Module) -> Result<Self> {
        Ok(S::Module {})
    }
}
