use std::process;
extern crate rustty;
extern crate ntree;
mod water;
mod tree;
mod display;
use tree::{Region,QTree,Pos};
use rustty::{Terminal,Event,Color,CellAccessor,HasSize,HasPosition,Size};
use rustty::ui::core::{HorizontalAlign,VerticalAlign,Widget,Frame,Painter,Alignable};
use rustty::ui::{Canvas,Label,Dialog};
use display::WorldView;
use water::{Board,Tile,Material};

fn main() {
//    let mut optiondlg = Dialog::new(50, 6);
//    let mut label = Label::from_str("Hi, this is an example!");
//    label.pack(&optiondlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
//    optiondlg.add_label(label);
//    label.set_text("hello world");
    let mut board = Board::new(30);
//    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 15, y:15}});
//    board.tree.tree.insert(Tile { material: Material::Ground(), pos: Pos {x: 80, y:15}});
    for i in 0..20{
        for j in 0..20{
            board.tree.tree.insert(Tile{material: Material::Water(1.0),pos:Pos{x: j,y: i}});
        }
    }
//    board.tree.delete(Tile { material: Material::Ground(), pos: Pos {x: 5, y:5}});
//    {let mut rangequery = board.tree.tree.range_query_mut(Region { x:0, y:0, width: 1000, height:1000});
//     let a = rangequery.next().unwrap();
//     a.material = Material::Water(2.0);
//     let b = rangequery.next().unwrap();
//     b.material = Material::Water(2.0);
//    }

//    {let mut rangequery = board.tree.tree.range_query_mut(Region { x:0, y:0, width: 1000, height:1000});
//     let a = rangequery.subquery_mut(Region{ x:0,y:0,width:3,height:3});
//z    }
    let mut term = Terminal::new().unwrap();
    let termsize = term.size();

    let mut worldview = WorldView::new(0,0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(10).unwrap() {

            match ch {
                'q' => {break 'main},
                'd' => {board.tree.delete(Tile::new(&worldview.cursor,&Material::Ground()));},
                'k' => {worldview.cursor.y = worldview.cursor.y.checked_sub(1).unwrap_or(worldview.cursor.y)},

                'h' => {worldview.cursor.x = worldview.cursor.x.checked_sub(1).unwrap_or(worldview.cursor.x)},
                'l' => {worldview.cursor.x = worldview.cursor.x+1},
                'j' => {worldview.cursor.y = worldview.cursor.y+1},
                'K' => {worldview.cursor.y = worldview.cursor.y.checked_sub(5).unwrap_or(worldview.cursor.y)},
                'H' => {worldview.cursor.x = worldview.cursor.x.checked_sub(5).unwrap_or(worldview.cursor.x)},
                'L' => {worldview.cursor.x = worldview.cursor.x+5},
                'J' => {worldview.cursor.y = worldview.cursor.y+5},
                

                _ => continue,
            }
        }
        worldview.draw(&mut board,&mut term);
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
