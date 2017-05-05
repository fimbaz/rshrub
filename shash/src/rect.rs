use std::hash::{Hash,Hasher};

use std::collections::hash_map::{Entry};

pub const MAX_RECT_SIZE: usize = 16384;
pub const RECT_BUCKET_SIZE: usize = 2;
impl Eq for Pos {}
impl Eq for BucketPos {}
#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Region {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}
#[derive(Clone,Copy,Debug, PartialEq)]
pub struct Pos  {pub x:usize,pub y: usize}
    pub trait HasPos {
        fn get_pos(&self) -> Pos;
    }
impl HasPos for Pos {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.x,y:self.y};
    }
}
#[derive(Clone,Copy,Debug)]
pub struct BucketPos(pub Pos);
impl HasPos for BucketPos {
    fn get_pos(&self) -> Pos {
        return self.0;
    }
                                    
}

impl  From<Pos> for BucketPos {
    fn from(pos: Pos) -> BucketPos{
        return BucketPos(pos.clone());
    }
}

impl Hash for BucketPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32((self.0.x/RECT_BUCKET_SIZE) as u32);
        state.write_u32((self.0.y/RECT_BUCKET_SIZE) as u32);
        state.finish();
  }
}

impl PartialEq for BucketPos{
    fn eq(&self, other: &BucketPos) -> bool{
        if self.0.x/RECT_BUCKET_SIZE == other.0.x/RECT_BUCKET_SIZE &&
            self.0.y/RECT_BUCKET_SIZE  == other.0.y/RECT_BUCKET_SIZE{
                true
            }else{
                false
            }
    }

    fn ne(&self, other: &BucketPos) -> bool {
        !self.eq(other)
    }
}
impl Pos {
     pub fn new(x: usize,y:usize) -> Pos{
     	 Pos{x: x,y: y}
     }
}
impl BucketPos {
     pub fn new(x: usize,y:usize) -> BucketPos{
     	 BucketPos(Pos{x: x,y: y})
     }
}

pub struct Iter<'t> {
    region: &'t Region,
    pos: Pos,
}

impl <'t> Iterator for Iter <'t>{
    type Item = Pos;
    fn next(&mut self) -> Option<Pos>{
        if !self.region.contains(&self.pos){ return None;}
        let mut newpos = self.pos;
        newpos.x+=RECT_BUCKET_SIZE;
        if !self.region.contains(&newpos){ newpos = Pos::new(self.region.x,newpos.y+RECT_BUCKET_SIZE); }
        let oldpos = self.pos;
        self.pos = newpos;
        return Some(oldpos);
        
    }    
}

impl Region{
    pub fn square(x: usize, y:usize, wh: usize) -> Region {
        Region { x: x, y: y, width: wh, height: wh }
    }
    pub fn rectangle(x: usize, y:usize,w: usize,h:usize) -> Region {
        Region { x: x, y: y, width: w, height: h }
    }
    pub fn overlaps(& self, other: &Self) -> bool {
        (self.x <= other.x+other.width) &&
            (self.x+self.width >= other.x)   &&
            (self.y <= other.y + other.height) &&
            (self.y + self.height >= other.y)
    }
    pub fn contains<P: HasPos>(&self,p:&P) -> bool {
        let pos = p.get_pos();
        self.x <= pos.x && self.y <= pos.y && self.x+self.width >= pos.x && self.y + self.height >= pos.y
    }
    pub fn iter(&self) -> Iter{
         Iter{region:self,pos:Pos::new(self.x,self.y)}
    }
    pub fn split(& self) -> Vec<Region> {
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
}

