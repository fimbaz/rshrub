use rect::{Pos,HasPos,Region,BucketPos,Iter};
use fnv::FnvHashMap;
use std::collections::hash_map;
use std::slice;
use std::iter::Filter;
pub struct Grid<P: HasPos> {
    pub ground_level: usize,
    pub map: FnvHashMap<BucketPos,Vec<P>>
}

pub struct RangeQuery<'t,P: HasPos + 't>{
    bucket_keys: Iter<'t>,
    region: &'t Region,
//    iter:  hash_map::Iter<'t,BucketPos,Vec<P>>,
    map: &'t FnvHashMap<BucketPos,Vec<P>>,
    points: slice::Iter<'t,P>
}
impl <P: HasPos> Grid<P>{
    pub fn new(ground_level: usize) -> Grid<P>{
        return Grid {map: FnvHashMap::default(),ground_level: ground_level};
    }
    pub fn range_query<'t>(&'t self,region: &'t Region) -> RangeQuery<'t,P>{
        RangeQuery{bucket_keys:region.iter(),map:&self.map,region:region,points: (&[]).iter()}
    }
}
impl <'t,P: HasPos> Iterator for  RangeQuery<'t,P> {
    type Item = &'t P;
    fn next(&mut self) -> Option<&'t P> {
        'outer: loop{
            for  ref point in &mut self.points {
                let pos = point.get_pos();
                if self.region.contains(&pos){
                    return Some(point);
                }
            }
            for bucket in &mut self.bucket_keys{
                if let Some(ref vec) = self.map.get(&bucket){
                    self.points = vec.iter();
                }
                continue 'outer;
            }
            break;
        }
        None
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
