use water::{Board,Material};
use tree::{Pos,Region};
use ntree::Region as NTRegion;
use rustty::{Color,CellAccessor,HasSize,HasPosition,Size,Cell,Attr};
use rustty::ui::core::{Widget,HorizontalAlign,VerticalAlign,Frame,Painter,Alignable};
use rustty::ui::{Canvas};
use std::io::{self,Write};
use std::process;
pub struct WorldView<'a> {
    board: &'a Board,
    pub origin: Pos, // (x,y) in the UI -> (x+origin.x,y+origin.y) in the game world.
    pub  cursor: Pos, // position of cursor in the /world/ (that way we can take references to a size-changing terminal)
}

impl From<Material> for Cell{
    fn from(material: Material) -> Cell {
        match material {
            Material::Ground() =>
                Cell::new('.',Color::Default,Color::Default,Attr::Default)
                ,
            Material::Air() =>
                Cell::new(' ',Color::Default,Color::Default,Attr::Default)
                ,
            Material::Water(f) =>
                Cell::new('▮',Color::Blue,Color::Blue,Attr::Default)
        }
    }
}

impl <'a>  WorldView<'a> {
     pub fn new(board: &'a Board,x: u16,y: u16) -> WorldView<'a> {
     	 return WorldView { board: board, origin: Pos {x:x,y:y}, cursor: Pos {x:0,y:0}};
     }
    pub fn draw(&mut self,frame:&mut CellAccessor){
        WorldView::draw_world(self.board,&mut self.origin,&self.cursor,frame);
    }
    fn adjust_origin(board:&Board,origin: &mut Pos,cursor: &Pos,frame: &mut CellAccessor){
        let frame_size = (frame.size().0 as u16,frame.size().1 as u16);
        if (Region {x:origin.x,y:origin.y,width:frame_size.0,height:frame_size.1}).contains(cursor){
            if cursor.x > origin.x + frame_size.0{
                origin.x = cursor.x.saturating_sub(origin.x + frame_size.0)
            }else if cursor.x < origin.x {
                origin.x = cursor.x
            }
            if cursor.y > origin.y + frame_size.1{
                origin.y = cursor.y.saturating_sub(frame_size.1)
            }else if cursor.y < origin.y{
                origin.y = cursor.y
            }
        }
    }
    fn draw_world(board:&Board,origin: &mut Pos,cursor: &Pos,frame: &mut CellAccessor){
        WorldView::adjust_origin(board,origin,cursor,frame);
        WorldView::draw_background(board,origin,cursor,frame);
        WorldView::draw_active(board,origin,cursor,frame);
    }
    pub fn draw_active(board:&Board,origin:&Pos,cursor: &Pos,frame: &mut CellAccessor){
        //let region = Region::rectangle(self.origin.x,self.origin.y,frame_size.0 as u16,frame_size.1 as u16);
        let frame_size = frame.size();
        let region = Region::rectangle(origin.x,origin.y,frame_size.0 as u16,frame_size.1 as u16);
        let result = board.tree.tree.range_query(region);

        for tile in result {
            let pos = Pos {x:tile.pos.x - origin.x, y:tile.pos.y - origin.y};
            let index = frame.pos_to_index(pos.x as usize,pos.y as usize).clone();
            match index {
                Some(i) =>{frame.cellvec_mut()[i] = Cell::from(tile.material)},
                None => {/* println!("{:?}",pos)*/}
            }
        }
        let cursor_index = if let Some(i) = frame.pos_to_index(cursor.x.saturating_sub(origin.x) as usize,cursor.y.saturating_sub(origin.y)as usize) {i} else { 0 } ;
        frame.cellvec_mut().get_mut(cursor_index).unwrap().set_attrs(Attr::Reverse);


        
    }
    pub fn set_cursor(&mut self,pos: &Pos) {
        self.cursor = *pos;
    }
    pub fn set_origin(&mut self,pos: &Pos){
        self.origin = *pos;
    }

    pub fn draw_background(board:&Board,origin:&Pos,cursor: &Pos,frame: &mut CellAccessor){
        let frame_ground_level = board.ground_level.checked_sub(origin.y).unwrap_or(0);
        let mut frame_coords: (u16,u16)=(0,0);
        let frame_size = frame.size();
        for cell in frame.cellvec_mut().iter_mut(){
            if frame_coords.0 == frame_size.0 as u16{
                frame_coords.0  = 0;
                frame_coords.1 = frame_coords.1 + 1;
            }
            if frame_coords.1 == frame_size.1 as u16{
                break;
            }
            frame_coords.0=frame_coords.0+1;
            if frame_coords.1 >= frame_ground_level{
                *cell=  Cell::from(Material::Ground())
            }else{
                *cell= Cell::from(Material::Air())
            }
        }
    }
}
