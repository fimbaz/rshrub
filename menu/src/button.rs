use rustty::ui::core::{Widget,VerticalAlign,HorizontalAlign};
use rustty::{Size,HasSize,CellAccessor,Cell,Attr,Color};
use rustty::ui::core::{Painter,Frame,Alignable};
//TODO: rename ButtonResult to UIEvent.

#[derive(PartialEq, Clone,Copy)]
pub enum ButtonResult<T: PartialEq + Copy> {
    Quit,
    Left,
    Right,
    Up,
    Down,
    KeyPress(char),
    Event(T)
}

pub trait Focusable: Widget{
    fn unset_focus(&mut self,) { }
    fn get_focus(& self) -> bool { false }
    fn set_focus(&mut self,) { }    
}
pub trait Button<T: PartialEq + Copy>: Focusable + BasicButton<T> {}
pub trait BasicButton<T: PartialEq + Copy>: Widget + Focusable {
    fn result(&self,c: ButtonResult<T>) -> ButtonResult<T>;
    fn accel(&self) -> ButtonResult<T>;
    
}
pub struct StdButton<T: PartialEq + Copy>{
    frame: Frame,
    result: ButtonResult<T>,
    focus: bool,
    accel: ButtonResult<T>,
    text: String
}
impl <T: PartialEq + Copy> StdButton<T>{
    pub fn new(text: &str, accel: ButtonResult<T>, result: ButtonResult<T>) -> StdButton<T>{
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

impl <T: PartialEq + Copy> Focusable for StdButton<T>{
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
impl <T: PartialEq + Copy> BasicButton<T> for StdButton<T>{
    fn result(&self,event: ButtonResult<T>) -> ButtonResult<T>{
        if event == self.accel{
            return  self.result;
        }
        event
    }
    fn accel(&self) -> ButtonResult<T>{
        return self.accel;
    }
    

}
impl <T: PartialEq + Copy> Button<T> for StdButton<T>{}

impl <T: PartialEq + Copy> Widget for StdButton<T> {
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

