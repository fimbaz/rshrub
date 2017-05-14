use std::collections::HashMap;
use std::boxed::Box;

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

pub struct Dialog<T> {
    frame: Frame,
    pub layouts: Vec<Box<Layout<T>>>,
}


impl <T: PartialEq + Copy> Dialog<T> {
    pub fn new(cols: usize, rows: usize) -> Dialog<T> {
        Dialog {
            frame: Frame::new(cols, rows),
            layouts: Vec::new(),
        }
    }
    pub fn add_layout<R: Layout<T> + 'static>(&mut self, mut layout: R) {
        if self.get_focused().is_none(){
            if let Some(button) =  layout.get_buttons_mut().get_mut(0){
                button.set_focus();
            }
        }
        self.layouts.push(Box::new(layout));
        self.layouts.last_mut().unwrap().align_elems();
        self.layouts.last_mut().unwrap().frame().draw_into(&mut self.frame);
    }

    pub fn add_label(&mut self, mut label: Label) {
        label.draw(&mut self.frame);
    }

    pub fn get_focused(&self) -> Option<&Box<Layout<T>>>{
        for layout in &self.layouts{
            if layout.get_focus(){
                return Some(layout)
            }
        }
        None
    }

    pub fn result_for_key(&self, result: ButtonResult<T>) -> ButtonResult<T> {
        if let Some(layout) = self.layouts.iter().find(|l|l.result_for_key(result) != result){
            layout.result_for_key(result)
        }else{
            result
        }
    }
}

impl  <T> Widget for Dialog<T> {
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

impl <T> HasSize for Dialog<T>{
    fn size(&self) -> Size {
        self.frame.size()
    }
}
