use rustty::{Terminal,Event};
use rustty::ui::core::Widget;
use std::borrow::BorrowMut;
pub struct UI{
    terminal: Terminal,
    widgets: Vec<Box<Widget>>
}
impl UI{
    pub fn new() -> UI {
        return UI { terminal: Terminal::new().unwrap(),widgets: Vec::new()};
    }
    pub fn pump(&mut self){
        loop {
            while let Some(Event::Key(ch)) = self.terminal.get_event(10).unwrap() {
                match ch {
                    'q' => {return;},
                    'd' => {self.draw_boxes()},
                    _ => continue
                }
                for  widget in &mut self.widgets{
                    widget.draw(&mut self.terminal);
                }
            }
        }
    }
    pub fn draw_boxes(&mut self){
        for  widget in &mut self.widgets{
            let none: () = widget.draw_box();
        }
    }
    pub fn add_widget(&mut self,widget: Box<Widget>){
        self.widgets.push(widget);
    }
 
}
