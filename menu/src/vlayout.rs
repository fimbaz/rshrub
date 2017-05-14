use rustty::{Size, Pos, HasSize, HasPosition};
use rustty::{Cell,CellAccessor,Color,Attr};
use std::boxed::Box;
use std::collections::HashMap;
use rustty::ui::core::{
    Alignable, 
    HorizontalAlign, 
    VerticalAlign, 
    Widget, 
    Frame, 
    Painter,
};
use layout::Layout;
use button::{StdButton,Button,ButtonResult,Focusable};
pub struct VerticalLayout<T: PartialEq + Copy> {
    pub frame: Frame,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button<T>>>,
    focus: bool
}

impl <T: PartialEq + Copy> VerticalLayout<T> {
    pub fn from_vec(widgets: Vec<Box<Button<T>>>, mut inner_margin: usize) -> VerticalLayout<T> {
        let first_origin = widgets.first().unwrap().frame().origin();
        let height = widgets.len() + widgets.len() * inner_margin;
        let width = widgets.iter().map(|s| s.frame().size().0).max().unwrap();
        VerticalLayout {
            frame: Frame::new(width, height),
            inner_margin: inner_margin,
            origin: first_origin,
            widgets: widgets,
            focus: false,
        }
    }
    fn get_inner_focus(&self) -> Option<&Box<Button<T>>>{
        for button in &self.widgets{
            if button.get_focus(){
                return Some(button);
            }

        }
        return None;
    }
    fn set_inner_focus(&mut self,index: usize){
        for (i,widget) in &mut self.widgets.iter_mut().enumerate(){
            if index == i{
                widget.set_focus();
            }else{
                widget.unset_focus();
            }
        }
    }
    pub fn redraw(&mut self){
        let size = self.frame.size();
        self.resize(size);
    }
}

impl <T: PartialEq + Copy> Widget for VerticalLayout<T> {
    fn draw(&mut self, parent: &mut CellAccessor) {
        if self.get_focus() {
            let mut new_frame = self.frame.clone();
            new_frame.draw_box();
            new_frame.draw_into(parent)
        }
        else{
            self.frame.draw_into(parent);
        }
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn draw_box(&mut self) {
        self.frame.draw_box();
    }

    fn resize(&mut self, new_size: Size) {
        self.frame.clear(Cell::with_char(' '));
        self.frame.resize(new_size);
        for widget in &mut self.widgets{
            widget.frame_mut().draw_into(&mut self.frame);
        }
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

impl <T: PartialEq + Copy> Layout<T> for VerticalLayout<T> {
    fn align_elems(&mut self) {
        let (x, y) = self.origin;
        let mut current_y = y;
        for widget in self.widgets.iter_mut() {
            widget.frame_mut().set_origin((x, current_y));
            current_y += 1 + self.inner_margin;
        }
        for w in self.widgets.iter() {
            w.frame().draw_into(&mut self.frame);
        }
    }
    fn get_buttons(&self) -> Vec<&Box<Button<T>>> {
        self.widgets.iter().collect()
    }
    fn get_buttons_mut(&mut self) -> Vec<&mut Box<Button<T>>> {
        self.widgets.iter_mut().collect()
    }
    fn down(&mut self){
        let i = (self.widgets.iter().position(|w|w.get_focus()).unwrap_or(0) + 1) % self.widgets.len();
        self.set_inner_focus(i);
        self.redraw();
    }
    fn up(&mut self){
        let i = (self.widgets.iter().position(|w|w.get_focus()).unwrap_or(0).checked_sub(1).unwrap_or(self.widgets.len()-1)) % self.widgets.len();
        self.set_inner_focus(i);
        self.redraw();
    }
    fn result_for_key(&self,result:ButtonResult<T>) -> ButtonResult<T>{
        if let Some( button) = self.widgets.iter().find(|b|b.result(result) != result){
            return button.result(result);
        }else{
            return result;
        }
        
        
            
    }

}

impl <T: PartialEq + Copy> Focusable for VerticalLayout<T>{
    fn set_focus(&mut self){
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
