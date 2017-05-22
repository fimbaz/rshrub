use std::hash::{Hash,Hasher};

use std::collections::hash_map;

pub const RECT_BUCKET_SIZE: usize = 10;
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
        fn set_pos(&mut self,x: usize, y: usize);
    }
impl HasPos for Pos {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.x,y:self.y};
    }
    fn set_pos(&mut self,x: usize, y: usize){
        *self = Pos { x , y };
    }

}
#[derive(Clone,Copy,Debug)]
pub struct BucketPos(pub Pos);
impl HasPos for BucketPos {
    fn get_pos(&self) -> Pos {
        return self.0;
    }
    fn set_pos(&mut self,_: usize, _: usize){
        unreachable!();
    }
}

impl  From<Pos> for BucketPos {
    fn from(pos: Pos) -> BucketPos{
        return BucketPos(pos.clone());
    }
}

impl Hash for BucketPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u16((self.0.x/RECT_BUCKET_SIZE) as u16);
        state.write_u16((self.0.y/RECT_BUCKET_SIZE) as u16);
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
    #[allow(dead_code)]
     pub fn new(x: usize,y:usize) -> Pos{
     	 Pos{x: x,y: y}
     }
}
impl BucketPos {
     pub fn new(x: usize,y:usize) -> BucketPos{
     	 BucketPos(Pos{x: x,y: y})
     }
}

#[allow(dead_code)]
pub struct Iter<'t> {
    region: &'t Region,
    pos: BucketPos,
}

impl <'t> Iterator for Iter <'t>{
    type Item = BucketPos;
    fn next(&mut self) -> Option<BucketPos>{
                if !self.region.contains_bucket::<BucketPos>(self.pos){ return None;}
        let mut newpos = self.pos;
        newpos.0.x+=RECT_BUCKET_SIZE;
        if !self.region.contains_bucket::<BucketPos>(newpos){
            newpos = BucketPos::new(self.region.x,newpos.0.y+RECT_BUCKET_SIZE);            
        }
        
        let oldpos = self.pos;
        self.pos = newpos;
        if !self.region.contains_bucket::<BucketPos>(oldpos){ return None;}
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
    //this doesn't work on big regions!  Let's fix it.
    //so wazzamatta?  a region only 'contains_bucket' if the bucket is in one of the corners. of the rect.
    //this worked for neighbor query, but sure doesn't work big random regions!
    //
    //why not just use 'contains'?
    //Does it work if we line the points up with bucket edges?

    //what happens if we /dont/ line up the bucket edges and just use 'contains'?
    //A: we'll miss the last column bucket and row bucket because we'll b
    
    
    pub fn contains_bucket<P: HasPos>(&self,bpos:BucketPos) -> bool {
        return self.contains(&Pos{x:bpos.0.x,y:bpos.0.y}) || 
        bpos == BucketPos::new(self.x,self.y) ||
            bpos == BucketPos::new(self.x+self.width,self.y) ||
            bpos == BucketPos::new(self.x,self.y+self.height) ||
            bpos == BucketPos::new(self.x+self.width,self.y+self.height);
            
    }

    pub fn iter(&self) -> Iter{
         Iter{region:self,pos:BucketPos::new(self.x,self.y)}
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

