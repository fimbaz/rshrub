extern crate fnv;
mod rect;
use rect::{BucketPos,Pos};
use fnv::FnvHashMap;
use std::hash::Hash;
fn main(){
    let pos = BucketPos::new(0,0);
    let mut map = FnvHashMap::default();
    map.entry(pos).or_insert_with(||vec![pos]);
    let pos2 = BucketPos::new(1,1);
    println!("{:?}",map.get(&pos2));
}
