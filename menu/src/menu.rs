use std::collections::HashMap;
use std::boxed::Box;

use rustty::{Size, HasSize,Cell,CellAccessor};
use rustty::ui::{Label};
use rustty::ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Painter,

};
use button::{ButtonResult,Button};
use layout::{Layout};
use vlayout::{VerticalLayout};
enum MenuEvent{
    Quit,
    Up,
    Left,
    Down,
    Right
}
impl MenuEvent {
    pub fn try_from(i: i32) -> Result<MenuEvent,()>{
        let event = match i{
            1 => {MenuEvent::Quit},
            2 => {MenuEvent::Up},
            3 => {MenuEvent::Left},
            4 => {MenuEvent::Down},
            5 => {MenuEvent::Right},
            _ => {return Err(());},
                
        };
        Ok(event)
    }
}

pub struct Menu{
    layout: VerticalLayout,
    active_item: usize,
    has_focus: bool,
}

impl Menu{
    pub fn from_vec(widgets: Vec<Box<Button>>,inner_margin: usize) -> Menu{
        return Menu{active_item:0,layout: VerticalLayout::from_vec(widgets,0),has_focus:true};
    }

}

impl Layout for Menu{
    fn align_elems(&mut self){
        self.layout.align_elems();
    }
}

impl Widget for Menu {
    fn draw(&mut self, parent: &mut CellAccessor) { 
        self.layout.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.layout.frame.align(parent, halign, valign, margin);
    }

    fn draw_box(&mut self) {
        self.layout.frame.draw_box();
    }

    fn resize(&mut self, new_size: Size) {
        self.layout.frame.resize(new_size);
    }

    fn frame(&self) -> &Frame {
        &self.layout.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.layout.frame
    }
}

