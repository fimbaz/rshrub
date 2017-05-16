use rect::{HasPos,Pos,BucketPos};
use grid::Grid;
use std::rc::Rc;
use std::fmt::Debug;
use std::cell::RefCell;
pub enum Neighbor2{
    Point = 0,    
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,

}
//Next step: make neighbor a slice.  Sigh...
#[derive(Debug)]
pub enum Neighbor<'t,P: HasPos + 't + Debug> {
    UpLeft(Option<&'t P>),
    Up(Option<&'t P>),
    UpRight(Option<&'t P>),
    Right(Option<&'t P>),
    DownRight(Option<&'t P>),
    Down(Option<&'t P>),
    DownLeft(Option<&'t P>),
    Left(Option<&'t P>),
    Point(Option<&'t P>)

}
impl Neighbor2{
    pub fn from_usize(n: usize) -> Option<Neighbor2>{
        match n{
            0 => Some(Neighbor2::Point),
            1 => Some(Neighbor2::UpLeft),
            2 => Some(Neighbor2::Up),
            3 => Some(Neighbor2::UpRight),
            4 => Some(Neighbor2::Right),
            5 => Some(Neighbor2::DownRight),
            6 => Some(Neighbor2::Down),
            7 => Some(Neighbor2::DownLeft),
            8 => Some(Neighbor2::Left),
            _ => None
                
        }
    }
    pub fn get_pos(&self,p: &HasPos) -> Option<Pos>{
        let n2 = self;
        let point = p.get_pos();
        match n2 {
            UpLeft => {
                if point.x != 0 && point.y != 0{
                    return Some(Pos::new(point.x-1,point.y-1))
                }
            }
            Up => {
                if point.y != 0{
                    return Some(Pos::new(point.x,point.y-1))
                }
            }
            UpRight => {
                if point.x != usize::max_value() && point.y != 0{
                    return Some(Pos::new(point.x+1,point.y-1))
                }
            }
            Right => {
                if point.x != usize::max_value(){
                    return Some(Pos::new(point.x+1,point.y))
                }
            }
            DownRight => {
                if point.x != usize::max_value() && point.y != usize::max_value(){
                    return Some(Pos::new(point.x+1,point.y+1))
                }
            }
            Down => {
                if  point.y != usize::max_value(){
                    return Some(Pos::new(point.x,point.y+1))
                }
            }
            DownLeft => {
                if point.x != 0 && point.y != usize::max_value(){
                    return Some(Pos::new(point.x-1,point.y+1))
                }
            }
            Left => {
                if point.x != 0{
                    return Some(Pos::new(point.x-1,point.y))
                }
            }
            Point => {
                return Some(Pos::new(point.x,point.y))
            }


        }
        return None;
    }
}
#[derive(Debug)]
pub struct  Neighborhood2<'t,P: HasPos + 't + Debug>{
    pub neighbors: &'t mut Box<[Option<Rc<P>>]>,
    bucket: Rc<RefCell<Vec<Rc<P>>>>,
    grid: &'t Grid<P>,
}

impl  <'t,P: HasPos +'t + Debug>  Drop for  Neighborhood2<'t,P>{
    fn drop(&mut self){
    }
}
impl <'t,P: HasPos +'t + Debug>  Neighborhood2<'t,P>{
    pub fn new(grid: &'t Grid<P>,neighborhood_vec: &'t mut Box<[Option<Rc<P>>]>,bucket:Rc<RefCell<Vec<Rc<P>>>>) -> Neighborhood2<'t,P> {
        Neighborhood2{neighbors:neighborhood_vec,grid: grid,bucket:bucket}
    }
    pub fn get_neighbor(&self,nbor: Neighbor2) -> Option<Rc<P>>{
        self.neighbors.get(nbor as usize).unwrap().clone()
    }
    pub fn len(&self) -> usize {
        self.neighbors.iter().fold(0,|i,x|if let &Some(ref f) = x {i+1} else {i})
    }
    pub fn populate<'r,'s>(&'s mut self,point: &'r P, iter:&'r mut Iterator<Item=Rc<P>>){
        let p = point.get_pos();
        for n in self.neighbors.iter_mut(){
            *n = None;
        }
        for neighbor in iter{
            match neighbor.get_pos() {
                //Point
                n if n.x == p.x && n.y == p.y => {  *self.neighbors.get_mut(0).unwrap() = Some(neighbor); }
                
                //Top Left
                n if p.x.checked_sub(1).is_some() && p.y.checked_sub(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y-1 => {*self.neighbors.get_mut(1).unwrap() = Some(neighbor);} 
                //Top
                n if p.y.checked_sub(1).is_some() &&
                    n.x == p.x && n.y == p.y-1 => {*self.neighbors.get_mut(2).unwrap() = Some(neighbor); }
                
                //Top Right
                n if p.x.checked_add(1).is_some() && p.y.checked_sub(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y-1 => {*self.neighbors.get_mut(3).unwrap() = Some(neighbor); }
                
                //Right                
                n if p.x.checked_add(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y => { *self.neighbors.get_mut(4).unwrap() = Some(neighbor); }
                
                //Bottom Right
                n if p.x.checked_add(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y + 1 => {*self.neighbors.get_mut(5).unwrap() = Some(neighbor); }

                //Bottom
                n if p.y.checked_add(1).is_some() &&
                    n.x == p.x && n.y == p.y + 1 => {*self.neighbors.get_mut(6).unwrap() = Some(neighbor); }
                
                //Bottom Left
                n if p.x.checked_sub(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y+1 => {*self.neighbors.get_mut(7).unwrap() = Some(neighbor); }
                //Left
                n if p.x.checked_sub(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y => {*self.neighbors.get_mut(8).unwrap() = Some(neighbor); }
                
                _ => {}
            }

        }
    }
}
#[derive(Debug)]
pub struct Neighborhood<'t,P: HasPos + 't> {
    pub top: Option<&'t P>,
    pub top_right: Option<&'t P>,
    pub right: Option<&'t P>,
    pub bottom_right: Option<&'t P>,
    pub bottom: Option<&'t P>,
    pub bottom_left: Option<&'t P>,
    pub left: Option<&'t P>,
    pub top_left: Option<&'t P>,
    pub point: Option<&'t P>
}

impl <'t,P: HasPos> Neighborhood<'t,P> {
    pub fn default() -> Neighborhood<'t,P>{
        Neighborhood { top: None, top_right: None, right: None, bottom_right: None, bottom: None, bottom_left: None, left: None, top_left: None, point: None}
    }

    pub fn new<'r>(point: &'t P, iter:&'t mut Iterator<Item=&'r P>) -> Neighborhood<'r,P>{
        let mut nhood = Neighborhood::default();
        let p = point.get_pos();
        for item in iter{
            match item.get_pos() {
                //Point
                nbor if nbor.x == p.x && nbor.y == p.y => { nhood.point = Some(item); }

                //Top left
                nbor if p.x.checked_sub(1).is_some() && p.y.checked_sub(1).is_some() &&
                    nbor.x == p.x-1 && nbor.y == p.y-1 => {nhood.top_left = Some(item); }

                //Top
                nbor if p.y.checked_sub(1).is_some() &&
                    nbor.x == p.x && nbor.y == p.y-1 => {nhood.top = Some(item); }

                //Top Right
                nbor if p.x.checked_add(1).is_some() && p.y.checked_sub(1).is_some() &&
                    nbor.x == p.x+1 && nbor.y == p.y-1 => {nhood.top_right = Some(item); }

                //Right                
                nbor if p.x.checked_add(1).is_some() &&
                    nbor.x == p.x+1 && nbor.y == p.y => {nhood.right = Some(item); }

                //Bottom Right
                nbor if p.x.checked_add(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    nbor.x == p.x+1 && nbor.y == p.y + 1 => {nhood.bottom_right = Some(item); }

                //Bottom
                nbor if p.y.checked_add(1).is_some() &&
                    nbor.x == p.x && nbor.y == p.y + 1 => {nhood.bottom = Some(item); }

                //Bottom Left
                nbor if p.x.checked_sub(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    nbor.x == p.x-1 && nbor.y == p.y+1 => {nhood.bottom_left = Some(item); }

                nbor if p.x.checked_sub(1).is_some() &&
                    nbor.x == p.x-1 && nbor.y == p.y => {nhood.left = Some(item); }
                _ => {}
            }
        }
        return nhood;
    }
    pub fn len(&self) -> usize {
        let mut  count = 0;
        if self.top_right.is_some() { count +=1;}
        if self.top.is_some() { count +=1;}
        if self.top_left.is_some() { count +=1;}
        if self.right.is_some() { count +=1;}
        if self.bottom_right.is_some() { count +=1;}
        if self.bottom.is_some() { count +=1;}
        if self.bottom_left.is_some() { count +=1;}
        if self.left.is_some() { count +=1;}
        if self.point.is_some() { count +=1;}
        count
    }
}

