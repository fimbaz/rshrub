use rustty::ui::core::{Widget,VerticalAlign,HorizontalAlign};
use rustty::{Size,HasSize,CellAccessor,Cell,Attr,Color};
use rustty::ui::core::{Painter,Frame,Alignable};
//TODO: rename ButtonResult to UIEvent.

#[derive(PartialEq, Clone,Copy)]
pub enum ButtonResult {
    Quit,
    Left,
    Right,
    Up,
    Down,
    KeyPress(char),
    Jeepers
}

pub trait Focusable: Widget{
    fn unset_focus(&mut self,) { }
    fn get_focus(& self) -> bool { false }
    fn set_focus(&mut self,) { }    
}
pub trait Button: Focusable + BasicButton {}
pub trait BasicButton: Widget + Focusable {
    fn result(&self,c: ButtonResult) -> ButtonResult;
    fn accel(&self) -> ButtonResult;
    
}
pub struct StdButton{
    frame: Frame,
    result: ButtonResult,
    focus: bool,
    accel: ButtonResult,
    text: String
}
impl StdButton{
    pub fn new(text: &str, accel: ButtonResult, result: ButtonResult) -> StdButton{
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

impl Focusable for StdButton{
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
}
impl BasicButton for StdButton{
    fn result(&self,event: ButtonResult) -> ButtonResult{
        if event == self.accel{
            return  self.result;
        }
        event
    }
    fn accel(&self) -> ButtonResult{
        return self.accel;
    }
    

}
impl Button for StdButton{}

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

