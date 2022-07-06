use crate::lua::Table;

pub struct HookLib<'l>(pub(crate) Table<'l>);