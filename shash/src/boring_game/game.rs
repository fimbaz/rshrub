

//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region};
use grid::Grid;
use std::cell::RefCell;
use neighborhood::Neighborhood;
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
        println!("hi");
        let region = Region::square(0,0,0); //dummy
        let mut all_the_neighbors_iter = self.grid.neighbor_query(&region); 
        while let Some(neighborhood) = all_the_neighbors_iter.nexties() {
            for (i,neighbor) in neighborhood.neighbors.iter().enumerate(){
                
            }

        }
    }
}
