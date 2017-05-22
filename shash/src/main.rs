extern crate fnv;
extern crate rustty;
mod rect;
mod boring_game;
mod tile;
mod grid;
mod neighborhood;
mod ui;
use rustty::{HasSize,Terminal,Event};
use rustty::ui::core::{Widget};
use rustty::ui::{Label};
use rect::{BucketPos,Pos,Region,HasPos};

use fnv::FnvHashMap;
use std::hash::Hash;
use boring_game::game::{BoringGame};
use std::cell::RefCell;
use ui::worldview::WorldView;
use tile::Tile;
fn main(){
    let mut term = Terminal::new().unwrap();
    let mut termsize = term.size();
    let mut worldview = WorldView::new(termsize.0,termsize.1);
    let mut  game = BoringGame::new();
    let mut point_info = Label::new(0,0);
    game.simulate();
    'main: loop{
        while let Some(Event::Key(ch)) = term.get_event(10).unwrap(){
            match ch{
                'q' => { panic!();}
                'i' => { game.insert_air(worldview.cursor,200000.0);}
                'k' => {worldview.cursor.y = worldview.cursor.y.checked_sub(1).unwrap_or(worldview.cursor.y)},
                'h' => {worldview.cursor.x = worldview.cursor.x.checked_sub(1).unwrap_or(worldview.cursor.x)},
                'l' => {worldview.cursor.x = worldview.cursor.x+1},
                'j' => {worldview.cursor.y = worldview.cursor.y+1},
                'K' => {worldview.cursor.y = worldview.cursor.y.checked_sub(5).unwrap_or(worldview.cursor.y)},
                'H' => {worldview.cursor.x = worldview.cursor.x.checked_sub(5).unwrap_or(worldview.cursor.x)},
                'L' => {worldview.cursor.x = worldview.cursor.x+5},
                'J' => {worldview.cursor.y = worldview.cursor.y+5},
                _ => {}
            }
            game.simulate();
            worldview.update_world(&game);
            worldview.draw(&mut term);
            
            term.swap_buffers().unwrap();
        }
    }
    for i in 1..1000{
        game.simulate();
        println!("{:?}",game.grid.occupied_buckets().len());
    }
}
