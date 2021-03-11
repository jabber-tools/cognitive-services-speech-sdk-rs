use crate::ffi::{SmartHandle, SPXPROPERTYBAGHANDLE};

#[derive(Debug)]
pub struct PropertyCollection {
    pub handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}
