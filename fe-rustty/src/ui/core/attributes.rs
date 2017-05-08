#[derive(Clone,Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone,Copy)]
pub enum HorizontalAlign {
    Left,
    Middle,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}
