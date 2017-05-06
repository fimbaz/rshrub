#![feature(test,conservative_impl_trait)]
extern crate fnv;
extern crate ref_slice;
mod rect;
mod grid;
mod neighborhood;
mod tile;
mod game;
#[cfg(test)]
mod tests {
    extern crate test;
    use fnv::FnvHashMap;
    use self::test::Bencher;
    use grid::{Grid,RangeQuery};
    use rect::{BucketPos,Pos,Iter,Region,HasPos};
    #[test]
    fn it_works() {
        let mut map = FnvHashMap::default();
        map.insert(1,"one");
        map.insert(2,"one");
        for v in map.iter(){
            println!("{:?}",v);
        }
        
    }
    #[test]
    fn rect_iterator(){
        let region = Region::square(0,0,100);
        println!("hi");
        for pos in region.iter(){
            println!("{:?}",pos);
        }
    }
    #[cfg(feature="bench")]
    #[bench]
    fn bench_ins(b: &mut Bencher){
        let mut map = FnvHashMap::default();
        for i in (0..1000){
            for j in (0..1000){
                let val = map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        let clj =  (|| {
            map.iter().fold(0,|i,x| {test::black_box(x); i+1})
        });
        println!("{:?}",clj());
        b.iter(clj);
    }


    #[cfg(feature="bench")]
    #[bench]
    fn rq_neighquery(b: &mut Bencher){
        let mut grid = Grid::new(30);
        for i in (0..100){
            for j in (0..100){
                let val = grid.map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        let region = Region::square(0,0,0);
        let mut query = grid.neighbor_query(&region);

        let mut clj =|| {
            let mut query = grid.neighbor_query(&region);
            let mut count =0;
            let mut element_count = 0;
            {
                while let Some(nbors) =query.nexties(){
                    count += nbors.len();
                    element_count += 1;
                }
            }
            (count,element_count)
        };
        println!("{:?}",clj());
        b.iter(clj);
    }
    
    #[cfg(feature="bench")]
    #[bench]
    fn rq_bigquery(b: &mut Bencher){
        let mut grid = Grid::new(30);
        for i in (0..1000){
            for j in (0..1000){
                let val = grid.map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        let mut ncount = 0;
        for val in grid.map.values(){
            ncount += val.len()
        }
        println!("{:?}",ncount);
        let clj = || {
            let mut count =0;
            let it = grid.map.iter();
            for item in it{
                for point in item.1{
                    let pos = point.get_pos();
                    let x = pos.x; let y = pos.y;
                    let region = Region::rectangle((x as usize).saturating_sub(1),(y as usize).saturating_sub(1),
                                                   if x == 0 { 1 } else { 2 },if y == 0 {1} else {2} );                    
                    let ncount = grid.range_query(&region).fold(0,|i,x| { test::black_box(x);  i+1});
                    count += ncount;
                }
            }
        };
        println!("{:?}",clj());
        b.iter(clj);
    
    }
    #[test]
    fn test_rq2(){
        let mut grid = Grid::new(30);
        for i in (0..100){
            for j in (0..100){
                let val = grid.map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        let region = Region { x: 0, y: 5, width: 2, height: 2 };
        
        let vec: Vec<&Pos> = grid.range_query(&region).collect();
        println!("{:?}",vec);
    }
    #[test]
    fn test_rq(){
        let mut grid = Grid::new(30);
        for i in (0..1000){
            for j in (0..1000){
                let val = grid.map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        let reg= Region { x: 0, y: 2, width: 2, height: 2 };
        for p in grid.range_query(&Region::square(5,5,5)){
            println!("{:?}",p);
        }
                         
        
    }

}
