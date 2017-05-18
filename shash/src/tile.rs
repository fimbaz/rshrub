use std::cell::{RefCell,Cell};
use grid::{Grid,GridCell};
use rect::{Pos,HasPos,Region,BucketPos,Iter};
use rustty::{Cell as TermCell};
impl HasPos for TileHolder {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.pos.x,y:self.pos.y};
    }
    fn set_pos(&mut self,x: usize, y: usize){
        self.pos = Pos { x , y };
    }
}


#[derive(Debug,Copy,Clone)]
pub enum Substrate{
    Dirt(),
    Space(),
}


#[derive(Debug,Copy,Clone)]
pub struct Resources{
    pub water: (f32,Option<f32>),
    pub air: (f32,Option<f32>),
    pub substrate: Substrate,
}
#[derive(Debug,Copy,Clone)]
pub struct Tile{
    pub resources: Resources
        
}
#[derive(Debug,Clone)]
pub struct TileHolder{
    pub pos: Pos,
    pub turn: Cell<usize>,
    pub tile: RefCell<Tile>,
}

impl Tile{
    pub fn new_v1(water:f32,air:f32,substrate: Substrate) -> Tile{
        //initialization is lazy here, but not in a good way.. just note
        //water.1 and air.1 are the 'backbuffers' -- copied to .0 before simulation begins.
        //similarly, turn is never incremented, but instead assigned out of the Grid.
        return Tile { resources: Resources {water:(0.0,Some(water)),air:(0.0,Some(air)),substrate} };
    }
    pub fn repr(&self)  -> TermCell{
        return TermCell::default();
    }
    pub fn stp_ground_repr() -> TermCell{
        return TermCell::with_char('.');
    }
    pub fn stp_air_repr() -> TermCell{
        return TermCell::with_char(' ');
    }
}

impl TileHolder {
    pub fn new_v1(pos:Pos,air:f32,water:f32,substrate:Substrate) -> TileHolder{
        TileHolder{pos:pos,tile:RefCell::new(Tile::new_v1(water,air,substrate)),turn:Cell::new(0)}
    }
}

impl GridCell for TileHolder{
    fn merge(&self,cell:Self){
        let mut my_resources = self.tile.borrow_mut().resources;
        let other_resources = self.tile.borrow().resources;
        my_resources.water.0 += other_resources.water.0;
        my_resources.air.0 += other_resources.air.0;
        my_resources.water.1 = other_resources.water.1; // this makes merging insane for now-- we may replace this whole impl with a stub until we need items
        my_resources.air.1 = other_resources.air.1;
        my_resources.substrate = other_resources.substrate;
    }

}
