use std::collections::HashMap;
use std::boxed::Box;

use rustty::{Size, HasSize,Cell,CellAccessor};
use rustty::ui::{VerticalLayout,Label};
use rustty::ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Button,
    ButtonResult,
    Layout,
    Painter,

};

enum MenuEvent{
    Quit
}
impl MenuEvent {
    pub fn try_from(i: i32) -> Result<(),()>{
        match i{
            1 => {MenuEvent::Quit; Ok(())},
            _ => Err(()),
                
        }
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
    fn forward_keys(&mut self,key_map: &mut HashMap<char,ButtonResult>){
        if self.has_focus{
            self.layout.forward_keys(key_map);
        }
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

