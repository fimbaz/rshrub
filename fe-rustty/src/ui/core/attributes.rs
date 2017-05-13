#[derive(Clone,Copy,Debug)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone,Copy,Debug)]
pub enum HorizontalAlign {
    Left,
    Middle,
    Right,
}

#[derive(PartialEq, Clone, Copy,Debug)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}
