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
                  
