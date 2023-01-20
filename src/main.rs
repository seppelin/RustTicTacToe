#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;
mod utils;
mod game;

use menu::MenuScene;
use raylib::prelude::*;

pub enum Scene {
    Exit,
    Menu,
    Game2P(game::P2Scene),
}

fn main() {
    let (mut handle, thread) = init().width(1200).height(800).title("Capitalism TTT by Seppelin").build();

    handle.set_target_fps(60);

    let mut scene: Scene = Scene::Menu;

    'scene_selection: loop{
        match scene {
            Scene::Menu => scene = MenuScene::run(&mut handle, &thread),
            Scene::Game2P(mut s) => scene = s.run(&mut handle, &thread),
            _ => break 'scene_selection,
        }
    }
}
