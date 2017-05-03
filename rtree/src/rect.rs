pub const MAX_RECT_SIZE: usize = 16384;
#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Region {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}
#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Pos  {pub x:usize,pub y: usize}
    pub trait HasPos {
        fn get_pos(&self) -> Pos;
    }
impl HasPos for Pos {
    fn get_pos(&self) -> Pos {
        return Pos {x:self.x,y:self.y};
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
