extern crate fnv;
mod rect;
use rect::{BucketPos,Pos,Region,HasPos};
use fnv::FnvHashMap;
use std::hash::Hash;

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

fn main(){

    let mut map = FnvHashMap::default();
    for i in 0..100{
        for j in 0..100{
            let pos = BucketPos::new(i,j);
            let  value = map.entry(pos).or_insert(vec![]);
            value.push(Pos::new(i,j));
        }
    }
    let pos2 = BucketPos::new(1,1);
    println!("{:?}",map.get(&pos2));
}
