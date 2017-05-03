use rect::{Pos,HasPos,Region,BucketPos};
use fnv::FnvHashMap;
struct Grid<P> {
    height: usize,
    width: usize,
    bucket_size: usize,
    map: FnvHashMap<BucketPos,P>
}

impl <P> Grid<P> {
    pub fn new(width: usize,height: usize,bucket_size: usize) -> Grid<P>{
        let map = FnvHashMap::default();
        Grid { height,width,bucket_size,map}
    }
                                                              
}
                  
struct NeighborQuery<'t,P: 't + HasPos>{
    map: &'t FnvHashMap<BucketPos,P>,
    query: &'t Region,
    bucket: &'t Vec<P>,
}
impl<'t,P: HasPos> Iterator for NeighborQuery<'t,P>{
    type Item = &'t P;
    fn next(&mut self) -> Option<&'t P>{
        None
    }
}
/*
fn get_neighbors(map:&FnvHashMap<BucketPos,Vec<Pos>>,bpos: BucketPos) -> Vec<&Pos>{
    let x = bpos.0.x;
    let y = bpos.0.y;
    let width = if x == 0 { 1 } else { 2 };
    let height = if y == 0 { 1 } else { 2 };
    let nhood = Region {x,y,width,height};
    let npos = Pos { x, y };
    for pos in vec![BucketPos::new(x,y),BucketPos::new(x+width,y),BucketPos::new(x,y+height),BucketPos::new(x+width,y+height)]{
        if let Some(neighbors) = map.get(&pos){
            return neighbors.iter().filter(|n|nhood.contains(*n)).collect()
        }
    }
    
}
*/
