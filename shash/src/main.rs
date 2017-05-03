extern crate fnv;
mod rect;
use rect::{BucketPos,Pos,Region,HasPos};
use fnv::FnvHashMap;
use std::hash::Hash;


fn main(){

    let mut map = FnvHashMap::default();
    for i in 0..100{
        for j in 0..100{
            let pos = BucketPos::new(i,j);
            let  value = map.entry(pos).or_insert(vec![]);
            value.push(Pos::new(i,j));
        }
    }
    let pos2 = BucketPos::new(2,2);
    println!("{:?}",map.get(&pos2));
}
