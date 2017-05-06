use std::cell::RefCell;
use grid::Grid;
use rect::{Pos,HasPos,Region,BucketPos,Iter};
impl HasPos for Tile {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.pos.x,y:self.pos.y};
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Substrate{
    Dirt(),
    Space(),
}
#[derive(Debug,Copy,Clone)]
pub struct Resources{
    pub water: f32,
    pub air: f32,
    pub substrate: Substrate,
}
#[derive(Debug,Copy,Clone)]
pub struct CellData{
    pub resources: Resources
}
#[derive(Debug,Clone)]
pub struct Tile{
    pub pos: Pos,
    pub data: RefCell<CellData>,
}



impl Tile {
    pub fn new(pos:Pos,air:f32,water:f32,substrate:Substrate) -> Tile{
        Tile{pos:pos,data:RefCell::new(CellData{resources:Resources{water,air,substrate}})}
        
    }
    pub fn merge(&self,tile: &Tile){
        let mut my_resources = self.data.borrow_mut().resources;
        let mut other_resources = tile.data.borrow().resources;
        my_resources.water += other_resources.water;
        my_resources.air += other_resources.air;
        my_resources.substrate = other_resources.substrate;

    }
}

