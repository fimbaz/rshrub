use rustty::{Event};
use rustty::ui::core::{Layout};
use rustty::ui::Dialog;
use ui::uibutton::UIButton;
use ui::basebutton::UIEvent;
use ui::core::UIComponent;


//components are responsible 
impl UIComponent for MainMenu{
    fn get_dialog_mut(&mut self) -> &mut Dialog{
        return &mut self.dialog;
    }
    fn get_dialog(& self) -> & Dialog{
        return & self.dialog;
    }

}
impl MainMenu{
    pub fn init(&mut self){
        
    }
}
pub struct MainMenu{
    dialog: Dialog,
}
