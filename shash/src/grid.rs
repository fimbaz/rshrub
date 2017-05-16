use rect::{Pos,HasPos,Region,BucketPos,Iter};
use fnv::FnvHashMap;
use std::collections::hash_map;
use std::slice;
use std::iter::Filter;
use std::cell::RefCell;
use std::cell::{RefMut,Ref};
use std::rc::Rc;
use std::fmt::Debug;
use std::borrow::Borrow;
use neighborhood::Neighborhood2;
pub trait GridCell {
   fn  merge(&self,Self);
}
#[derive(Debug)]
pub struct Grid<P: HasPos + Debug> {
    pub map: FnvHashMap<BucketPos,Rc<RefCell<Vec<Rc<P>>>>>,
}

pub struct RangeQuery<'t,'r,P: HasPos + 't>{
    bucket_keys: Iter<'r>,
    region: &'r  Region,
    map: &'t FnvHashMap<BucketPos,Rc<RefCell<Vec<Rc<P>>>>>,
    bucket_ref: Option<Rc<RefCell<Vec<Rc<P>>>>>,
    cursor_pos: usize,
}
impl <P: HasPos + GridCell + Debug> Grid<P>{
    pub fn new() -> Grid<P>{
        return Grid {map: FnvHashMap::default()};
    }
    pub fn occupied_buckets(&self) -> Vec<BucketPos>{
        self.map.keys().map(|x|x.clone()).collect()
    }
    pub fn insert(&mut self,point: P){
        let rc_ref_bucket: &mut Rc<RefCell<Vec<Rc<P>>>> = self.map.entry(BucketPos::from(point.get_pos())).or_insert(Rc::new(RefCell::new(vec![])));
        let mut pos_in_bucket = None;
        {
            let bucket   = RefCell::borrow(Rc::borrow(rc_ref_bucket));
            pos_in_bucket =  bucket.iter().position(|x|x.get_pos()==point.get_pos()).clone();
        }

        if pos_in_bucket.is_some(){
            let mut bor = RefCell::borrow(Rc::borrow(rc_ref_bucket));
            let mut existing_point = bor.get(pos_in_bucket.unwrap()).unwrap();
            existing_point.merge(point);
            
        }else{
            let mut bor = RefCell::borrow_mut(Rc::borrow(rc_ref_bucket));
            bor.push(Rc::new(point));
        }


        
    }
    pub fn delete(&mut self,pos: Pos) -> Option<P>{
        if let Some(bucket_ref) = self.map.get_mut(&BucketPos::from(pos)){
            let mut bucket = RefCell::borrow_mut(Rc::borrow(bucket_ref));
            if let Some(i) = bucket.borrow().iter().position(|x|x.get_pos()==pos){
                let rc = bucket.remove(i);
                return Rc::try_unwrap(rc).ok();
            }
        }
        return None;
    }
    pub fn range_query<'t,'r>(&'t self,region: &'r Region) -> RangeQuery<'t,'r,P>{
        let mut bucket_keys = region.iter();
        
        if let Some(bucket_ref) = self.map.get(&bucket_keys.next().unwrap()){
            RangeQuery{bucket_keys:bucket_keys,map:&self.map,region:region,bucket_ref:Some(bucket_ref.clone()),cursor_pos:0}
        }else{
            RangeQuery{bucket_keys:bucket_keys,map:&self.map,region:region,bucket_ref:None,cursor_pos:0}
        }
    }
    
    pub fn neighbor_query<'t,'r>(&'t self,query:&'r Region) -> NeighborQuery<'t,P>{
        let mut main_iter = self.map.iter();
        let mut bucket = main_iter.next().unwrap();
        let mut neighbors = vec![None,None,None,None,None,None,None,None,None].into_boxed_slice();
        NeighborQuery { neighbors: neighbors, grid:self, main_iter:main_iter,bucket:bucket.1.clone(),region: Region::square(0,0,0),bucket_cursor:0}
    }
}

impl <'t,'r,P: HasPos + Debug> Iterator for  RangeQuery<'t,'r,P> {
    type Item = Rc<P>;
    fn next(&mut self) -> Option<Rc<P>> {
        'outer: loop{
            if let Some(ref bucket_ref) = self.bucket_ref{
                let mut bucket = RefCell::borrow(Rc::borrow(bucket_ref));
                while let Some(point) = bucket.get(self.cursor_pos){
                    let pos = point.get_pos();
                    self.cursor_pos +=1;
                    if self.region.contains(&pos){
                        return Some(point.clone());
                    }
                }
            }
            for bucket in &mut self.bucket_keys{
                if let Some(vec) = self.map.get(&bucket){
                    self.bucket_ref = Some(vec.clone());
                        self.cursor_pos = 0;
                }
                continue 'outer;
            }
            break 'outer;
        }
        None
    }
}


struct NeighborQuery<'t,P: HasPos + GridCell + 't + Debug>{
    grid: &'t Grid<P>,
    main_iter:  hash_map::Iter<'t,BucketPos,Rc<RefCell<Vec<Rc<P>>>>>,
    bucket: Rc<RefCell<Vec<Rc<P>>>>,
    bucket_cursor: usize,
    neighbors: Box<[Option<Rc<P>>]>, //so Neighborhood2 can Drop w/o realloc
    region: Region,
        
}

impl<'t,P: HasPos + GridCell + Debug>  NeighborQuery<'t,P>{
    //returns a value that continues the iter borrow, so
    //'nexties' can't be called again until the neighborhood borrowed from the previous call
    //is out of scope.  This is to save us allocating a vec every single time (since the points we're
    //accessing aren't contiguous in the heap)
    pub fn nexties<'r>(&'r mut self) -> Option<Neighborhood2<'r,P>>{
        'outer: loop{
            {
                let bucket = RefCell::borrow(Rc::borrow(&self.bucket));
                while let Some(point) = bucket.get(self.bucket_cursor){
                    let pos = point.get_pos();
                    self.region = Region::rectangle(pos.x.saturating_sub(1),pos.y.saturating_sub(1),
                                                    if pos.x == 0 { 1 } else { 2 },if pos.y == 0 {1} else {2} );
                    let mut rq = self.grid.range_query(&self.region);
                    let mut nhood = Neighborhood2::new(self.grid,&mut self.neighbors,self.bucket.clone());
                    nhood.populate(&point,&mut rq);
                    self.bucket_cursor +=1;
                    return Some(nhood);
                }
                
            }
            if let Some((key,bucket_vec)) = self.main_iter.next(){
                self.bucket = bucket_vec.clone();
                self.bucket_cursor = 0;

                continue 'outer;                
            }else{
                break 'outer;
            }
        }
        None
    }
}
