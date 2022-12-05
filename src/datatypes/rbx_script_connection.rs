use crate::drop_pointer;

extern "C" {
    fn disconnect_connection(connection: u32);
    fn is_connected(connection: u32) -> bool;
}

#[repr(transparent)]
pub struct RbxScriptConnection(pub(crate) u32);

impl Drop for RbxScriptConnection {
    fn drop(&mut self) {
        unsafe { drop_pointer(self.0) }
    }
}

impl RbxScriptConnection {
    pub fn connected(&self) -> bool {
        unsafe { is_connected(self.0) }
    }

    pub fn disconnect(&self) {
        unsafe { disconnect_connection(self.0) }
    }

    pub fn leak(self) {
        std::mem::forget(self);
    }
}
