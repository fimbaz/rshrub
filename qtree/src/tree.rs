use std::iter::{FromIterator};
use ntree::Region as NTRegion;
pub use ntree::{NTree,RangeQuery,RangeQueryMut};
use ntree::NTreeVariant::{Branch,Bucket};
use ntree::NTreeVariant;
use water::Tile;
use std::slice;                
pub struct QTree<P: HasPos>{
    pub tree: NTree<Region,P>
}
pub fn inner_delete<P: HasPos + PartialEq>(mut point: P,parent_branch:&mut Vec<NTree<Region,P>>) -> bool {
    match parent_branch.iter_mut().find(|b| b.region.contains(& point)) {
        Some(ref mut tree) => {
            match tree.kind {
                Bucket { ref mut points, ref bucket_limit } =>{
                    let old_len = points.len();
                    points.retain(|p| *p != point);
                    if points.len() != old_len{
                        true
                    }else{
                        false
                    }
                },
                Branch { ref mut subregions , ref mut count } => {
                    let delete_occured = inner_delete(point,subregions);
                    if delete_occured {
                        *count -= 1;
                        true
                    }else {
                        false
                    }
                }
            }
        },
        None => false
    }
}
    





    
impl <P: HasPos + PartialEq> QTree<P> {
    pub fn neighbor_query(& self) -> NeighborQuery<P>{
        NeighborQuery { tree: self,query: self.tree.range_query(Region{x:1, y:1,height:16384,width: 16384})}
    }

    pub fn new(region: Region, size: u8) -> QTree<P> {
        return QTree {tree:NTree::new(region,size)};
    }
    pub fn insert(&mut self,point: P) -> bool {
        self.tree.insert(point)
    }
    pub fn range_query<'t>(&'t self,query:  Region) -> RangeQuery<'t,Region,P>{
        return self.tree.range_query(query);
    }
        pub fn range_query_mut<'t>(&'t mut self,query: Region) -> RangeQueryMut<'t,Region,P>{
        return self.tree.range_query_mut(query);
    }

    pub fn contains(& self,point: & P) -> bool {
        return self.tree.region.contains(point);
    }
    pub fn nearby<'a>(&'a self,point: &mut P) -> Option<&'a[P]> {
        return self.tree.nearby(point);
    }
    pub fn nearby_mut<'a>(&'a mut self,point: &mut P) -> Option<&'a mut[P]> {
        return self.tree.nearby_mut(point);
    }
    pub fn delete(&mut self,mut point: P) -> Option<P>{
        QTree::inner_delete(point,&mut self.tree)
    }
    fn inner_delete(mut point: P,parent:&mut NTree<Region,P>) -> Option<P> {
        if !parent.region.contains(&point) { return None }
        match parent.kind {
            Bucket { ref mut points, ref bucket_limit } => {
                let index = if let Some((index,point)) = points.iter().enumerate().find(|x| *x.1 == point ){
                    Some(index)
                }else{
                    None
                };
                if let Some(i) = index{
                    Some(points.swap_remove(i)) //all this so we can return the element we've removed.
                }else{
                    None
                }
            },            
            Branch { ref mut subregions, ref mut count } => {
                let mut  search_result: Option<&mut NTree<Region,P>> = subregions.iter_mut().find(|b| b.region.contains(&point)); //invariants should let us unwrap this line later.
                let mut is_subregion_empty = false;
                let ret_point_opt: Option<P> =  match search_result {
                    Some(ref mut containing_subregion) => {
                        match  QTree::inner_delete(point,containing_subregion) {
                            Some(ret_point) => {
                                match containing_subregion.kind {
                                    Branch { ref mut  count, .. } => {
                                        *count -= 1;
                                        if *count == 0 {
                                            is_subregion_empty = true;
                                        }
                                    },
                                    _ => {},
                                }
                                Some(ret_point)
                            }
                            None => None,
                        }
                    },
                    None => None,
                };
                if is_subregion_empty{
                    search_result.unwrap().kind = Bucket { points: vec![], bucket_limit: 4}; //no  branches with count == 0 should ever exist outside of this 
                }
                ret_point_opt
            }
        }
    }
}
/*    
    pub fn delete(&mut self,point: P) -> bool{
        if !self.tree.region.contains(& point)
        { return false }
        match self.tree.kind {
            Bucket {..} => unreachable!() ,
                Branch { ref mut subregions, .. } => {
                    inner_delete(point,subregions);
                }
        }
        return true
    }
*/


#[derive(Clone, Copy,Debug,PartialOrd,PartialEq)]
pub struct Pos  {pub x:u16,pub y: u16}
    pub trait HasPos {
        fn get_pos(&self) -> Pos;
    }
impl HasPos for Pos {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.x,y:self.y};
    }
}
#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Region {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}
impl Region {
    pub fn square(x: u16, y:u16, wh: u16) -> Region {
        Region { x: x, y: y, width: wh, height: wh }
    }
    pub fn rectangle(x: u16, y:u16,w: u16,h:u16) -> Region {
        Region { x: x, y: y, width: w, height: h }
    }
}


fn contains2(r:&Region,p:&Pos) -> bool {
    let result = r.x <= p.x && r.y <= p.y && r.x+r.width >= p.x && r.y + r.height >= p.y;
    result
    //    r.x <=  p.x && r.y <=  p.y && (r.x + r.width) >= p.x && (r.y + r.height) >= p.y
}


impl <T: HasPos> NTRegion<T> for Region {
    fn contains_region(&self,other: &Self) -> bool {
        let own = self;
        own.contains(& Pos { x: other.x, y: other.y })
            || own.contains(& Pos { x: other.x + other.width, y: other.y })
            || own.contains(& Pos { x: other.x, y: other.y + other.height })
            || own.contains(& Pos { x: other.x + other.width, y: other.y + other.height })
    }
    fn contains(&self, treepos: & T) -> bool {
        let p = treepos.get_pos();
        self.x <= p.x && self.y <= p.y && (self.x + self.width) >= p.x && (self.y + self.height) >= p.y
    }
    fn split(& self) -> Vec<Region> {
        let halfwidth = self.width / 2 ;
        let halfheight = self.height / 2;
        let halfwidth_remainder = self.width %2;
        let halfheight_remainder = self.height %2;
    
        vec![
            Region {
                x: self.x,
                y: self.y,
                width: halfwidth,
                height: halfheight,
            },

            Region {
                x: self.x,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight,
            },

            Region {
                x: self.x + halfwidth,
                y: self.y,
                width: halfwidth,
                height: halfheight 
            },

            Region {
                x: self.x + halfwidth,
                y: self.y + halfheight,
                width: halfwidth ,
                height: halfheight,
            }
        ]
    }
    fn overlaps(& self, other: &Self) -> bool {
        (self.x <= other.x+other.width) &&
                (self.x+self.width >= other.x)   &&
                (self.y <= other.y + other.height) &&
                (self.y + self.height >= other.y)
    }
}
pub struct NeighborQuery<'t,P:'t + HasPos + PartialEq>{
    pub tree: &'t QTree<P>,
    pub query: RangeQuery<'t,Region,P>,
}

impl <'t,P: HasPos + PartialEq>  Iterator for NeighborQuery<'t,P>{
    type Item = RangeQuery<'t,Region,P>;
     fn next(&mut self) -> Option<RangeQuery<'t,Region,P>>{
         if let Some(point) =  self.query.next(){
             let pos = point.get_pos();
             return Some(self.tree.range_query(Region { x: pos.x.saturating_sub(1),y: pos.y.saturating_sub(1),height:2,width:2}));
         }else{
             None
         }
                                  
        //return a unit rectangle that contains an active point, and a reference to the smallest subtree containing that point.
    }
}

