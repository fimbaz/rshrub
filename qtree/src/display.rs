use water::{Board,Material};
use tree::{Pos,Region};
use rustty::{Color,CellAccessor,HasSize,HasPosition,Size,Cell,Attr};
use rustty::ui::core::{Widget,HorizontalAlign,VerticalAlign,Frame,Painter,Alignable};
pub struct WorldView<'a> {
    board: &'a Board,
    pub origin: Pos, // (x,y) in the UI -> (x+origin.x,y+origin.y) in the game world.
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
     	 return WorldView { board: board, origin: Pos {x:x,y:y} };
     }
    pub fn draw_world(&self,frame: &mut CellAccessor){
        self.draw_background(frame);
        self.draw_active(frame);
    }
    pub fn draw_active(&self,frame: &mut CellAccessor){
        let frame_size=frame.size();
        let region = Region::rectangle(self.origin.x,self.origin.y,frame_size.0 as u16,frame_size.1 as u16);
        let result = self.board.tree.tree.range_query(&region);
        for tile in result {
            let pos = Pos {x:tile.pos.x - self.origin.x, y:tile.pos.y - self.origin.y};
            let index = frame.pos_to_index(pos.x as usize,pos.y as usize).clone();
            match index {
                Some(i) =>{frame.cellvec_mut()[i] = Cell::from(tile.material);}
                None => {}
            }
                             
        }

    }
    pub fn draw_background(& self,frame: &mut CellAccessor){
        let frame_ground_level = self.board.ground_level.checked_sub(self.origin.y).unwrap_or(0);
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
