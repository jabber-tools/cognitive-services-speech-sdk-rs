use crate::error::{convert_err, Result};
use crate::ffi::property_bag_set_string;
use crate::ffi::{SmartHandle, SPXPROPERTYBAGHANDLE};
use std::ffi::CString;

// TODO: rename to PropertyBag
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
}
