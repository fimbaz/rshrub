use grid::Grid;
use rect::{Pos,HasPos,Region,BucketPos,Iter};
impl HasPos for Tile {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.pos.x,y:self.pos.y};
    }
}

#[derive(Copy,Clone)]
pub enum Substrate{
    Dirt(),
    Space(),
}
#[derive(Copy,Clone)]
pub struct Resources{
    pub water: f32,
    pub air: f32,
}
pub struct InnerTile{
    pub resources: Resources
}
pub struct Tile{
    pub pos: Pos,
    pub substrate: Substrate,
    pub data: Box<InnerTile>,
}



impl Tile {
    pub fn merge(&mut self,tile: &Tile){
        self.data.resources.water += tile.data.resources.water;
        self.data.resources.air += tile.data.resources.air;
        self.substrate = tile.substrate;
    }
}

