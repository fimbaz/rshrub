use std::collections::HashMap;
use std::boxed::Box;

use rustty::{Size, HasSize};
use rustty::CellAccessor;

use rustty::ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Button,
    Painter
};
use rustty::ui::Label;
use ui::basebutton::{UIEvent,BaseButton};

use ui::layout::{Layout};
pub struct Dialog {
    frame: Frame,
    buttons: Vec<Box<BaseButton>>,
    layouts: Vec<Box<Layout>>,
    accel2result: HashMap<char, UIEvent>,
}


impl Dialog {
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            frame: Frame::new(cols, rows),
            buttons: Vec::new(),
            layouts: Vec::new(),
            accel2result: HashMap::new(),
        }
    }

    pub fn add_button<T: BaseButton + 'static>(&mut self, button: T) {
        self.accel2result.insert(button.accel(), button.result());
        self.buttons.push(Box::new(button));

        self.buttons.last_mut().unwrap().draw(&mut self.frame);
    }

    pub fn add_layout<T: Layout + 'static>(&mut self, layout: T) {
        self.layouts.push(Box::new(layout));
        
        self.layouts.last_mut().unwrap().align_elems();
        self.layouts.last_mut().unwrap().frame().draw_into(&mut self.frame);
        self.layouts.last_mut().unwrap().forward_keys(&mut self.accel2result);
    }
    pub fn add_label(&mut self, mut label: Label) {
        label.draw(&mut self.frame);
    }
    pub fn button_pressed(&mut self, res: UIEvent) {
        match self.buttons.iter_mut().find(|x| x.result() == res) {
            Some(i) => { i.pressed(); i.draw(&mut self.frame)}
            _       => { panic!("Not a valid button result for\
                                Dialog::button_checked()"); }
        }
    }
    pub fn is_button_pressed(&self, res: UIEvent) -> bool {
        match self.buttons.iter().find(|x| x.result() == res) {
            Some(i) => i.state(),
            _       => panic!("Not a valid button result for\
                               Dialog::is_button_checked()")
        }
    }

    pub fn result_for_key(&self, key: char) -> Option<UIEvent> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
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
