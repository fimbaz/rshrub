extern crate fnv;
mod rect;
mod grid;
mod neighborhood;
use rect::{BucketPos,Pos,Region,HasPos,Iter};
use grid::{Grid};
use fnv::FnvHashMap;
use std::hash::Hash;


fn main(){

    let mut grid = Grid::new(30);
    for i in 0..10{
        for j in 0..10{
            let pos = BucketPos::new(i,j);
            let  value = grid.map.entry(pos).or_insert(vec![]);
            value.push(Pos::new(i,j));
        }
    }
    let region = Region::square(0,0,0);
    let mut query = grid.neighbor_query(&region);
    while let Some(item) = query.nexties(){
        println!("{:?}",item);
    }
}
