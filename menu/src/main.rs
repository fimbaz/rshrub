extern crate rustty;
mod button;
mod layout;
mod vlayout;
mod dialog;
use std::borrow::BorrowMut;
use rustty::{Terminal,Event,HasSize,Cell,Color,Attr};
use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    Frame,
    Layout
};

use rustty::ui::{
    Label,
};
use button::{Button,StdButton,ButtonResult};
use dialog::Dialog;
use vlayout::VerticalLayout;
fn main(){
    let mut term = Terminal::new().unwrap();
    let size = term.size();
    let mut dialog = Dialog::new(size.0,size.1);

    let mut label1 = Label::new(0,2);
//    label1.set_text("hello world...");
//    label1.align_text(HorizontalAlign::Middle,VerticalAlign::Top,(1,1));
    //    label1.pack(&dialog,HorizontalAlign::Middle,VerticalAlign::Top,(1,1));
    //    dialog.add_label(label1);
    let mut quitbutton = StdButton::new("Quit",'q',ButtonResult::Quit);
    let mut cont = StdButton::new("Continue",'j',ButtonResult::Quit);
    let mut special = StdButton::new("Go Back In Time and Kill Hitler's Grandmother",'j',ButtonResult::Quit);
    let mut layout = VerticalLayout::from_vec(vec![Box::new(quitbutton),Box::new(cont),Box::new(special)],0);
//    layout.down();
    dialog.add_layout(layout);
    
    'main:  loop{
        while let Some(Event::Key(ch)) = term.get_event(10).unwrap(){
            match ch{
                'w' => dialog.layouts.get_mut(0).unwrap().up(),
                's' => dialog.layouts.get_mut(0).unwrap().down(),
                _ => {}
                    
            }
            term.clear();
            dialog.resize(term.size());
            dialog.draw(&mut term);

            term.swap_buffers().unwrap();
        }
            
            
    }
}
