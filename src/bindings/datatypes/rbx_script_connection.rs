extern "C" {
    fn disconnect_connection(connection: u32);
    fn is_connected(connection: u32) -> bool;
}

#[repr(C)]
pub struct RbxScriptConnection(pub(crate) u32);

impl RbxScriptConnection {
    fn to_ptr(&self) -> u32 {
        self.0
    }

    pub fn connected(&self) -> bool {
        unsafe { is_connected(self.to_ptr()) }
    }

    pub fn disconnect(&self) {
        unsafe { disconnect_connection(self.to_ptr()) }
    }

    pub fn leak(self) {
        std::mem::forget(self);
    }
}

crate::impl_datatype_drop!(RbxScriptConnection, |self| { self.disconnect() });
