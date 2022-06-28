use super::get_game;

crate::impl_instance!(DataModel);

impl DataModel {
    pub fn instance() -> DataModel {
        unsafe { DataModel(get_game()) }
    }
}
