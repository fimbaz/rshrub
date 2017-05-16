//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region,HasPos};
use std::borrow::Borrow;
use std::rc::Rc;
use grid::Grid;
use std::cell::RefCell;
use neighborhood::{Neighbor2,Neighborhood2};
use tile::{Tile,TileHolder,Resources,Substrate};
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
            for (i,maybe_neighbor_ref) in enumerated_neighbors {//maybe_neighbor_ref is actually an Option<TileHolder>, but Tileholder is just a Pos and a RefCell.
                let mut npress_air = 0.0;
                if let Some(ref neighbor_ref) = *maybe_neighbor_ref {
                    let mut neighbor = neighbor_ref.tile.borrow_mut();
                    npress_air = neighbor.resources.air.0;
                }else{
                    npress_air = 0.0;
                    let neighbor_pos = Neighbor2::from_usize(i).unwrap().get_pos(&point_ref.pos);
                }
                
            }
            
        }

    }
}
