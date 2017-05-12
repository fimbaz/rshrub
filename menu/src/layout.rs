use rustty::ui::core::{Widget};
use button::ButtonResult;
use rustty::HasSize;
use std::collections::HashMap;

/// Specialized version of a widget that implements an alignment function
/// and method for forwarding keys to the parent widgets key map. 
pub trait Layout: Widget {
    fn align_elems(&mut self);
}


