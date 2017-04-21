use std::process;
extern crate rustty;
extern crate ntree;
mod water;
mod tree;
mod display;
use tree::{Region,QTree,Pos};
use rustty::{Terminal,Event,Color,CellAccessor,HasSize,HasPosition,Size};
use rustty::ui::core::{HorizontalAlign,VerticalAlign,Widget,Frame,Painter,Alignable};
use rustty::ui::{Canvas,Label};
use display::WorldView;
use water::{Board,Tile,Material};
fn main() {
    let mut board = Board::new(30);
    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 100, y:100}});
    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 15, y:15}});
    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 80, y:15}});
    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 100, y:15}});
    for tile in board.tree.tree.range_query_mut(&Region { x:0, y:0, width: 1000, height:1000}){
        tile.material = Material::Water(2.0)
    }
    let mut term = Terminal::new().unwrap();


    let mut worldview = WorldView::new(&board,0,0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(10).unwrap() {

            match ch {
                'q' => {break 'main},
                'k' => {worldview.origin.y = worldview.origin.y.checked_sub(1).unwrap_or(worldview.origin.y)},
                'h' => {worldview.origin.x = worldview.origin.x.checked_sub(1).unwrap_or(worldview.origin.x)},
                'l' => {worldview.origin.x = worldview.origin.x+1},
                'j' => {worldview.origin.y = worldview.origin.y+1},
                'K' => {worldview.origin.y = worldview.origin.y.checked_sub(5).unwrap_or(worldview.origin.y)},
                'H' => {worldview.origin.x = worldview.origin.x.checked_sub(5).unwrap_or(worldview.origin.x)},
                'L' => {worldview.origin.x = worldview.origin.x+5},
                'J' => {worldview.origin.y = worldview.origin.y+5},


                _ => continue,
            }
        }
        worldview.draw_world(&mut term);
        term.swap_buffers().unwrap();
    }
}
       
/*
fn main(){
    let mut ntree = QTree::new(Region::square(0,0,100),4);
    ntree.insert(Pos{x:5,y:5});
    ntree.insert(Pos{x:5,y:6});
    ntree.insert(Pos{x:5,y:7});
    ntree.insert(Pos{x:6,y:5});
    ntree.insert(Pos{x:6,y:6});
    ntree.insert(Pos{x:6,y:7});
    ntree.insert(Pos{x:7,y:5});
    ntree.insert(Pos{x:7,y:6});
    ntree.insert(Pos{x:7,y:7});
    println!("{:?}","yeesh");
    let region = Region::square(5,5,3);
    //    println!("{:?}",region.contains(&Pos{x: 3, y:3}));
    let points = ntree.range_query(&region).map(|x| x.clone()).collect::<Vec<Pos>>();
    println!("{:?}",points);
}
*/
