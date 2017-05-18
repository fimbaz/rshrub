extern crate fnv;
extern crate rustty;
mod rect;
mod boring_game;
mod tile;
mod grid;
mod neighborhood;
mod ui;
use rect::{BucketPos,Pos,Region,HasPos};

use fnv::FnvHashMap;
use std::hash::Hash;
use boring_game::game::{BoringGame};
use std::cell::RefCell;
use ui::core::UI;
use tile::Tile;
fn main(){
//    let mut ui =  UI::new();
//    ui.pump();
    let mut  game = BoringGame::new();
    for i in 1..1000{
        game.simulate();
        println!("{:?}",game.grid.occupied_buckets().len());
    }
    
}
