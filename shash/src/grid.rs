use rect::{Pos,HasPos,Region,BucketPos};
use fnv::FnvHashMap;
use std::collections::hash_map;
use std::slice;
use std::iter::Filter;
struct Grid<P: HasPos> {
    ground_level: usize,
    map: FnvHashMap<BucketPos,Vec<P>>
}

struct RangeQuery<'t,P: HasPos + 't>{
    region: &'t Region,
    iter:  hash_map::Iter<'t,BucketPos,Vec<P>>,
    map: &'t FnvHashMap<BucketPos,Vec<P>>,
    inner_iter: slice::Iter<'t,P>
}
impl <P: HasPos> Grid<P>{
    pub fn new(ground_level: usize) -> Grid<P>{
        return Grid {map: FnvHashMap::default(),ground_level: ground_level};
    }
    pub fn range_query<'t>(&'t self,region: &'t Region) -> RangeQuery<'t,P>{
        let mut iter = self.map.iter();
        let inner_iter = iter.next().unwrap().1.iter();
        RangeQuery{region: region,map: &self.map,iter:iter,inner_iter: inner_iter}
    }
}

/*
struct NeighborQuery<'t,P: 't + HasPos>{
    map: &'t FnvHashMap<BucketPos,P>,
    query: &'t Region,
    main_iter:  hash_map::Iter<'t,BucketPos,P>
}
impl <'t,P: HasPos> NeighborQuery<'t,P>{
    pub fn new(map:&'t FnvHashMap<BucketPos,P>,query:&'t Region) -> NeighborQuery<'t,P>{
        NeighborQuery { map:map, query:query, main_iter:map.iter()}
    }
}
impl<'t,P: HasPos> Iterator for NeighborQuery<'t,P>{
    type Item = &'t P;
    fn next(&mut self) -> Option<&'t P>{
        None
    }
}
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
