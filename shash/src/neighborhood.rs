use rect::{HasPos,Pos,BucketPos};
use grid::Grid;
use std::fmt::Debug;
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

#[derive(Debug)]
pub struct  Neighborhood2<'t,P: HasPos + 't + Debug>{
    neighbors: Box<[Option<&'t P>]>,
    grid: &'t Grid<P>,
}

impl <'t,P: HasPos +'t + Debug>  Neighborhood2<'t,P>{
    pub fn new(grid: &'t Grid<P>) -> Neighborhood2<'t,P> {
        Neighborhood2{neighbors:vec![None,None,None,None,None,None,None,None,None].into_boxed_slice(),grid: grid}
    }
    pub fn get_neighbor(&self,nbor: Neighbor2) -> Option<&P>{
        *self.neighbors.get(nbor as usize).unwrap()
    }
    pub fn len(&self) -> usize {
        self.neighbors.iter().fold(0,|i,x|if let &Some(f) = x {i+1} else {i})
    }

    pub fn populate<'r,'s>(&'s mut self,point: &'r P, iter:&'r mut Iterator<Item=&'t P>){
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

