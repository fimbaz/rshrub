
//A reference game to make sure our data structures aren't friggin crazy.
use fnv::FnvHashMap;
use rect::{BucketPos,Pos,Region};
use grid::Grid;
use neighborhood::Neighborhood;
use tile::{InnerTile,Tile,Resources,Substrate};
pub struct BoringGame {
    pub grid: Grid<Tile>
}

impl BoringGame{
    pub fn new_tile(&mut self,resources: Resources,pos:Pos) -> Tile{
        let mut entry = self.grid.map.entry(BucketPos::from(pos)).or_insert(vec![]);
        let substrate = if pos.y < self.grid.ground_level{
            Substrate::Dirt()
        }
        else {
            Substrate::Space()
        };
        
        let tile = Tile{pos:pos.clone(),
                        data:Box::new(InnerTile{resources:resources}),
                        substrate:substrate};
        tile
    }

    pub fn new() -> BoringGame {
        let mut grid = Grid::new(30);
        let mut game= BoringGame { grid: grid };
        for i in (0..100){
            for j in (0..100) {
                game.new_tile(Resources{water: 1.0, air: 0.0 },Pos::new(i,j));
            }
        }
        BoringGame { grid : Grid::new(30)}
    }
    pub fn simulate(&mut self) {
        let region = Region::square(0,0,0); //dummy
        let mut all_the_neighbors_iter = self.grid.neighbor_query(&region); 
        while let Some(neighbors) = all_the_neighbors_iter.nexties() {
            if neighbors.top.is_some(){
                println!("we have a neighbor");
            }
        }
    }
}
