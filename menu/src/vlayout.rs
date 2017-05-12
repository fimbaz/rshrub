use rustty::{Size, Pos, HasSize, HasPosition};
use rustty::{Cell,CellAccessor};
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
use button::{StdButton,Button,ButtonResult};
pub struct VerticalLayout {
    pub frame: Frame,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button>>,
}

impl VerticalLayout {
    pub fn from_vec(widgets: Vec<Box<Button>>, inner_margin: usize) -> VerticalLayout {
        let first_origin = widgets.first().unwrap().frame().origin();
        let height = widgets.len() + widgets.len() * inner_margin;
        let width = widgets.iter().map(|s| s.frame().size().0).max().unwrap();
        VerticalLayout {
            frame: Frame::new(width, height),
            inner_margin: inner_margin,
            origin: first_origin,
            widgets: widgets
        }
    }


}

impl Widget for VerticalLayout {
    fn draw(&mut self, parent: &mut CellAccessor) { 
        self.frame.draw_into(parent);
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

impl Layout for VerticalLayout {
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
    fn get_buttons(&self) -> Vec<&Box<Button>> {
        self.widgets.iter().collect()
    }
    fn get_buttons_mut(&mut self) -> Vec<&mut Box<Button>> {
        self.widgets.iter_mut().collect()
    }

}
