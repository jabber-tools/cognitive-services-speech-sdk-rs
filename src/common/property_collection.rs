use crate::common::PropertyId;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_get_string, property_bag_release, property_bag_set_string, SmartHandle,
    SPXPROPERTYBAGHANDLE,
};
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct PropertyCollection {
    pub handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}

impl PropertyCollection {
    pub fn from_handle(handle: SPXPROPERTYBAGHANDLE) -> PropertyCollection {
        PropertyCollection {
            handle: SmartHandle::create("PropertyCollection", handle, property_bag_release),
        }
    }

    pub fn set_property_by_string<S>(&mut self, prop_name: S, prop_val: S) -> Result<()>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let c_name = CString::new(prop_name)?;
            let c_val = CString::new(prop_val)?;
            let ret =
                property_bag_set_string(self.handle.inner(), -1, c_name.as_ptr(), c_val.as_ptr());
            convert_err(ret, "PropertyCollection.set_property_by_string error")?;
        }

        Ok(())
    }

    pub fn set_property<S>(&mut self, prop_id: PropertyId, prop_val: S) -> Result<()>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let c_val = CString::new(prop_val)?;
            let ret = property_bag_set_string(
                self.handle.inner(),
                prop_id.to_i32(),
                std::ptr::null(),
                c_val.as_ptr(),
            );
            convert_err(ret, "PropertyCollection.set_property error")?;
        }

        Ok(())
    }

    pub fn get_property<S>(&self, prop_id: PropertyId, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            // TODO: see NULL_C_STR_PTR in orig solution
            let c_default_val = CString::new(default_val)?.as_ptr();
            let ret = property_bag_get_string(
                self.handle.inner(),
                prop_id.to_i32(),
                std::ptr::null(),
                c_default_val,
            );
            Ok(CStr::from_ptr(ret).to_str()?.to_owned())
        }
    }

    pub fn get_property_by_string<S>(&self, prop_name: S, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let c_name = CString::new(prop_name)?.as_ptr();
            let c_default_val = CString::new(default_val)?.as_ptr();
            let ret = property_bag_get_string(self.handle.inner(), -1, c_name, c_default_val);
            Ok(CStr::from_ptr(ret).to_str()?.to_owned())
        }
    }
}
