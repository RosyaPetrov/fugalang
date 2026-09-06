#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    Private,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub visibility: Visibility,
    pub mutable: bool,
    pub comptime: bool,
    pub is_unsafe: bool,
    pub is_macro: bool,
}
