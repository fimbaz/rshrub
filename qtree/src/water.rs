use std::iter;
use tree::{Region,QTree,Pos,HasPos};
use rustty::{Cell,Color,Attr};
use std::slice;
use std::cell::RefCell;
#[derive(Clone, Copy,Debug,PartialOrd,PartialEq)]
pub enum Material {
    Water(f32),
    Ground(),
    Air(f32),
}

#[derive(Debug,PartialOrd)]
pub struct Tile {
    pub pos: Pos,
    pub material: RefCell<Material>,
}

impl Tile{
    pub fn new(pos: &Pos,material: &Material) -> Tile{
        Tile { pos: pos.clone(),material: RefCell::new(material.clone()) }
    }
                      
}
//this is too clever, but since I keep deleting it and rewriting it, here it stays.
impl PartialEq for Tile {
    fn eq(&self,other:&Tile) -> bool {
        self.pos == other.pos
    }
}
impl HasPos for Tile{
    fn get_pos(&self) -> Pos {
        return self.pos.clone();
    }
}
impl<'a> HasPos for &'a Tile{
    fn get_pos(&self) ->  Pos {
        return self.pos.clone();
    }
}

pub struct Board {
    pub tree: QTree<Tile>,
    pub ground_level: u16,
}
impl Board{
    pub fn new(ground_level: u16) -> Board{
        Board{ tree: QTree::new(Region::square(0,0,16384),4),ground_level: ground_level}
    }
    pub fn simulate_air(){
        
    }
}

