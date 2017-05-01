use rect::{HasPos,Region};
use std::mem;
use std::slice;
use ref_slice::ref_slice;
use self::Kind::{Branch,Bucket};
use std::vec::IntoIter;
#[derive(Debug)]
pub struct Tree<P>{
    pub region: Region,
    pub kind: Kind<P>,
}

#[derive(Debug)]
pub enum Kind<P> {
    Bucket {
        points: Vec<P>,
    },
    Branch {
        subregions: Vec<Tree<P>>,
        count: usize
    }
}




impl <P: HasPos> Tree<P> {
    pub fn new(region: Region) -> Tree<P> {
        Tree{
            kind: Branch {
                subregions: region
                    .split()
                    .into_iter()
                    .map(|r| Tree { region: r, kind: Bucket { points: vec![] } })
                    .collect(),
                count: 0,
            },
            region: region
        }
    }

    
    pub fn insert(&mut self,point: P) -> bool{
        if !self.region.contains(&point) {return false}
        match self.kind {
            Bucket { ref mut points } => {
                if points.len() != 4 {
                    points.push(point);
                    return true
		}
	    },
	    Branch { ref mut subregions, ref mut count } => {
	    	match subregions.iter_mut().find(|r| r.region.contains(&point)){
		    Some(ref mut subregion) => {*count +=1; return subregion.insert(point)},
		    None => return false
		}
            }
        }
        split_and_insert(self,point);
        true
    }
    pub fn range_query<'t>(&'t self, query: &'t Region) -> RangeQuery<'t,P> {
        RangeQuery {
            query: query,
            points: (&[]).iter(),
            stack: vec![ref_slice(self).iter()],
        }
    }


}
fn split_and_insert<P: HasPos>(bucket: &mut Tree<P>,point: P){
    let old_points;
    match bucket.kind {
	Bucket { ref mut points } => {
	    old_points = mem::replace(points,vec![]);
	},
	Branch { .. } => unreachable!()
    }
    *bucket = Tree::new(bucket.region.clone());
    for old_point in old_points.into_iter() {
        bucket.insert(old_point);
    }
    bucket.insert(point);
}

pub struct RangeQuery<'t,P: 't> {
    pub query: &'t Region,
    pub points: slice::Iter<'t, P>,
    pub stack: Vec<slice::Iter<'t, Tree<P>>>,
}

impl<'t,P: HasPos> Iterator for RangeQuery<'t,P> {
    type Item = &'t P;
    fn next(&mut self) -> Option<&'t P> {
	'outer: loop {
	    for p in &mut self.points {
		if self.query.contains(p){
		    return Some(p)
		}
	    }
	    'region_search: loop {
		let mut children_iter = match self.stack.pop() {
		    Some(x) => x,
		    None => return None,
		};
		'children: loop {
		    match children_iter.next() {
			None => continue 'region_search,
			Some(value) => {
			    if value.region.overlaps(self.query) {
				self.stack.push(children_iter);
				match value.kind{
				    Bucket { ref points, .. } => {
					self.points = points.iter();
					continue 'outer;
				    }
				    Branch { ref subregions, .. } => children_iter = subregions.iter()
				}
			    }
			}
		    }
		}
	    }
	}
    }
}






