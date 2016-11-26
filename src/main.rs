extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


mod rs_2dcanvas;

enum PlayerState {
    Idle,
    Up,
    Down,
    Ai
}

struct Player {
    position: rs_2dcanvas::Position,
    // size: rs_2dcanvas::Size,
    speed: f64,
    is_ai: bool,
    state: PlayerState
}

impl Player {
    pub fn new(is_ai: bool, position: rs_2dcanvas::Position) -> Player {
        Player {
            position: position,
            // size: rs_2dcanvas::Size { width: 5, height: 20 },
            speed: 2.0,
            is_ai: is_ai,
            state: if is_ai == true { PlayerState::Ai } else { PlayerState::Idle }
        }
    }

    pub fn on_tick(&mut self) {
        match self.state {
            PlayerState::Up => self.position.y -= self.speed,
            PlayerState::Down => self.position.y += self.speed,
            PlayerState::Ai => self.on_tick_ai(),
            PlayerState::Idle => self.position.y += 0.0 // How can I do nothing on a match? this is unnecessary
        }
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    fn on_tick_ai(&mut self) {

    }

    fn update_y(&mut self, y: f64) {
        self.position.y;
    }

    fn update_x(&mut self, x: f64) {
        self.position.x;
    }
}

fn main() {
    let mut engine = rs_2dcanvas::Engine::new();
    let mut player = Player::new(
        false,
        rs_2dcanvas::Position {
            x: 10.0,
            y: 10.0
        }
    );

    let mut player_component = rs_2dcanvas::Rectangle::new(
        player.position.clone(),
        rs_2dcanvas::Size { width: 5, height: 20 },
        [0.0, 1.0, 0.0, 1.0]
    );

    let mut enemy = Player::new(
        true,
        rs_2dcanvas::Position {
            x: 1270.0,
            y: 10.0
        }
    );

    let mut enemy_component = rs_2dcanvas::Rectangle::new(
        enemy.position.clone(),
        rs_2dcanvas::Size { width: 5, height: 20 },
        [1.0, 0.0, 0.0, 1.0]
    );

    let mut ball_component = rs_2dcanvas::Rectangle::new(
        rs_2dcanvas::Position {
            x: 1280.0/2.0,
            y: 720.0/2.0
        },
        rs_2dcanvas::Size { width: 5, height: 5 },
        [1.0, 1.0, 1.0, 1.0]
    );

    println!("State is: {:?}", engine.state);

    let opengl = opengl_graphics::OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
                "RS 2DCanvas",
                [1280, 720]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut gl = GlGraphics::new(opengl);

    engine.start();
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => player.set_state(PlayerState::Up),
                Key::Down => player.set_state(PlayerState::Down),
                _ => println!("Pressed keyboard key {:?}", key)
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Up => player.set_state(PlayerState::Idle),
                Key::Down => player.set_state(PlayerState::Idle),
                _ => println!("Released keyboard key {:?}", key)
            }
        };


        if let Some(u) = e.update_args() {
            player.on_tick();
            enemy.on_tick();
        }

        if let Some(r) = e.render_args() {
            player_component.update_y(player.position.y);
            enemy_component.update_y(enemy.position.y);
            engine.render(&mut gl, vec![
                &player_component,
                &enemy_component,
                &ball_component
            ], &r);
        }
    }
    engine.stop();

    println!("State is: {:?}", engine.state);
}
