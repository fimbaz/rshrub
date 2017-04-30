#![feature(test)]

extern crate core;
extern crate ntree;
extern crate rustty;
extern crate rand;
mod water;
mod tree;
mod display;
mod bench;
#[cfg(test)]
mod tests {
    #[cfg(feature = "bench")]
    use self::test::Bencher;
    use tree::{Pos,Region,QTree};
    use ntree::Region as NTRegion;
    use water::{Board,Tile,Material};
    use rustty::ui::core::{Frame};
    use display::{WorldView};
    use rand::{random,XorShiftRng,Rng,SeedableRng,StdRng};
    use std::process;
    #[cfg(test)]
    #[cfg(feature = "bench")]
    extern crate rand;
    #[cfg(feature = "bench")]
    extern crate test;
    #[cfg(feature = "bench")]
    use self::test::Bencher;
    #[cfg(feature = "bench")]

    use std::mem::replace;
    use std::mem::size_of;
    
    fn test_full_region(region: &Region){
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..(region.x+region.width+5){
            for j in 0..(region.y+region.height+5){
	    	tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
            }
        }
	let rq = tree.tree.range_query(region.clone());
        assert_eq!(rq.fold(0,|i,x| {i+1}),(region.width+1)*(region.height+1));
    }
//    #[test]
//    fn test_rect_sanity(){
//        let A = Region{x:1,y:0,width:1,height:3};
//        let B = Region{x:0,y:1,width:3,height:1};
//        assert_eq!(true,underlaps(&A,&B));
    //    }

    fn test_sane_rect(){
        
    }
    #[test]
    fn test_delete(){
        let region = Region { x: 0, y: 0, width: 99, height: 99 };
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..100{
            for j in 0..100{
	    	tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
            }
        }

        
        for i in 0..47{
            for j in 0..47{
                tree.delete(Tile{pos: Pos {x: i, y: j},material: Material::Ground()});
            }
        }

	{let rq = tree.tree.range_query(region.clone());
         assert_eq!(rq.fold(0,|i,x| {i+1}),100*100-(47*47));}
        
    }
    
    #[test]
    fn test_rect_sanity_better(){
        let contains2 = |r: &Region,p: &Pos| r.x <= p.x && r.y <= p.y && r.x+r.width >= p.x && r.y + r.height >= p.y;
        let overlaps = |own: &Region, other: &Region| {
            (own.x < other.x + other.width) &&
                (own.x+own.width > other.x)   &&
                (own.y < other.y + other.height) &&
                (own.y + own.height > other.y)
        };

        let A=Region{x:1,y:0,width:2,height:4};
        let B=Region{x:0,y:1,width:4,height:2};
        let P=Pos{x:3,y:2};
        if A.contains(&P) && B.contains(&P){
            println!("Point is where it should be...");
        }
        assert_eq!(overlaps(&A,&B),true);

    }
/*    #[test]
    fn test_gupta_rect_sanity(){
        // If one rectangle is on left side of other
        if l1.x > r2.x || l2.x > r1.x{
            return false;
        }
        // If one rectangle is above other
        if l1.y < r2.y || l2.y < r1.y{
            return false;
        }
            
        
        return true;
    }
*/
    #[test]
    fn test_neighbor_query(){
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..(100){
            for j in 0..(100){
	    	tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
            }
        }
        let mut querymut = tree.tree.neighbor_query_mut();
        let mut tree = querymut.nexties(Region::square(0,0,0));
        {querymut.nexties(Region::square(0,0,0));}
    }

    #[test]
    fn it_works() {
        let mut board = Board::new(30);        
        for i in 0..1000{
            for j in 0..1000{
                board.tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
		
            }
        }
        let badregion = Region { x: 3, y: 1, width: 4, height: 1 };
        let mut count=0;
        for point in board.tree.tree.range_query(badregion){
//            println!("{:?}",point);
            count = count + 1 ;
        }
        assert_eq!(10,count);

        let initial_result =board.tree.tree.range_query(Region{x:0,y:0,width:20,height:20}).fold(0,|i,x| {i+1});
        let seed: &[_] = &[3, 5, 1, 5];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for i in 0..10{
            for j in 0..10{
                let x = rng.gen::<u16>() %250 ;
                let y = rng.gen::<u16>() %250;
                let width = rng.gen::<u16>() %250;
                let height = rng.gen::<u16>() %250;
                let region =Region{x:x,y:y,width:width,height:height};
                let mut points = board.tree.tree.range_query(region);
                let mut  count = 0;
                for point in points{
                    assert_eq!(true,region.contains(point));
                    count = count+1;
                }
                if !(width*height == count){
                    assert_eq!((width+1)*(height+1),count);
                }
                
            }
        }
         

        
        
    }

    #[test]
    fn smallish_correctness_test() {
        let mut board = Board::new(30);        
        for i in 0..20{
            for j in 0..20{
                board.tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
		
            }
        }
        let badregion = Region { x: 3, y: 1, width: 4, height: 1 };
        let mut count=0;
        for point in board.tree.tree.range_query(badregion){
//            println!("{:?}",point);
            count = count + 1 ;
        }
        assert_eq!(10,count);

        let initial_result =board.tree.tree.range_query(Region{x:0,y:0,width:20,height:20}).fold(0,|i,x| {i+1});
        let seed: &[_] = &[3, 5, 1, 1];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for i in 0..10{
            for j in 0..10{
                let x = rng.gen::<u16>() %10 ;
                let y = rng.gen::<u16>() %10;
                let width = rng.gen::<u16>() %5;
                let height = rng.gen::<u16>() %5;
                let region =Region{x:x,y:y,width:width,height:height};
//                println!("{:?}",region);
                let mut points = board.tree.tree.range_query(region);
                let mut  count = 0;
                for point in points{
                    assert_eq!(true,region.contains(point));
                    count = count+1;
                }
                if !(width*height == count){
                    assert_eq!((width+1)*(height+1),count);
                }
                
            }
        }
         

        
        
    }
    
    #[cfg(feature = "bench")]
    fn create_full_region(region: &Region) -> QTree<Tile>{
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..(region.x+region.width+5){
            for j in 0..(region.y+region.height+5){
	    	tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
            }
        }
	{let rq = tree.tree.range_query(region.clone());
         assert_eq!(rq.fold(0,|i,x| {i+1}),(region.width+1)*(region.height+1))};
        return tree;
    }
    #[cfg(feature = "bench")]
    fn create_holey_region(region: &Region) -> QTree<Tile>{
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..(region.x+region.width+5){
            for j in 0..(region.y+region.height+5){
                if i % 10 != 0 || j %10 == 1{
	    	    tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
                }
            }
        }
	{let rq = tree.tree.range_query(region.clone());};
        return tree;
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn test_search_region2(b: &mut Bencher){
        let mut tree = create_full_region(&Region {x:0,y:0,width:100,height:100});
        b.iter(|| {
            for p in tree.range_query(Region {x:0,y:0,width:30,height:30}) { test::black_box(p); }
        })
    }
    #[cfg(feature = "bench")]
    fn raster_tree(b: &mut Bencher,tree: &mut QTree<Tile>,raster_size: u16){
        b.iter(|| {
            for i in 0..raster_size {
                for j in 0..raster_size {
                    for p in tree.range_query(Region {x:i,y:j,width:2,height:2}) { test::black_box(p); }
                }
            }

        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn raster_tree_small(b: &mut Bencher){
        let mut tree = create_full_region(&Region {x:0,y:0,width:100,height:100});
        raster_tree(b,&mut tree,10);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn raster_tree_medium(b: &mut Bencher){
        let mut tree = create_full_region(&Region {x:0,y:0,width:50,height:50});        
        raster_tree(b,&mut tree,30);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn raster_tree_huge(b: &mut Bencher){
        let mut tree = create_full_region(&Region {x:0,y:0,width:1000,height:1000});        
        raster_tree(b,&mut tree,1000);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn big_world_one_query(b: &mut Bencher){
        let mut tree = create_full_region(&Region {x:0,y:0,width:1000,height:1000});        
        raster_tree(b,&mut tree,1);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn big_holey_world_one_query(b: &mut Bencher){
        let mut tree = create_holey_region(&Region {x:0,y:0,width:1000,height:1000});        
        raster_tree(b,&mut tree,1);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn show_node_count(b: &mut Bencher){
        let mut tree = QTree::new(Region::square(0,0,16384),4);
        for i in 0..1000{
            for j in 0..1000{
	    	tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
            }
        }
        let mut rquery = tree.tree.range_query(Region{x:20,y:20,width:0,height:0});

        rquery.next();
//        println!("{:?}",rquery.next());
    }
    
}

