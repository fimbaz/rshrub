use std::collections::HashMap;
use std::boxed::Box;

use std::cell::RefCell;
use rustty::{Size, HasSize};
use rustty::{Cell,CellAccessor};

use rustty::ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Painter
};
use layout::Layout;
use vlayout::VerticalLayout;
use button::{ButtonResult,Button,StdButton};
use rustty::ui::Label;

pub struct Dialog {
    frame: Frame,
    pub layouts: Vec<Box<Layout>>,
}


impl Dialog {
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            frame: Frame::new(cols, rows),
            layouts: Vec::new(),
        }
    }
    pub fn add_layout<T: Layout + 'static>(&mut self, layout: T) {
        if self.get_focused().is_none(){
            for layout in &self.layouts {
                for button in layout.get_buttons(){
                    
                }
            }
        }
        self.layouts.push(Box::new(layout));
        self.layouts.last_mut().unwrap().align_elems();
        self.layouts.last_mut().unwrap().frame().draw_into(&mut self.frame);
    }

    pub fn add_label(&mut self, mut label: Label) {
        label.draw(&mut self.frame);
    }

    pub fn get_focused(&self) -> Option<&RefCell<Box<Button>>>{
        for layout in &self.layouts{
            for button in layout.get_buttons(){
                if button.borrow().get_focus(){
                    return Some(button)
                }
            }
        }
        None
    }

    pub fn result_for_key(&self, key: char) -> Option<ButtonResult> {
        let mut maybe_result: Option<ButtonResult> = None;
        for layout in &self.layouts{
            for button in layout.get_buttons(){
                match button.borrow().result(ButtonResult::KeyPress(key)){
                    Some(result) => {return Some(result); }
                    None => {continue; }
                }
            }
        }
        None
    }
}

impl Widget for Dialog {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.draw_into(parent);
    }
    
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn draw_box(&mut self) {
        self.frame.draw_box();
    }

fn resize(&mut self, new_size: Size) {
    self.frame.resize(new_size);
        self.frame.clear(Cell::with_char(' '));
        for layout in &mut self.layouts{
            layout.frame_mut().realign(&self.frame);
            layout.frame_mut().draw_into(&mut self.frame);
        }
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

impl HasSize for Dialog {
    fn size(&self) -> Size {
        self.frame.size()
    }
}
