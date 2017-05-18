use rect::{Pos,HasPos,Region,BucketPos,Iter};
use fnv::FnvHashMap;
use std::collections::hash_map;
use std::slice;
use std::iter::Filter;
use std::cell::{RefCell,Cell};
use std::cell::{RefMut,Ref};
use std::rc::Rc;
use std::fmt::Debug;
use std::borrow::Borrow;
use neighborhood::Neighborhood2;
pub trait GridCell {
    fn  merge(&self,Self);
}
type Bucket<P> = Rc<RefCell<Vec<Rc<P>>>>;
type Map<P> = FnvHashMap<BucketPos,Bucket<P>>;

#[derive(Debug)]
pub struct Grid<P: HasPos + Debug> {
    pub map: Rc<RefCell<Map<P>>>,
    pub turn: Rc<Cell<usize>>,
}

pub struct RangeQuery<'r,P: HasPos + GridCell + Debug>{
    bucket_keys: Iter<'r>,
    region: &'r  Region,
    grid: Grid<P>,
    bucket_ref: Option<Bucket<P>>,
    cursor_pos: usize,
}
impl <P: HasPos + GridCell + Debug> Grid<P>{
    pub fn new() -> Grid<P>{
        return Grid {turn: Rc::new(Cell::new(0)),map: Rc::new(RefCell::new(FnvHashMap::default()))};
    }
    pub fn clone(&self) -> Grid<P>{
        return Grid{turn:self.turn.clone(),map:self.map.clone()};
    }
    pub fn next_turn(&self){
        let cell = Rc::borrow(&self.turn);
        Cell::set(cell,cell.get()+1);
    }
    pub fn borrow_map(&self) -> RefMut<Map<P>>{
        return RefCell::borrow_mut(Rc::borrow(&self.map));
    }
    pub fn get_bucket(&self,pos: BucketPos) -> Option<Bucket<P>>{
        return self.borrow_map().get(&pos).map(|x|x.clone());
    }
    pub fn occupied_buckets(&self) -> Vec<BucketPos>{
        self.borrow_map().keys().map(|x|x.clone()).collect()
    }
    pub fn insert(&mut self,point: P){
        let mut map = self.borrow_map();
        let rc_ref_bucket: &mut Rc<RefCell<Vec<Rc<P>>>> = map.entry(BucketPos::from(point.get_pos())).or_insert(Rc::new(RefCell::new(vec![])));
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
        if let Some(bucket_ref) = self.borrow_map().get_mut(&BucketPos::from(pos)){
            let mut bucket = RefCell::borrow_mut(Rc::borrow(bucket_ref));
            if let Some(i) = bucket.borrow().iter().position(|x|x.get_pos()==pos){
                let rc = bucket.remove(i);
                return Rc::try_unwrap(rc).ok();
            }
        }
        return None;
    }
    pub fn range_query<'t,'r>(&'t self,region: &'r Region) -> RangeQuery<'r,P>{
        let mut bucket_keys = region.iter();
        
        if let Some(bucket_ref) = self.borrow_map().get(&bucket_keys.next().unwrap()){
            RangeQuery{bucket_keys:bucket_keys,grid:(*self).clone(),region:region,bucket_ref:Some(bucket_ref.clone()),cursor_pos:0}
        }else{
            RangeQuery{bucket_keys:bucket_keys,grid:self.clone(),region:region,bucket_ref:None,cursor_pos:0}
        }
    }
    
    pub fn neighbor_query<'r>(& self,query:&'r Region) -> NeighborQuery<P>{
        let mut bucket_list = self.occupied_buckets();
        let map = self.borrow_map();
        let mut bucket = map.get(bucket_list.get(0).expect("Grid must contain at least one point.")).expect("occupied_buckets must return only keys to buckets in Grid");
        let mut neighbors = vec![None,None,None,None,None,None,None,None,None].into_boxed_slice();
        NeighborQuery { neighbors: neighbors, grid:self.clone(), bucket_list:bucket_list,bucket:bucket.clone(),region: Region::square(0,0,0),bucket_cursor:0,bucket_list_i:0}
    }
}

impl <'r,P: HasPos + Debug + GridCell> Iterator for  RangeQuery<'r,P> {
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
                if let Some(vec) = self.grid.get_bucket(bucket){
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


struct NeighborQuery<P: HasPos + GridCell + Debug>{
    grid: Grid<P>,
    bucket_list:  Vec<BucketPos>,
    bucket_list_i: usize,
    bucket: Bucket<P>,
    bucket_cursor: usize,
    neighbors: Box<[Option<Rc<P>>]>, //so Neighborhood2 can Drop w/o realloc
    region: Region,
        
}

impl<P: HasPos + GridCell + Debug>  NeighborQuery<P>{
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
                    let mut nhood = Neighborhood2::new(self.grid.clone(),&mut self.neighbors,self.bucket.clone());
                    nhood.populate(&point,&mut rq);
                    self.bucket_cursor +=1;
                    return Some(nhood);
                }
                
            }
            self.bucket_list_i +=1;
            if let Some(bucket_key) = self.bucket_list.get(self.bucket_list_i){
                let bucket_vec = self.grid.get_bucket(*bucket_key).unwrap();
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
