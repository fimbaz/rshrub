use rustty::ui::Dialog;
use rustty::{Terminal,Event};
use rustty::ui::core::{Layout,Widget};
use std::borrow::BorrowMut;


pub trait UIComponent{
    fn get_dialog_mut(&mut self) -> &mut Dialog;
    fn get_dialog(&self) -> & Dialog;
}

pub struct UI{
    terminal: Terminal,
    components: Vec<Box<UIComponent>>
}


impl UI{
    pub fn new() -> UI {
        return UI { terminal: Terminal::new().unwrap(),components: Vec::new()};
    }
    pub fn pump(&mut self){
        loop {
            while let Some(Event::Key(ch)) = self.terminal.get_event(10).unwrap() {
                match ch {
                    'q' => {return;},
                    'd' => {self.draw_boxes()},
                    _ => continue
                }
                for  component in &mut self.components{
                    component.get_dialog_mut().draw(&mut self.terminal);
                }
            }
        }
    }
    pub fn draw_boxes(&mut self){
        for  component in &mut self.components{
            component.get_dialog_mut().draw_box();
        }
    }
    pub fn add_component(&mut self,component: Box<UIComponent>){
        self.components.push(component);
    }
 
}
