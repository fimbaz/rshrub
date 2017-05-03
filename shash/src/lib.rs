#![feature(test)]
extern crate fnv;
extern crate ref_slice;
mod rect;
mod grid;


#[cfg(test)]
mod tests {
    extern crate test;
    use fnv::FnvHashMap;
    use self::test::Bencher;

    use rect::{BucketPos,Pos};
    #[test]
    fn it_works() {
        let mut map = FnvHashMap::default();
        map.insert(1,"one");
        map.insert(2,"one");
        for v in map.iter(){
            println!("{:?}",v);
        }
        
    }
    #[cfg(feature="bench")]
    #[bench]
    fn bench_ins(b: &mut Bencher){
        let mut map = FnvHashMap::default();
        for i in (1..1000){
            for j in (1..1000){
                let val = map.entry(BucketPos::new(i,j)).or_insert(vec![]);
                val.push(Pos::new(i,j));
            }
        }
        b.iter(|| {
            map.iter().any(|x| {test::black_box(x); false});
        });
    }
}
