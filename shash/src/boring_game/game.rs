//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region,HasPos};
use std::borrow::Borrow;
use std::rc::Rc;
use grid::Grid;
use std::cell::RefCell;
use neighborhood::{Neighbor2,Neighborhood2};
use tile::{Tile,TileHolder,Resources,Substrate};

pub const AIR_SENSITIVITY: f32       = 0.1;
pub const STANDARD_AIR_PRESSURE: f32 = 1.0;
pub const AIR_DAMPING: f32           =  8.0; //AIR_DAMPING must be >= number of neighbors to preserve conservation of mass (untested).
pub struct BoringGame {
    pub grid: Grid<TileHolder>,
    pub ground_level: usize,
        
}

impl BoringGame{
    pub fn new_tile(&mut self,pos:Pos,water: f32, air: f32) -> Result<TileHolder,String>{
        let substrate = if pos.y < self.ground_level{
            Substrate::Dirt()
        }
        else {
            Substrate::Space()
        };
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    pub fn new_tile_with_substrate(&mut self,pos:Pos,water: f32, air: f32,substrate:Substrate) -> Result<TileHolder,String>{
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    
    pub fn new() -> BoringGame {
        let mut grid = Grid::new();
        let mut game= BoringGame { grid: grid, ground_level: 30 };
        for i in 0..100 {
            for j in 0..100 {
                game.new_tile(Pos::new(i,j),0.0,1.0);
            }
        }
        game
    }
    pub fn simulate(&mut self) {
        let region = Region::square(0,0,0); //dummy
        let mut all_the_neighbors_iter = self.grid.neighbor_query(&region);
        //1 iteration of this loop simulates one active cell
        while let Some(neighborhood) = all_the_neighbors_iter.nexties() {
            let mut enumerated_neighbors = neighborhood.neighbors.iter().enumerate();
            let (i,point_ref)            = enumerated_neighbors.next().unwrap();
            let rc                       = point_ref.as_ref().unwrap().clone();
            
            let point_ref: &TileHolder = Rc::borrow(&rc);
            let point = point_ref.tile.borrow_mut();
            let mut ppress_air = point.resources.air.0;
            let mut neighbor_exists = true;
            for (i,maybe_neighbor_ref) in enumerated_neighbors {//maybe_neighbor_ref is actually an Option<TileHolder>, but Tileholder is just a Pos and a RefCell.
                let neighbor_ref = maybe_neighbor_ref;
                let mut npress_air = 0.0;
                if neighbor_ref.is_some(){ //Some(ref neighbor_ref) = *maybe_neighbor_ref {
                    let mut neighbor = neighbor_ref.as_ref().unwrap().tile.borrow_mut();
                    npress_air = neighbor.resources.air.0;
                }else{
                    npress_air = STANDARD_AIR_PRESSURE;
                    neighbor_exists = false;//if the cell gets interesting, we'll have to allocate it.
                }
                let dpress = npress_air - ppress_air;
                let flow = f32::min(f32::max(dpress, ppress_air/AIR_DAMPING), -npress_air/AIR_DAMPING);
                if flow > AIR_SENSITIVITY{
                    npress_air +=flow;
                    ppress_air -=flow;
                }
                if npress_air != STANDARD_AIR_PRESSURE{
                    if neighbor_ref.is_some(){
                        let mut neighbor = neighbor_ref.as_ref().unwrap().tile.borrow_mut();
                        neighbor.resources.air.1 = npress_air;
                        //get on with it.
                    }else{
                        let neighbor_pos = Neighbor2::from_usize(i).unwrap().get_pos(&point_ref.pos);
                    }
                }
            }        
        }
    }
}
