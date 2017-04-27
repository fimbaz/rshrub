//#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(feature = "bench", feature(test))]

//! A generic, n-dimensional quadtree for fast neighbor lookups on multiple axes.

extern crate ref_slice;
use std::{mem, slice};
use self::NTreeVariant::{Branch, Bucket};
use std::vec::IntoIter;
#[cfg(test)]
mod test;
use std::fmt;
/// The required interface for Regions in this n-tree.
///
/// Regions must be able to split themselves, tell if they overlap
/// other regions, and tell if a point is contained within the region.
pub trait Region<P>: Clone + fmt::Debug {
    /// Does this region contain this point?
    fn contains(&self, & P) -> bool;
    fn contains_region(&self,r: &Self) -> bool;

    /// Split this region, returning a Vec of sub-regions.
    ///
    /// Invariants:
    ///   - The sub-regions must NOT overlap.
    ///   - All points in self must be contained within one and only one sub-region.
    fn split(&self) -> Vec<Self>;

    /// Does this region overlap with this other region?
    fn overlaps(& self, other: &Self) -> bool;
}

/// A quadtree-like structure, but for arbitrary arity.
///
/// Regions can split themselves into arbitrary numbers of splits,
/// allowing this structure to be used to index data by any number
/// of attributes and quickly query for data that falls within a
/// specific range.
#[derive(Debug)]
pub struct NTree<R, P> {
    pub region: R,
    pub kind: NTreeVariant<R, P>,

}
#[derive(Debug)]
pub enum NTreeVariant<R, P> {
    /// A leaf of the tree, which contains points.
    Bucket {
        points: Vec<P>,
        bucket_limit: u8

    },
    /// An interior node of the tree, which contains n subtrees.
    Branch {
        subregions: Vec<NTree<R, P>>,
        count: usize
    }
}

impl<P, R: Region<P>> NTree<R, P> {
    /// Create a new n-tree which contains points within
    /// the region and whose buckets are limited to the passed-in size.
    ///
    /// The number of regions returned by region.split() dictates
    /// the arity of the tree.
    pub fn new(region: R, size: u8) -> NTree<R, P> {
        NTree {
            kind: Branch {
                subregions: region
                    .split()
                    .into_iter()
                    .map(|r| NTree {
                        region: r,
                        kind: Bucket { points: vec![], bucket_limit: size }
                    })
                    .collect(),
                count: 0,
            },
            region: region
        }
    }
    /// Insert a point into the n-tree, returns true if the point
    /// is within the n-tree and was inserted and false if not.
    pub fn insert(&mut self,  point: P) -> bool {
        if !self.region.contains(& point) { return false }
        
        match self.kind {
            Bucket { ref mut points, ref bucket_limit } => {
                if points.len() as u8 != *bucket_limit {
                    points.push(point);
                    return true
                }
            },
            Branch { ref mut subregions, ref mut count } => {
                match subregions.iter_mut().find(|r| r.contains(&point)) {
                    Some(ref mut subregion) => {*count+=1; return subregion.insert(point)},
                    None => return false
                }
            }
        };

        // Bucket is full
        split_and_insert(self, point);
        true
    }
    pub fn range_query<'t>(&'t self, query: R) -> RangeQuery<'t, R, P> {
        RangeQuery {
            query: query,
            points: (&[]).iter(),
            stack: vec![ref_slice::ref_slice(self).iter()],
            perf_count: 0
        }
    }
    /// Get all the points which within the queried region.
    ///
    /// Finds all points which are located in regions overlapping
    /// the passed in region, then filters out all points which
    /// are not strictly within the region.
    pub fn range_query_mut<'t>(&'t mut self, query: R) -> RangeQueryMut<'t, R, P> {
        RangeQueryMut {
            stack: vec![ref_slice::mut_ref_slice(self).iter_mut()], //1 len slice w/ head of tree
            query: query,
            points: (&mut []).iter_mut(), //empty slice


        }
    }
    /// Is the point contained in the n-tree?
    pub fn contains(&self, point: & P) -> bool {
        self.region.contains(point)
    }

    /// Get all the points nearby a specified point.
    ///
    /// This will return no more than bucket_limit points.
    pub fn nearby<'a>(&'a self, point: & P) -> Option<&'a[P]> {
        if self.region.contains(point) {
            match self.kind {
                Bucket { ref points, .. } => Some(points.as_slice()),
                Branch { ref subregions, .. } => {
                    subregions
                        .iter()
                        .find(|r| r.contains(point))
                        .and_then(|r| r.nearby(point))
                }
            }
        } else {
            None
        }
    }
    pub fn nearby_mut<'a>(&'a mut self, point: &mut P) -> Option<&'a mut[P]> {
        if self.region.contains(point) {
            match self.kind {
                Bucket { ref mut points, .. } => Some(&mut points[..]),
                Branch { ref mut subregions,.. } => {
                    subregions
                        .iter_mut()
                        .find(|r| r.contains(point))
                        .and_then(|r| r.nearby_mut(point))
                }
            }
        } else {
            None
        }
    }

    pub fn neighbor_query_mut<'a>(&'a mut self) -> NeighborQueryMut<'a,R,P>{
        return NeighborQueryMut { stack: vec![IndexedTree::new(self)],points: (&mut []).iter_mut() };
    }

}

fn split_and_insert<P, R: Region<P>>(bucket: &mut NTree<R, P>, point: P) {
    let old_points;
    let old_bucket_limit;

    match bucket.kind {
        // Get the old region, points, and bucket limit.
        Bucket { ref mut points, bucket_limit } => {
            old_points = mem::replace(points, vec![]);
            old_bucket_limit = bucket_limit;
        },
        Branch { .. } => unreachable!()
    }

    // Replace the bucket with a split branch.
    *bucket = NTree::new(bucket.region.clone(), old_bucket_limit);

    // Insert all the old points into the right place.
    for old_point in old_points.into_iter() {
        bucket.insert(old_point);
    }

    // Finally, insert the new point.
    bucket.insert(point);
}


pub struct RangeQuery<'t, R: 't, P: 't> {
    pub query: R,
    pub points: slice::Iter<'t, P>,
    pub stack: Vec<slice::Iter<'t, NTree<R, P>>>,
    pub perf_count: usize
        
}


impl<'t, R: Region<P>, P> Iterator for RangeQuery<'t, R, P> {
    type Item = &'t P;
        fn next(&mut self) -> Option<&'t P> {
        'outer: loop {
            // try to find the next point in the region we're
            // currently examining.
            for p in &mut self.points {
                if self.query.contains(p) {
                    return Some(p)
                }
            }

            // no relevant points, so lets find a new region.

            'region_search: loop {
                let mut children_iter = match self.stack.pop() {
                    Some(x) => x,

                    // no more regions, so we're over.
                    None => return None,
                };

                'children: loop {
                    // look at the next item in the current sequence
                    // of children.
                    match children_iter.next() {
                        // this region is empty, next region!
                        None => continue 'region_search,

                        Some(value) => {
                            if value.region.overlaps(&self.query) {
                                // we always need to save this state, either we
                                // recur into a new region, or we break out and
                                // handle the points; either way, this is the
                                // last we touch `children_iter` for a little
                                // while.
                                self.stack.push(children_iter);

                                match value.kind {
                                    Bucket { ref points, .. } => {
                                        // found something with points
                                        let tmp_points = points.iter();
                                        for point in tmp_points{
                                            assert_eq!(true,value.region.contains(&point));
                                        }
                                        self.points = points.iter();
                                        self.perf_count = self.perf_count+1;
                                        continue 'outer;
                                    }
                                    // step down into nested regions.
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

pub struct IndexedTree<'t,R: 't,P: 't>{
    tree: &'t mut NTree<R,P>,
    index: u8,
}
impl <'t,R: Region<P>,P> IndexedTree<'t,R,P>{
    pub fn new(tree: &'t mut NTree<R,P> ) -> IndexedTree<'t,R,P>{
        IndexedTree{tree:tree,index:0}
    }
                 
}

pub struct NeighborQueryMut<'t,R: 't,P: 't>{
    pub stack: Vec<IndexedTree<'t,R,P>>,
    pub points: slice::IterMut<'t, P>,    
        
}
impl <'t, R: Region<P>,P>  NeighborQueryMut<'t, R,P>{
    pub fn nexties<'b>(&'b mut self,region: R) -> Option<(&'b NTree<R,P>,R)>{
        //return a unit rectangle that contains an active point, and a reference to the smallest subtree containing that point.
        Some((self.stack.get(0).unwrap().tree,region))
    }
}
/*
impl<'t, R: Region<P>, P> Iterator for NeighborQueryMut<'t, R, P> {
    type Item = &'t P;
        fn next(&mut self) -> Option<&'t P> {
        'outer: loop {
            // try to find the next point in the region we're
            // currently examining.
            for p in &mut self.points {
                if self.query.contains(p) {
                    return Some(p)
                }
            }

            // no relevant points, so lets find a new region.

            'region_search: loop {
                let mut children_iter = match self.stack.pop() {
                    Some(x) => x,

                    // no more regions, so we're over.
                    None => return None,
                };

                'children: loop {
                    // look at the next item in the current sequence
                    // of children.
                    match children_iter.next() {
                        // this region is empty, next region!
                        None => continue 'region_search,

                        Some(value) => {
                            if value.region.overlaps(&self.query) {
                                // we always need to save this state, either we
                                // recur into a new region, or we break out and
                                // handle the points; either way, this is the
                                // last we touch `children_iter` for a little
                                // while.
                                self.stack.push(children_iter);

                                match value.kind {
                                    Bucket { ref points, .. } => {
                                        // found something with points
                                        let tmp_points = points.iter();
                                        for point in tmp_points{
                                            assert_eq!(true,value.region.contains(&point));
                                        }
                                        self.points = points.iter();
                                        self.perf_count = self.perf_count+1;
                                        continue 'outer;
                                    }
                                    // step down into nested regions.
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
*/
                                 
/*
pub struct NeighborQueryMut<'t,R: 't,P: 't>{
    pub query:R,
    pub points: slice::IterMut<'t,P>,
    pub stack: Vec<slice::IterMut<'t, NTree<R, P>>>,

}
//input: a range query.
//stack: a vector of iterators, each containing the members of a branch or bucket
//if we pop an iterator off the stack, we'd like to return it

pub struct SubQueryMut<'t,R: 't,P: 't>{
    pub rangequery: RangeQueryMut<'t,R,P>,
    pub subquery: RangeQueryMut<'t,R,P>,
    pub tmp_stack: Vec<slice::IterMut<'t,NTree<R,P>>>
    //pub saved_stack: Vec<slice::IterMut<'t,NTree<R,P>>>,
}

impl <'t,R: Region<P> + Copy,P> RangeQueryMut<'t,R,P>  {
    pub fn subquery_mut(mut self,region: R) -> SubQueryMut<'t,R,P> {
        let mut tmp_slice: Option<([NTree<R,P>])> = None;
        let mut tmp_stack: Vec<slice::IterMut<'t,NTree<R,P>>> = Vec::new();
        let mut good_tree: slice::IterMut<'t,NTree<R,P>> = (&mut []).iter_mut();
//        let mut good_slice: Option<&mut NTree<R,P>> = None;
        {
            let mut found = false;
            'outer: loop {
                if let Some(subtrees_it) = self.stack.pop(){
                    let subtrees = subtrees_it.into_slice();
                    'inner : for subtree in subtrees.iter_mut() {
                        if  subtree.region.contains_region(&region){
                            found=true;
                            break ;
                        }
                        if found {
                            tmp_slice = Some(subtrees);
                        }else{
                            tmp_stack.push(subtrees.iter_mut());
                        }
                    }


                }
            
            }
            
            return SubQueryMut { subquery: RangeQueryMut{query: region,
                                                         points: (&mut[]).iter_mut(),
                                                         stack: vec![ref_slice::mut_ref_slice(&mut tmp_slice.unwrap()).iter_mut()]},rangequery:self,tmp_stack: tmp_stack    };
        }
    }
}
/*
/// An iterator over the points within a region.


impl<'t, R: Region<P>, P> Iterator for NeighborQueryMut<'t, R, P> {
    type Item = &'t mut P;
    fn next(&mut self) -> Option<&'t mut P> {
        'outer: loop {
            for p in &mut self.points {
                if self.query.contains(p) {
                    return Some(p)
                }
            }
            'region_search: loop {
                let mut children_iter = match self.stack.pop() {
                    Some(x) => x,
                    None => return None,
                };
                'children: loop {
                    for value in children_iter.next() {
                        if value.region.overlaps(&self.query) {
                                    self.stack.push(children_iter);
                                    match value.kind {
                                        Bucket { ref mut points, .. } => {
                                            self.points = points.iter_mut();
                                            continue 'outer;
                                        }
                                        Branch { subregions } => children_iter = subregions.iter_mut()
                                    }
                        }
                    }
                    continue 'region_search
                }
            }

        }
    }
}
*/
*/
// This iterates over the leaves of the tree from left-to-right by
// maintaining (a) the sequence of points at the current level
// (possibly empty), and (b) stack of iterators over the remaining
// children of the parents of the current point.
pub struct RangeQueryMut<'t, R: 't, P: 't> {
    pub query: R,
    pub points: slice::IterMut<'t, P>,
    pub stack: Vec<slice::IterMut<'t, NTree<R, P>>>,
    
}
/*
impl  <'t,R,P> Drop for SubQueryMut<'t,R,P> {
    fn drop (&mut self){
        return;
//        for item in self.saved_stack{

//            self.rangequery.stack.push(item.as_slice().iter_mut());
        }
}

*/
//        let mut subquery = RangeQuery { 
//            stack: vec![ref_slice::mut_ref_slice(self).iter_mut()], //1 len slice w/ head of tree
//            query: region,
//            points: (&mut []).iter_mut(), //empty slice
//        }
        
//        let old_query = mem::replace(self,subquery);
//        mem::replace(subquery.stack,old_query.stack)
//        self.query = query;


impl<'t, R: Region<P>, P> Iterator for RangeQueryMut<'t, R, P> {
    type Item = &'t mut P;

    fn next(&mut self) -> Option<&'t mut P> {
        'outer: loop {
            for p in &mut self.points {
                if self.query.contains(p) {
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
                            if value.region.overlaps(&self.query) {
                                self.stack.push(children_iter);
                                match value.kind {
                                    Bucket { ref mut points, .. } => {
                                        self.points = points.iter_mut();
                                        continue 'outer;
                                    }
                                    Branch { ref  mut subregions, .. } => children_iter = subregions.iter_mut()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}




