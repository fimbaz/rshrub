use rect::{Pos,HasPos,Region,BucketPos,Iter};
use fnv::FnvHashMap;
use std::collections::hash_map;
use std::slice;
use std::iter::Filter;
use neighborhood::Neighborhood2;
pub trait GridCell {
    fn  merge(&self,Self);
}
pub struct Grid<P: HasPos> {
    pub map: FnvHashMap<BucketPos,Vec<P>>
}

pub struct RangeQuery<'t,'r,P: HasPos + 't>{
    bucket_keys: Iter<'r>,
    region: &'r  Region,
    map: &'t FnvHashMap<BucketPos,Vec<P>>,
    points: slice::Iter<'t,P>
}
impl <P: HasPos + GridCell> Grid<P>{
    pub fn new() -> Grid<P>{
        return Grid {map: FnvHashMap::default()};
    }
    pub fn insert(&mut self,point: P){
        let bucket = self.map.entry(BucketPos::from(point.get_pos())).or_insert(vec![]);
        if let Some(ref existing_point) = bucket.iter().find(|x|x.get_pos()==point.get_pos()){
            existing_point.merge(point);
        }
            
    }
    //TODO remove internal calls and save a hash.
    pub fn translate(&mut self,old_pos: Pos,new_pos: Pos){
        if let Some(mut point) = self.delete(old_pos) {
            point.set_pos(new_pos.x,new_pos.y);
            self.insert(point);
        }
    }
    pub fn delete(&mut self,pos: Pos) -> Option<P>{
        if let Some(bucket) = self.map.get_mut(&BucketPos::from(pos)){
            if let Some(i) = bucket.iter().position(|x|x.get_pos()==pos){
                let point = bucket.remove(i);
                return Some(point);
            }
        }
        return None;
    }
    pub fn range_query<'t,'r>(&'t self,region: &'r Region) -> RangeQuery<'t,'r,P>{
        RangeQuery{bucket_keys:region.iter(),map:&self.map,region:region,points: (&[]).iter()}
    }
    pub fn neighbor_query<'t,'r>(&'t self,query:&'r Region) -> NeighborQuery<'t,P>{
        let mut main_iter = self.map.iter();
        let mut bucket_iter = (&[]).iter();
        if let Some((key,bucket_vec)) = main_iter.next(){
            bucket_iter = bucket_vec.iter()
        }
        NeighborQuery { grid:self, main_iter:main_iter,nhood: Neighborhood2::default(),bucket:  bucket_iter,region: Region::square(0,0,0)}
    }
}
impl <'t,'r,P: HasPos> Iterator for  RangeQuery<'t,'r,P> {
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


struct NeighborQuery<'t,P: HasPos + GridCell + 't>{
    grid: &'t Grid<P>,
    main_iter:  hash_map::Iter<'t,BucketPos,Vec<P>>,
    bucket: slice::Iter<'t,P>,
    nhood: Neighborhood2<'t,P>,
    region: Region,
        
}

impl <'t,P: HasPos + GridCell> NeighborQuery<'t,P>{
    pub fn neighbor_query(grid:&'t Grid<P>,query:&'t Region) -> NeighborQuery<'t,P>{
        let mut main_iter = grid.map.iter();
        let mut bucket_iter = (&[]).iter();
        if let Some((key,bucket_vec)) = main_iter.next(){
            bucket_iter = bucket_vec.iter()
        }
        NeighborQuery { grid:grid, main_iter:main_iter,nhood: Neighborhood2::default(),bucket:  bucket_iter,region: Region::square(0,0,0)}
    }
}
impl<'t,P: HasPos + GridCell>  NeighborQuery<'t,P>{
    //returns a value that continues the iter borrow, so
    //'nexties' can't be called again until the neighborhood borrowed from the previous call
    //is out of scope.  This is to save us allocating a vec every single time (since the points we're
    //accessing aren't contiguous in the heap)
    pub fn nexties<'r,'s>(&'r mut self) -> Option<&'s Neighborhood2<'r,P>>{
        'outer: loop{
            for point in &mut self.bucket{
                let pos = point.get_pos();
                self.region = Region::rectangle(pos.x.saturating_sub(1),pos.y.saturating_sub(1),
                                                if pos.x == 0 { 1 } else { 2 },if pos.y == 0 {1} else {2} );
                let mut rq = self.grid.range_query(&self.region);
                self.nhood.populate(&point,&mut rq);
                return Some(&self.nhood);
            }
            if let Some((key,bucket_vec)) = self.main_iter.next(){
                self.bucket = bucket_vec.iter();
            }else{
                break;
            }
        }
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
