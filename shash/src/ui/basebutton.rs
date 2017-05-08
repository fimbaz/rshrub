use rustty::ui::core::{Widget};
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum UIEvent{
    Quit()
}

pub fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
    let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
    for (i, c) in s.chars().enumerate() {
        if c.to_lowercase().next().unwrap_or(c) == lower_accel {
            return Some(i)
        }
    }
    None
}

pub trait BaseButton: Widget { 
    fn accel(&self) -> char;
    fn result(&self) -> UIEvent;
    fn pressed(&mut self) { }
    fn state(&self) -> bool { false }
}

