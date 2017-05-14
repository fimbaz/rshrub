use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell,Attr,Color};
use ui::core::alignable::{Alignable};
use ui::core::{HorizontalAlign,VerticalAlign};
/// The `Frame` struct is the building block for all future
/// widgets inside of *ui*. Objects of `Frame` abstract away
/// the actual creation and drawing of areas of a terminal,
/// because this process is the same for all widgets. Every
/// widget should contain one `Frame` type to be used to render
/// text to the screen
#[derive(Clone,Debug)]
pub struct Frame  {
    origin: Pos,
    size: Size,
    halign: HorizontalAlign,
    valign: VerticalAlign,
    margin: (usize,usize),
    buf: Vec<Cell>,
}

impl Frame {
    /// Constructs a new Frame object with a width of `cols`
    /// and height of `rows`
    pub fn new(cols: usize, rows: usize) -> Frame {
        Frame {
            origin: (0, 0),
            size: (cols, rows),
            buf: vec![Cell::default(); cols * rows],
            halign: HorizontalAlign::Left,
            valign: VerticalAlign::Top,
            margin: (0,0)
        }
    }
    pub fn new_aligned(cols: usize, rows:usize,halign:HorizontalAlign,valign:VerticalAlign,margin:(usize,usize)) -> Frame{
        Frame {
            origin: (0, 0),
            size: (cols, rows),
            buf: vec![Cell::default(); cols * rows],
            halign: halign,
            valign: valign,
            margin: margin
        }
    }
    /// Draw the buffer contained inside of the base object to 
    /// a valid object that implements CellAccessor.
    pub fn draw_into(&self, cells: &mut CellAccessor) {
        let (cols, rows) = self.size();
        let (x, y) = self.origin();
        for ix in 0..cols {
            let offset_x = x + ix;
            for iy in 0..rows {
                let offset_y = y + iy;
                match cells.get_mut(offset_x, offset_y) {
                    Some(cell) => { *cell = *self.get(ix, iy).unwrap(); },
                    None => (),
                }
            }
        }
    }
    pub fn set_style(&mut self,fg: Color, bg: Color,attr: Attr) {
        let (cols, rows) = self.size();
        for ix in 0..cols {
            for iy in 0..rows {
                let mut cell = self.get_mut(ix,iy).unwrap();
                *cell = Cell::new(cell.ch(),fg,bg,attr);
            }
        }
    }


    pub fn resize(&mut self, new_size: Size) {
        let difference = (new_size.0 * self.size.0) - (new_size.1 * self.size.1);
        self.buf.extend(vec![Cell::default(); difference]);
        self.size = new_size;
    }
}
impl HasSize for Frame {
    fn size(&self) -> Size {
        self.size
    }
}

impl CellAccessor for Frame {
    fn cellvec(&self) -> &Vec<Cell> {
        &self.buf
    }

    fn cellvec_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.buf
    }

}

impl HasPosition for Frame {
    fn origin(&self) -> Pos {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Pos) {
        self.origin = new_origin;
    }
}


impl Alignable for Frame {
    fn get_valign(&self) -> VerticalAlign{
        return self.valign;
    }
    fn get_halign(&self) -> HorizontalAlign{
        return self.halign;
    }
    fn  get_margin(&self) ->(usize,usize){
        return self.margin;
    }
    fn set_valign(&mut self,valign:VerticalAlign){
        self.valign = valign;
    }
    
    fn set_halign(&mut self,halign:HorizontalAlign){
        self.halign = halign;
    }
    fn set_margin(&mut self,margin:(usize,usize)){
        self.margin = margin;
    }
    
}
