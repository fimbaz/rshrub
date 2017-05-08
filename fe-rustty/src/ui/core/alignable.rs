use core::position::{HasSize, HasPosition};
use ui::core::attributes::{HorizontalAlign, VerticalAlign};
use ui::core::Widget;

pub trait Alignable: HasSize + HasPosition {
    fn get_valign(& self) -> VerticalAlign{
        return VerticalAlign::Top;
    }
    fn get_halign(& self) -> HorizontalAlign{
        return HorizontalAlign::Left;
    }
    fn get_margin(&self) -> (usize,usize){
        return (0,0);
    }
    fn set_valign(&mut self,valign: VerticalAlign);
    fn set_halign(&mut self,halign: HorizontalAlign);
    fn set_margin(&mut self,margin: (usize,usize));
    fn realign(&mut self,parent: &HasSize){
        let halign = self.get_halign();
        let valign = self.get_valign();
        let margin = self.get_margin();
        self.align(parent,halign,valign,margin);

    }
    fn halign(&mut self, parent: &HasSize, halign: HorizontalAlign, margin: usize) {
        let (cols, _) = self.size();
        let (_, y) = self.origin();
        let (parent_cols, _) = parent.size();
        let newx = match halign {
            HorizontalAlign::Left => margin,
            HorizontalAlign::Right => parent_cols - cols - margin,
            HorizontalAlign::Middle => (parent_cols - cols) / 2,
        };
        self.set_origin((newx, y));
    }

    fn valign(&mut self, parent: &HasSize, valign: VerticalAlign, margin: usize) {
        let (_, rows) = self.size();
        let (x, _) = self.origin();
        let (_, parent_rows) = parent.size();
        let newy = match valign {
            VerticalAlign::Top => margin,
            VerticalAlign::Bottom => parent_rows - rows - margin,
            VerticalAlign::Middle => (parent_rows - rows) / 2,
        };
        self.set_origin((x, newy));
    }
    
    fn align(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign, 
             margin: (usize, usize)) {
        self.set_halign(halign);
        self.set_valign(valign);
        self.set_margin(margin);
        self.halign(parent, halign, margin.0);
        self.valign(parent, valign, margin.1);
    }
}

