

//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region};
use grid::Grid;
use std::cell::RefCell;
use neighborhood::Neighborhood;
use tile::{Tile,Resources,Substrate};
pub struct BoringGame {
    pub grid: Grid<Tile>
}

impl BoringGame{
    pub fn new_tile(&mut self,pos:Pos,water: f32, air: f32) -> Result<(),String>{
        let mut bucket = self.grid.map.entry(BucketPos::from(pos)).or_insert(vec![]);
        let substrate = if pos.y < self.grid.ground_level{
            Substrate::Dirt()
        }
        else {
            Substrate::Space()
        };
        
        bucket.push(Tile::new(pos,water,air,substrate));
        Ok(())
    }

    pub fn new() -> BoringGame {
        let mut grid = Grid::new(30);
        let mut game= BoringGame { grid: grid };
        for i in (0..100){
            for j in (0..100) {
                game.new_tile(Pos::new(i,j),1.0,0.0);
            }
        }
        game
    }
    pub fn simulate(&mut self) {
        println!("hi");
        let region = Region::square(0,0,0); //dummy
        let mut all_the_neighbors_iter = self.grid.neighbor_query(&region); 
        while let Some(neighbors) = all_the_neighbors_iter.nexties() {
            if neighbors.top.is_some(){
                
            }
        }
    }
}
