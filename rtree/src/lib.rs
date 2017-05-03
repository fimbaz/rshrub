#![feature(test)]
extern crate ref_slice;
mod rect;
mod tree;
#[cfg(test)]
mod tests {
    extern crate test;
    use rect::{Pos,Region,MAX_RECT_SIZE};
    use tree::{Tree,RangeQuery};
    #[cfg(feature = "bench")]
    use self::test::Bencher;

    #[test]
    fn create_rect(){
        let sq = Region::square(0,0,10);
        let rect = Region::rectangle(0,0,32,32);
        test::black_box(sq);
        test::black_box(rect);
    }
    #[test]
    fn test_overlaps(){
        let a=Region{x:1,y:0,width:2,height:4};
        let b=Region{x:0,y:1,width:4,height:2};
        assert_eq!(a.overlaps(&b),true);
    }
    #[test]
    fn test_contains(){
        let a=Region{x:1,y:0,width:2,height:4};
        let b=Region{x:0,y:1,width:4,height:2};
        let p=Pos{x:3,y:2};
        
    }

    fn create_full_region(i: usize,j: usize) -> Tree<Pos>{
        let a=Region{x:0,y:0,width:16384,height:16384};
        let mut tree = Tree::new(a);
        for x in (0..i){
            for y in (0..j){
                tree.insert(Pos{x:x,y:y});
            }
        }
        return tree;
    }

    #[test]
    fn test_create_region(){
        let tree =create_full_region(100,100);
        assert_eq!(10000,tree.range_query(&tree.region).fold(0,|i,x|i+1));
    }
    #[test]
    fn test_create(){
        let a=Region{x:0,y:0,width:16384,height:16384};
        let mut tree: Tree<Pos> = create_full_region(100,100);
        assert_eq!(tree.range_query(&a).fold(0,|i,x|i+1),10000);

        
    }

    #[cfg(feature="bench")]
    #[bench]
    fn test_rq_10000(b: &mut Bencher){
        let a=Region{x:0,y:0,width:MAX_RECT_SIZE,height:MAX_RECT_SIZE};
        let mut tree: Tree<Pos> = Tree::new(a);
        for i in 0..100{
            for j in 0..100{
	        tree.insert(Pos{x:i,y:j});
            }
        }
        let mut clj = || {
            let mut count=0;
            for pos in tree.range_query(&Region{x:0,y:0,width:100,height:100}){
                count = count+1;
                test::black_box(pos);
            }
            count
        };

        println!("{:?}",clj());
        b.iter(clj);
        
    }

    #[cfg(feature="bench")]
    #[bench]
    fn create_region_10000(b: &mut Bencher){
        b.iter(||
               for i in 0..100{
                   let a=Region{x:0,y:0,width:MAX_RECT_SIZE,height:MAX_RECT_SIZE};
                   let mut tree: Tree<Pos> = Tree::new(a);
                   for j in 0..100{
	    	       tree.insert(Pos{x:i,y:j});
                   }
               });
        
    }

    #[test]
    fn it_works() {
    }
}
