use crate::RustStr;

extern "C" {
    fn lua_value_string(string: RustStr) -> u32;
}

#[repr(transparent)]
pub struct LuaValue(u32);

crate::impl_datatype_drop!(LuaValue);

impl LuaValue {
    fn to_ptr(&self) -> u32 {
        self.0
    }
}

impl From<&str> for LuaValue {
    fn from(value: &str) -> LuaValue {
        unsafe { LuaValue(lua_value_string(value.into())) }
    }
}

macro_rules! impl_scalar_lua_value {
	($($name:ident($type:ident))*) => {
		$(
            extern "C" {
                fn $name(value: $type) -> u32;
            }

			impl From<$type> for LuaValue {
				fn from(value: $type) -> LuaValue {
					unsafe { Self($name(value)) }
				}
			}
		)*
	}
}

impl_scalar_lua_value!(
    lua_value_f32(f32)
    lua_value_f64(f64)

    lua_value_i8(i8)
    lua_value_i16(i16)
    lua_value_i32(i32)
    lua_value_i64(i64)

    lua_value_u8(u8)
    lua_value_u16(u16)
    lua_value_u32(u32)
    lua_value_u64(u64)

    lua_value_usize(usize)
    lua_value_isize(isize)
);
