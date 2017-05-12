use rustty::ui::core::{Widget,VerticalAlign,HorizontalAlign};
use rustty::{Size,HasSize,CellAccessor,Cell,Attr,Color};
use rustty::ui::core::{Painter,Frame,Alignable};
//TODO: rename ButtonResult to UIEvent.

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonResult {
    Quit,
    Left,
    Right,
    Up,
    Down,
    KeyPress(char),
}

pub trait Button: Widget {
    //only return None if we've consumed the event.
    fn result(&self,c: ButtonResult) -> Option<ButtonResult>;
    fn set_focus(&mut self,) { }
    fn unset_focus(&mut self,) { }
    fn get_focus(& self) -> bool { false }
    fn accel(&self) -> char;
    
}
pub struct StdButton{
    frame: Frame,
    result: ButtonResult,
    focus: bool,
    accel: char,
    text: String
}
impl StdButton{
    pub fn new(text: &str, accel: char, result: ButtonResult) -> StdButton{
        let s = format!("[{}]",text);
        let width=s.chars().count();
        let mut button = StdButton { frame: Frame::new(width,1),
                                     accel: accel,
                                     result: result,
                                     focus: false,
                                     text: s};
        button.frame.printline(0,0,&button.text[..]);
        button
    }

}
impl Button for StdButton{
    fn result(&self,event: ButtonResult) -> Option<ButtonResult>{
        if let  ButtonResult::KeyPress(c) = event{
            if c == self.accel{
                return Some(self.result);
            }
        }
        return None
    }
    fn set_focus(&mut self){
        self.frame.set_style(Color::Default,Color::Default,Attr::Reverse);
        self.focus = true;
    }
    fn unset_focus(&mut self){
        self.frame.set_style(Color::Default,Color::Default,Attr::Default);
        self.focus = false;
    }

    fn get_focus(&self)-> bool{
        return self.focus;
    }
    fn accel(&self) -> char{
        return self.accel;
    }
    

}

impl Widget for StdButton {
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

