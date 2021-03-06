use rect::{Pos,HasPos,Region,BucketPos};
use grid::GridCell;
use tile::Tile;
use boring_game::game::BoringGame;
use rustty::Cell;
use rustty::{Size,HasSize,Attr};
use rustty::ui::core::{Widget,Frame,HorizontalAlign,VerticalAlign,Painter,Alignable};
use rustty::{CellAccessor};
pub struct WorldView {
    pub origin: Pos,
    pub cursor: Pos,
    pub frame: Frame
}

impl WorldView {
    pub fn new(x: usize, y: usize) -> WorldView{
        return WorldView { origin: Pos::new(x,y),cursor: Pos::new(0,0),frame: Frame::new(80,40) };
    }
    pub fn update_world(&mut self,game: &BoringGame){
        self.adjust_origin();
        self.draw_background(game.ground_level);
        self.update_active(&game);
        self.draw_cursor();
    }
    fn update_active(&mut self,game: &BoringGame){
        let size = self.frame.size();
        let roi = Region{x:self.origin.x,y:self.origin.y,width:size.0,height:size.1};
        let boxes: Vec<BucketPos> = roi.iter().collect();
            
        for tile in  game.grid.range_query(&roi){
            let pos = Pos {x: tile.pos.x - self.origin.x, y: tile.pos.y - self.origin.y};
            let index = self.frame.pos_to_index(pos.x as usize, pos.y as usize).clone();
            match index{
                Some(i) => {self.frame.cellvec_mut()[i] = tile.repr();}
                None => {}
            }
        }
    }
    fn adjust_origin(&mut self){
        let frame = &mut self.frame;
        let origin = self.origin;
        let cursor = self.cursor;
        let frame_size = (frame.size().0.saturating_sub(1) as usize,frame.size().1.saturating_sub(1) as usize);
        if !(Region {x:self.origin.x,y:self.origin.y,width:frame_size.0,height:frame_size.1}).contains(&self.cursor){
            if self.cursor.x >= self.origin.x + frame_size.0 as usize{
                self.origin.x = self.origin.x + self.cursor.x.saturating_sub(self.origin.x + frame_size.0);
            }else if self.cursor.x < self.origin.x {
                self.origin.x = self.cursor.x
            }
            if self.cursor.y > self.origin.y + frame_size.1{
                self.origin.y = self.origin.y + self.cursor.y.saturating_sub(self.origin.y + frame_size.1); 
            }else if self.cursor.y < self.origin.y{
                self.origin.y = self.cursor.y
            }
        }
    }
    pub fn draw_cursor(&mut self){
        let cursor = self.cursor;
        let origin = self.origin;
        let cursor_index = if let Some(i) = self.frame.pos_to_index(cursor.x.saturating_sub(origin.x) as usize,cursor.y.saturating_sub(origin.y)as usize) {i} else { 0 } ;
        self.frame.cellvec_mut().get_mut(cursor_index).unwrap().set_attrs(Attr::Reverse);

    }
    pub fn draw_background(&mut self,ground_level: usize){
        let frame_ground_level = ground_level.checked_sub(self.origin.y).unwrap_or(0) as u16;
        let mut frame_coords: (u16,u16)=(0,0);
        let frame_size = self.frame.size();
        for cell in self.frame.cellvec_mut().iter_mut(){
            if frame_coords.0 == frame_size.0 as u16{
                frame_coords.0  = 0;
                frame_coords.1 = frame_coords.1 + 1;
            }
            if frame_coords.1 == frame_size.1 as u16{
                break;
            }
            frame_coords.0=frame_coords.0+1;
            if frame_coords.1 >= frame_ground_level{
                *cell=  Tile::stp_ground_repr()
            }else{
                *cell= Tile::stp_air_repr();
            }
        }
    }


}

impl Widget for WorldView {
    fn draw(&mut self,parent: &mut CellAccessor){
        self.frame.draw_into(parent);
    }
    fn pack(&mut self,parent: &HasSize,halign: HorizontalAlign,valign: VerticalAlign,margin: (usize,usize)){
        self.frame.align(parent,halign,valign,margin);
    }
    fn draw_box(&mut self) {
        self.frame.draw_box();
    }
    fn resize(&mut self, new_size: Size){
        self.frame.resize(new_size);
    }
    fn frame(&self) -> &Frame{
        &self.frame
    }
    fn frame_mut(&mut self) -> &mut Frame{
        &mut self.frame
    }
}
