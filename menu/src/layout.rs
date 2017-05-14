use rustty::ui::core::{Widget};
use button::{Button,ButtonResult,Focusable};
use std::cell::RefCell;
use rustty::HasSize;
use std::collections::HashMap;

/// Specialized version of a widget that implements an alignment function
/// and method for forwarding keys to the parent widgets key map. 
pub trait Layout<T: PartialEq + Copy>: Widget + Focusable {
    fn align_elems(&mut self);
    fn result_for_key(&self, result:ButtonResult<T>) -> ButtonResult<T>;
    fn get_buttons(&self) -> Vec<&Box<Button<T>>>;
    fn get_buttons_mut(&mut self) -> Vec<&mut Box<Button<T>>>;
    fn down(&mut self);
    fn up(&mut self);
    
}


