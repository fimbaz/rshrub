use std::iter::{FromIterator};
use ntree::Region as NTRegion;
pub use ntree::{NTree,RangeQuery};
use ntree::NTreeVariant::{Branch,Bucket};
use ntree::NTreeVariant;
use water::Tile;
use std::slice;                
pub struct QTree<P: HasPos>{
    pub tree: NTree<Region,P>
}

impl<'a,'b> QTree<Tile> {
    fn neighbor_query(&'a mut self) -> NeighborQuery<'a,'b> {
            NeighborQuery {
                range: self.tree.region.clone(),
                pos: Pos {x: 0, y:0},
                neighborhood: Vec::new(),
                rangequery: None
            }
    }

}
pub fn inner_delete<P: HasPos + PartialEq>(mut point: P,parent_branch:&mut Vec<NTree<Region,P>>) -> bool {
    match parent_branch.iter_mut().find(|b| b.region.contains(& point)) {
        Some(ref mut tree) => {
            let is_empty = match tree.kind {
                Bucket { ref mut points, ref bucket_limit } =>{
                    points.retain(|p| *p != point);
                    if points.len() == 0 {
                        true
                    }else{
                        false
                    }
                },
                Branch { ref mut subregions } => {
                    inner_delete(point,subregions)
                }
            };
        },
        None => return false
    }

    return true
}
    
    
    
impl <P: HasPos + PartialEq> QTree<P> {
    pub fn new(region: Region, size: u8) -> QTree<P> {
        return QTree {tree:NTree::new(region,size)};
    }
    pub fn insert(&mut self,point: P) -> bool {
        self.tree.insert(point)
    }
    pub fn range_query<'t,'q>(&'t mut self,query:&'q mut Region) -> RangeQuery<'t, 'q,Region,P>{
        return self.tree.range_query(query);
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

    pub fn delete(&mut self,point: P) -> bool{
        if !self.tree.region.contains(& point)
        { return false }
        match self.tree.kind {
            Bucket {..} => unreachable!() ,
                Branch { ref mut subregions } => {
                    inner_delete(point,subregions);
                }
        }
        return true
    }

}

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
#[derive(Clone, Debug, PartialEq)]
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

impl <T: HasPos> NTRegion<T> for Region {
    fn contains(&self, treepos: & T) -> bool {
        let p = treepos.get_pos();
        self.x <= p.x && self.y <= p.y && (self.x + self.width) > p.x && (self.y + self.height) > p.y
    }

    fn split(&mut self) -> Vec<Region> {
        let halfwidth = self.width / 2;
        let halfheight = self.height / 2;
        vec![
            Region {
                x: self.x,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },

            Region {
                x: self.x,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight
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
                width: halfwidth,
                height: halfheight
            }
        ]
    }

    fn overlaps(& self, other: & Region) -> bool {
        let underlaps = |own:&Region,other: &Region|{
            other.contains(& Pos { x: own.x, y: own.y })
                || other.contains(& Pos { x: own.x + own.width, y: own.y })
                || other.contains(& Pos { x: own.x, y: own.y + own.height })
                || other.contains(& Pos { x: own.x + own.width, y: own.y + own.height })
        };
        return underlaps(self,other) || underlaps(other,self);
    }

}



/*
impl <'a,'b,T: HasPos,T2: HasPos> NeighborQuery<'a,'b,T,T2> {
    pub fn new(tree: &QTree<T>) -> NeighborQuery<T,T2>{
        return NeighborQuery { tree: tree, pos: Pos {x:0,y:0},neighborhood: None};
    }
}
impl <'a,'b,T: HasPos,T2: HasPos> NeighborQuery<'a,'b,T,T2> {
    fn next(&'a mut self) -> Vec<&'a T>{
        let ref region = Region::square(self.pos.x-1,self.pos.y-1,3);
        return self.tree.tree.range_query(region).collect()
    }
}
*/

pub struct NeighborQuery<'a,'b> {
    pos: Pos,
    rangequery: Option<RangeQuery<'a,'b,Region,Tile>>,
    neighborhood: Vec<&'a mut Tile>,
    range: Region,
        
}
/*
impl <'a,'b> NeighborQuery<'a,'b> {

    fn next(&'a mut self) -> &Vec<&mut Tile> {
        let region = Region::square(self.pos.x-1,self.pos.y-1,3);

        //This is meant to drive our main loop. 
        //Neighborquery shall hold, on each iteration, a vector of mutable references to the active neighbors of the element at Pos ..
        //(we won't need a Neighborhood enum ultimately, because Tiles hold their own position.
        //the 'calling' cell can infer which neighbors are active and which are STP air/ground
        //so long as the list of active neighbors is exhaustive.  We can do the confusing graph stuff in here, then make it pretty in another func.. just satify the constraint.
        //The slow way is to do a RangeQuery for each iteration.  The fast way is to thumb through the stack, and keep local refs around between iters-- hence the short 'b lifetime of those innermost refs])
        }
}


*/
