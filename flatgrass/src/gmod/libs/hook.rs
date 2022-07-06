use crate::lua::table::Table;

pub struct HookLib<'l>(pub(crate) Table<'l>);