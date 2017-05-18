#![feature(test,conservative_impl_trait)]
extern crate fnv;
extern crate ref_slice;
extern crate rustty;
pub mod rect;
mod grid;
mod neighborhood;
mod boring_game;
mod tile;
mod ui;

#[cfg(test)]
mod tests {
    extern crate test;
    use fnv::FnvHashMap;
    use self::test::Bencher;
    use grid::{Grid,RangeQuery};
    use rect::{BucketPos,Pos,Iter,Region,HasPos};
    use grid::GridCell;
    #[derive(Debug,Eq,PartialEq)]
    struct ToyPos ( Pos );
    impl HasPos for ToyPos {
        fn get_pos(&self) -> Pos{
            self.0
        }
        fn set_pos(&mut self,x: usize,y: usize){
            self.0 = Pos { x, y};
        }

    }

    impl GridCell for ToyPos {
        fn merge(&self,point:ToyPos) {
        }
    }
    impl ToyPos {
        pub fn new(x: usize, y: usize) -> ToyPos{
            ToyPos(Pos::new(x,y))
        }
    }
    
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
    fn rq_neighquery(b: &mut Bencher){
        let mut grid = Grid::new();
        for i in (0..1000){
            for j in (0..1000){
                let val = grid.insert(ToyPos::new(i,j));

            }
        }
        let region = Region::square(0,0,0);
        let mut query = grid.neighbor_query(&region);

        let mut clj =|| {
            let mut query = grid.neighbor_query(&region);
            let mut count =0;
            {
                while let Some(nbors) =query.nexties(){
                    count += nbors.len();
                }
            }
            count
        };
        println!("{:?}",clj());
        b.iter(clj);
    }
    #[cfg(feature="bench")]
    #[bench]
    fn rq_neighquery2(b: &mut Bencher){
        let mut grid = Grid::new();
        for i in (0..1000000){
            for j in (0..10000){
                if i % 100 != 0 || j%100 !=0 {
                    continue
                }
                let val = grid.insert(ToyPos::new(i,j));
            }
        }
        let region = Region::square(0,0,0);
        let mut query = grid.neighbor_query(&region);

        let mut clj =|| {
            let mut query = grid.neighbor_query(&region);
            let mut count =0;
            {
                while let Some(nbors) =query.nexties(){
                    count += nbors.len();
                }
            }
            count
        };
        println!("{:?}",clj());
        b.iter(clj);
    }

    #[cfg(feature="bench")]
    #[bench]
    fn rq_neighquery3(b: &mut Bencher){
        let mut grid = Grid::new();
        for i in (0..100){
            for j in (0..100){
                let val = grid.insert(ToyPos::new(i,j));
            }
        }
        println!("done inserting");
        let region = Region::square(0,0,0);
        let mut query = grid.neighbor_query(&region);

        let mut clj =|| {
            let mut query = grid.neighbor_query(&region);
            let mut count =0;
            {
                while let Some(nbors) =query.nexties(){
//                    count += nbors.len();
                }
            }
            count
        };
        println!("{:?}",clj());
        b.iter(clj);
    }

    #[cfg(feature="bench")]
    #[bench]
    fn rq_get_buckets(b: &mut Bencher){
        let mut grid = Grid::new();
        for i in (0..1000){
            for j in (0..1000){
                grid.insert(ToyPos::new(i,j));
            }
        }

        let mut clj =|| {
            test::black_box(grid.occupied_buckets());
        };
        println!("{:?}",clj());
        b.iter(clj);
    }
    
    #[test]
    fn test_rq2(){
        let mut grid = Grid::new();
        for i in (0..100){
            for j in (0..100){
                let val = grid.insert(ToyPos::new(i,j));
            }
        }
        let region = Region { x: 0, y: 5, width: 2, height: 2 };
        
//        let vec: Vec<&ToyPos> = grid.range_query(&region).collect();
//        println!("{:?}",vec);
    }
    #[test]
    fn test_rq(){
        let mut grid = Grid::new();
        for i in (0..1000){
            for j in (0..1000){
                let val = grid.insert(ToyPos::new(i,j));
            }
        }
        let reg= Region { x: 0, y: 2, width: 2, height: 2 };
        for p in grid.range_query(&Region::square(5,5,5)){
            println!("{:?}",p);
        }
                         
        
    }

}

