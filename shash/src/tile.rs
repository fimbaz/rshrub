use std::cell::RefCell;
use grid::Grid;
use rect::{Pos,HasPos,Region,BucketPos,Iter};
impl HasPos for TileHolder {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.pos.x,y:self.pos.y};
    }
}

pub trait MergeCells {
    fn merge(&self,point:Self);
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
pub struct Tile{
    pub resources: Resources
}
#[derive(Debug,Clone)]
pub struct TileHolder{
    pub pos: Pos,
    pub tile: RefCell<Tile>,
}

impl Tile{
    pub fn new_v1(water:f32,air:f32,substrate: Substrate) -> Tile{
        return Tile { resources: Resources {water,air,substrate} };
    }
}

impl TileHolder {
    pub fn new_v1(pos:Pos,air:f32,water:f32,substrate:Substrate) -> TileHolder{
        TileHolder{pos:pos,tile:RefCell::new(Tile::new_v1(water,air,substrate))}
    }
}

impl MergeCells for TileHolder{
    fn merge(&self,cell:TileHolder){
        let mut my_resources = self.tile.borrow_mut().resources;
        let other_resources = self.tile.borrow().resources;
        my_resources.water += other_resources.water;
        my_resources.air += other_resources.air;
        my_resources.substrate = other_resources.substrate;
    }

}
