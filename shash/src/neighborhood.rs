use rect::{HasPos,Pos,BucketPos};
use grid::Grid;
//Next step: make neighbor a slice.  Sigh...
pub enum Neighbor<'t,P: HasPos + 't> {
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
pub struct  Neighborhood2<'t,P: HasPos + 't>(Box<[Neighbor<'t,P>]>);
impl <'t,'r,P: HasPos +'t + 'r>  Neighborhood2<'t,P>{
    pub fn new() -> Neighborhood2<'t,P> {
        Neighborhood2(vec![Neighbor::Point(None),Neighbor::UpLeft(None),Neighbor::Up(None),Neighbor::UpRight(None),
                           Neighbor::Right(None),Neighbor::DownRight(None),Neighbor::Down(None),
                           Neighbor::DownLeft(None),Neighbor::Left(None)].into_boxed_slice())
    }
    pub fn default () -> Neighborhood2<'r,P>{
        Neighborhood2::new()
    }
    pub fn populate(&'r mut self,point: &'t P, iter:&'t mut Iterator<Item=&'r P>){
        let p = point.get_pos();
        for neighbor in iter{
            match neighbor.get_pos() {
                //Point
                n if n.x == p.x && n.y == p.y => { *self.0.get_mut(0).unwrap() = Neighbor::Point(Some(neighbor)); }

                //Top Left
                n if p.x.checked_sub(1).is_some() && p.y.checked_sub(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y-1 => {*self.0.get_mut(1).unwrap() = Neighbor::UpLeft(Some(neighbor)); }
                //Top
                n if p.y.checked_sub(1).is_some() &&
                    n.x == p.x && n.y == p.y-1 => {*self.0.get_mut(2).unwrap() = Neighbor::Up(Some(neighbor)); }

                //Top Right
                n if p.x.checked_add(1).is_some() && p.y.checked_sub(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y-1 => {*self.0.get_mut(3).unwrap() = Neighbor::UpRight(Some(neighbor)); }

                //Right                
                n if p.x.checked_add(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y => {*self.0.get_mut(4).unwrap() = Neighbor::Right(Some(neighbor)); }

                //Bottom Right
                n if p.x.checked_add(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    n.x == p.x+1 && n.y == p.y + 1 => {*self.0.get_mut(5).unwrap() = Neighbor::DownRight(Some(neighbor)); }

                //Bottom
                n if p.y.checked_add(1).is_some() &&
                    n.x == p.x && n.y == p.y + 1 => {*self.0.get_mut(6).unwrap() = Neighbor::Down(Some(neighbor)); }

                //Bottom Left
                n if p.x.checked_sub(1).is_some() &&  p.y.checked_add(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y+1 => {*self.0.get_mut(7).unwrap() = Neighbor::DownLeft(Some(neighbor)); }
                //Left
                n if p.x.checked_sub(1).is_some() &&
                    n.x == p.x-1 && n.y == p.y => {*self.0.get_mut(8).unwrap() = Neighbor::Left(Some(neighbor)); }
                
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

