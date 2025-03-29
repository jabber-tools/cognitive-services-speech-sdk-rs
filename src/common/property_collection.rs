use crate::common::PropertyId;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_free_string, property_bag_get_string, property_bag_release,
    property_bag_set_string, SmartHandle, NULL_C_STR_PTR, SPXPROPERTYBAGHANDLE,
};
use std::ffi::{CStr, CString};

/// PropertyCollection is a class to retrieve or set a property value from a property collection.
#[derive(Debug)]
pub struct PropertyCollection {
    pub handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}

impl PropertyCollection {
    /// Creates a PropertyCollection from a handle (for internal use)
    ///
    /// # Safety
    /// `handle` must be a valid handle to a live property collection.
    pub unsafe fn from_handle(handle: SPXPROPERTYBAGHANDLE) -> PropertyCollection {
        PropertyCollection {
            handle: SmartHandle::create("PropertyCollection", handle, property_bag_release),
        }
    }

    /// SetProperty sets the value of a property.
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

    /// SetProperty sets the value of a property.
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

    /// Returns value of a property.
    /// If the property value is not defined, the specified default value is returned.
    pub fn get_property<S>(&self, prop_id: PropertyId, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let c_default_val = CString::new(default_val)?;
            let c_default_val_ptr = c_default_val.as_ptr();
            let ret = property_bag_get_string(
                self.handle.inner(),
                prop_id.to_i32(),
                std::ptr::null(),
                c_default_val_ptr,
            );
            if ret == NULL_C_STR_PTR {
                Ok("".to_owned())
            } else {
                // Ok(CStr::from_ptr(ret).to_str()?.to_owned())
                // based on tests result will not always contain
                // happnening in SpeechRecognitionCanceledEvent::from_handle
                // when getting property SpeechServiceResponseJsonErrorDetails
                // valid UTF-8 data -> we use to_string_lossy()
                let value = CStr::from_ptr(ret).to_string_lossy().into_owned();

                convert_err(
                    property_bag_free_string(ret),
                    "PropertyCollection::get_property(property_bag_free_string) error",
                )?;

                Ok(value)
            }
        }
    }

    /// Returns value of a property.
    /// If the property value is not defined, the specified default value is returned.
    pub fn get_property_by_string<S>(&self, prop_name: S, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let c_name = CString::new(prop_name)?;
            let c_name_ptr = c_name.as_ptr();
            let c_default_val = CString::new(default_val)?;
            let c_default_val_ptr = c_default_val.as_ptr();
            let ret =
                property_bag_get_string(self.handle.inner(), -1, c_name_ptr, c_default_val_ptr);
            if ret == NULL_C_STR_PTR {
                Ok("".to_owned())
            } else {
                // Ok(CStr::from_ptr(ret).to_str()?.to_owned())
                let value = CStr::from_ptr(ret).to_string_lossy().into_owned();

                convert_err(
                    property_bag_free_string(ret),
                    "PropertyCollection::get_property_by_string(property_bag_free_string) error",
                )?;

                Ok(value)
            }
        }
    }
}
