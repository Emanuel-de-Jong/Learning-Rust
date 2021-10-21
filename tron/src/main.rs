mod background;
mod player;
mod keybinds;
mod vec2;

use keybinds::Keybinds;
use vec2::Vec2;
use tron::*;
use background::Background;
use player::Player;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::event::{self, KeyCode, KeyMods};
use std::{env, path};
use std::collections::HashSet;

struct MainState {
    keybinds: Keybinds,
    background: Background,
    players: Vec<Player>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let x = (V_GRIDS/10.0).floor()*GRID_SIZE;
        let players = vec![
            Player::new(ctx, "player1".into(), Vec2::new(x, SCREEN_HEIGHT/2.0), Color::new(0.5, 0.5, 1.0, 1.0), Direction::Right)?,
            Player::new(ctx, "player2".into(), Vec2::new(SCREEN_WIDTH-x-GRID_SIZE, SCREEN_HEIGHT/2.0), Color::new(0.5, 1.0, 0.5, 1.0), Direction::Left)?
        ];
        Ok(MainState {
            keybinds: Keybinds::default(),
            background: Background::new(ctx)?,
            players: players
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, player::DRIVES_PER_SECOND) {
            let all_prev_positions: Vec<HashSet<Vec2>> = self.players.iter().map(|player| player.prev_positions.clone()).collect();
            for player in self.players.iter_mut() {
                player.update(ctx, &all_prev_positions)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, background::BG_COLOR);
        
        for player in self.players.iter_mut() {
            player.draw(ctx)?;
        }
        self.background.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, b: bool) {
        if key == self.keybinds.general[&Action::Restart] {

        }

        for player in self.players.iter_mut() {
            player.key_down_event(ctx, key, mods, b, self.keybinds.player(&player.name));
        }
    }
}


pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("tron", "killbottt")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}