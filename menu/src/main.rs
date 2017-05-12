extern crate rustty;
mod menu;
mod button;
mod layout;
mod vlayout;
mod dialog;
use std::borrow::BorrowMut;
use menu::Menu;
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
    VerticalLayout,
    HorizontalLayout
};
use button::{Button,StdButton,ButtonResult};
use dialog::Dialog;
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
    cont.frame_mut().set_style(Color::Default,Color::Default,Attr::Reverse);
    let mut menu1 = Menu::from_vec(vec![Box::new(quitbutton),Box::new(cont)],0);


    menu1.pack(&dialog,HorizontalAlign::Middle,VerticalAlign::Middle,(0,0));
    dialog.add_layout(menu1);

    
    'main:  loop{
        while let Some(Event::Key(ch)) = term.get_event(10).unwrap(){
            match dialog.result_for_key(ch){
                Some(ButtonResult::Quit) => {break 'main;}
                _ => {}
            }
            term.clear();
            dialog.resize(term.size());
            dialog.draw(&mut term);

            term.swap_buffers().unwrap();
        }
            
            
    }
}
