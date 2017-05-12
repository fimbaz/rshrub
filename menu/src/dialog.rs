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

pub struct Dialog {
    frame: Frame,
    pub buttons: Vec<Box<Button>>,
    pub layouts: Vec<Box<Layout>>,
}


impl Dialog {
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            frame: Frame::new(cols, rows),
            buttons: Vec::new(),
            layouts: Vec::new(),
        }
    }
    pub fn add_button<T: Button + 'static>(&mut self, button: T) {
        self.buttons.push(Box::new(button));

        self.buttons.last_mut().unwrap().draw(&mut self.frame);
    }
    pub fn add_layout<T: Layout + 'static>(&mut self, layout: T) {
        self.layouts.push(Box::new(layout));
        
        self.layouts.last_mut().unwrap().align_elems();
        self.layouts.last_mut().unwrap().frame().draw_into(&mut self.frame);
    }

    pub fn add_label(&mut self, mut label: Label) {
        label.draw(&mut self.frame);
    }

    pub fn focus_button(&mut self, index:usize) {
        if let Some(button) = self.buttons.get_mut(index){
            button.set_focus()
        }
        
    }


    pub fn result_for_key(&self, key: char) -> Option<ButtonResult> {
        let mut maybe_result: Option<ButtonResult> = None;
        for button in &self.buttons{
            match button.result(ButtonResult::KeyPress(key)){
                Some(result) => {  return Some(result); }
                None => { continue ; }
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
        for button in &mut self.buttons{
            button.frame_mut().realign(&self.frame);
            button.frame_mut().draw_into(&mut self.frame);
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
