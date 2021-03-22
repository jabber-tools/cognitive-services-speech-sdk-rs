use crate::common::PropertyId;
use crate::error::{convert_err, Result};
use crate::ffi::{property_bag_get_string, property_bag_set_string};
use crate::ffi::{SmartHandle, SPXPROPERTYBAGHANDLE};
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct PropertyCollection {
    pub handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}

impl PropertyCollection {
    pub fn set_property_by_string<S>(&mut self, prop_nam: S, prop_val: S) -> Result<()>
    where
        S: Into<Vec<u8>>,
    {
        let c_name = CString::new(prop_nam)?;
        let c_val = CString::new(prop_val)?;
        unsafe {
            let ret =
                property_bag_set_string(self.handle.get(), -1, c_name.as_ptr(), c_val.as_ptr());
            convert_err(ret, "PropertyCollection.set_property_by_string error")?;
        }

        Ok(())
    }

    pub fn get_property<S>(&self, prop_id: PropertyId, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            // TODO: see NULL_C_STR_PTR in orig solution
            let ret = property_bag_get_string(
                self.handle.get(),
                prop_id.to_i32(),
                std::ptr::null(),
                CString::new(default_val)?.into_raw(),
            );
            Ok(CStr::from_ptr(ret).to_str()?.to_owned())
        }
    }

    pub fn get_property_by_string(&self, _prop_name: String, _default_val: String) -> String {
        unimplemented!();
    }
}
