use rustty::{Size, HasSize};
use rustty::{Attr, CellAccessor};
use ui::basebutton::{BaseButton,UIEvent};
use rustty::ui::core::{
    Alignable, 
    HorizontalAlign, 
    VerticalAlign,
    Widget,
    Painter,
    Frame,
    Button,
    ButtonResult,
    find_accel_char_index
};
pub struct UIButton {
    frame: Frame,
    accel: char,
    result: UIEvent,
    text: String
}


impl BaseButton for UIButton {
    fn accel(&self) -> char {
        self.accel
    }

    fn result(&self) -> UIEvent {
        self.result
    }
}

impl Widget for UIButton {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize,usize)) {
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

