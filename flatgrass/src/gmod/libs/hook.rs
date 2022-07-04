use crate::lua::value::LuaTable;

pub struct HookLib<'l>(pub(crate) LuaTable<'l>);