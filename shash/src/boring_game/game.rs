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
pub const AIR_SENSITIVITY: f32       = 0.0001;
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
    fn new_tile(&self,pos:Pos,water: f32, air: f32) -> Result<TileHolder,String>{
        let substrate = if pos.y < self.ground_level{
            Substrate::Dirt()
        }
        else {
            Substrate::Space()
        };
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    pub fn insert_air(&mut self,pos: Pos,air:f32){
        let tile = self.new_tile(pos,0.0,air).unwrap();

        self.grid.insert(tile);
    }
    pub fn new_tile_with_substrate(&self,pos:Pos,water: f32, air: f32,substrate:Substrate) -> Result<TileHolder,String>{
        Ok(TileHolder::new_v1(pos,water,air,substrate))
    }
    pub fn new() -> BoringGame {
        let mut grid = Grid::new();
        
        let mut game= BoringGame { grid: grid, ground_level: 30 };
        let tile = game.new_tile(Pos::new(20,20),0.0,0.5).unwrap();
        game.grid.insert(tile);
        game
    }
    pub fn simulate(&mut self) {
        let region = Region::square(0,0,0); //dummy
        let mut all_the_neighbors_iter = self.grid.neighbor_query(&region);
        //1 iteration of this loop simulates one active cell
        while let Some(neighborhood) = all_the_neighbors_iter.nexties() {
            let mut enumerated_neighbors = neighborhood.neighbors.iter().enumerate();
            enumerated_neighbors.next().unwrap(); //ditch the point

            let point_rc=neighborhood.get_neighbor(Neighbor2::Point).unwrap();
            let point_tileholder: &TileHolder = Rc::borrow(&point_rc);
            
            let mut point = point_tileholder.tile.borrow_mut();
            if  point_tileholder.turn.get() == self.grid.borrow().turn.get(){
                //tile was inserted this turn-- please don't simulate it yet.
                continue;
            }
            let mut ppress_air = point.resources.air.0;
            let mut neighbor_exists = true;
            for (i,maybe_neighbor) in enumerated_neighbors {
                let neighbor_pos_1 = Neighbor2::from_usize(i);
                let neighbor_pos = neighbor_pos_1.as_ref().unwrap().get_pos(&point_tileholder.pos);
                if neighbor_pos.is_none(){
                    continue;
                }
                let mut npress_air = STANDARD_AIR_PRESSURE;
                if let Some(ref neighbor) =  *maybe_neighbor{
                    if  neighbor.turn.get() == self.grid.borrow().turn.get(){
                        //tile was inserted this turn-- please don't simulate it yet.
                        continue;
                    }
                    let mut ntile = neighbor.tile.borrow_mut();
                    if ntile.resources.air.1.is_some() {
                        ntile.resources.air.0 = ntile.resources.air.1.unwrap();
                        ntile.resources.air.1 = None;

                    }
                    neighbor.turn.set(self.grid.turn.get());
                    npress_air = ntile.resources.air.0;
                }else{
                    neighbor_exists = false;//if the cell gets interesting, we'll have to allocate it.
                }
                let dpress = ppress_air - npress_air;
                let flow = f32::max(f32::min(dpress, ppress_air/AIR_DAMPING), -npress_air/AIR_DAMPING);
                if f32::abs(flow) > AIR_SENSITIVITY{
                    npress_air +=flow;
                    ppress_air -=flow;
                }
                point.resources.air.1 = Some(ppress_air);
                if npress_air < (STANDARD_AIR_PRESSURE-AIR_SENSITIVITY) || npress_air > (STANDARD_AIR_PRESSURE+AIR_SENSITIVITY){
                    if let Some(ref neighbor) = *maybe_neighbor{
                        let mut ntile = neighbor.tile.borrow_mut();
                        ntile.resources.air.1 = Some(npress_air);
                        //get on with it.
                    }else{
                        let mut tile = self.new_tile(neighbor_pos.unwrap(),UNUSED_VALUE,npress_air).unwrap();
                        tile.turn.set(self.grid.borrow().turn.get());
                        self.grid.insert(tile);
                    }
                }
                
            }        
        }
        self.grid.next_turn();
    }
}

