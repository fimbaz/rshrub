//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region,HasPos};
use std::borrow::Borrow;
use std::rc::Rc;
use grid::Grid;
use std::cell::RefCell;
use neighborhood::{Neighbor2,Neighborhood2};
use tile::{Tile,TileHolder,Resources,Substrate};
pub const UNUSED_VALUE: f32       = 0.0;
pub const AIR_SENSITIVITY: f32       = 0.01;
pub const STANDARD_AIR_PRESSURE: f32 = 1.0;
pub const AIR_DAMPING: f32           =  8.0; //AIR_DAMPING must be >= number of neighbors to preserve conservation of mass (untested).
pub struct BoringGame {
    pub grid: Grid<TileHolder>,
    pub ground_level: usize,
        
}
//A tile is ready to delete if
//1. it has been marked for deletion and
//2. the turn on which it was so marked has ended.

impl BoringGame{
    pub fn new_tile(&self,pos:Pos,water: f32, air: f32) -> Result<TileHolder,String>{
        let substrate = if pos.y < self.ground_level{
            Substrate::Dirt()
        }
        else {
            Substrate::Space()
        };
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    pub fn new_tile_with_substrate(&self,pos:Pos,water: f32, air: f32,substrate:Substrate) -> Result<TileHolder,String>{
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    
    pub fn new() -> BoringGame {
        let mut grid = Grid::new();
        
        let mut game= BoringGame { grid: grid, ground_level: 30 };
        let tile = game.new_tile(Pos::new(50,50),0.0,100.0).unwrap();
        game.grid.insert(tile);
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
                let neighbor_pos = Neighbor2::from_usize(i).expect("i should always be between 0 and 8").get_pos(&point_ref.pos);
                if neighbor_pos.is_none(){
                    continue;
                }
                let neighbor_ref = maybe_neighbor_ref;
                let mut npress_air = STANDARD_AIR_PRESSURE;
                if neighbor_ref.is_some(){ //Some(ref neighbor_ref) = *maybe_neighbor_ref {
                    let mut tile_holder = neighbor_ref.as_ref().unwrap();
                    let mut neighbor = tile_holder.tile.borrow_mut();
                    if neighbor.resources.air.1.is_some(){
                        neighbor.resources.air.0 = neighbor.resources.air.1.unwrap();
                        neighbor.resources.air.1 = None;
                        tile_holder.turn.set(self.grid.turn.get());
                    }
                    npress_air = neighbor.resources.air.0;
                }else{
                    neighbor_exists = false;//if the cell gets interesting, we'll have to allocate it.
                }
                let dpress = npress_air - ppress_air;
                let flow = f32::max(f32::min(dpress, ppress_air/AIR_DAMPING), -npress_air/AIR_DAMPING);
                if f32::abs(flow) > AIR_SENSITIVITY{
                    println!("{:?}",flow);
                    npress_air +=flow;
                    ppress_air -=flow;
                }
                if npress_air < (STANDARD_AIR_PRESSURE-AIR_SENSITIVITY) || npress_air > (STANDARD_AIR_PRESSURE+AIR_SENSITIVITY){
                    if neighbor_ref.is_some(){
                        let mut neighbor = neighbor_ref.as_ref().unwrap().tile.borrow_mut();
                        neighbor.resources.air.1 = Some(npress_air);
                        //get on with it.
                    }else{
                        let tile = self.new_tile(neighbor_pos.expect("nonexistent neighbors have been filtered out."),UNUSED_VALUE,npress_air);
                        self.grid.insert(tile.unwrap());
                    }
                }
                
            }        
        }
        self.grid.next_turn();
        
    }
}

