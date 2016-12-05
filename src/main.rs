extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use rand::Rng;


mod rs_2dcanvas;

enum PlayerState {
    Idle,
    Up,
    Down,
    Ai
}

struct Player {
    position: rs_2dcanvas::Position,
    boundaries: rs_2dcanvas::Boundaries,
    speed: f64,
    is_ai: bool,
    state: PlayerState
}

impl Player {
    pub fn new(is_ai: bool, position: rs_2dcanvas::Position, boundaries: rs_2dcanvas::Boundaries) -> Player {
        Player {
            position: position,
            boundaries: boundaries,
            speed: 2.0,
            is_ai: is_ai,
            state: if is_ai == true { PlayerState::Ai } else { PlayerState::Idle }
        }
    }

    pub fn on_tick(&mut self) {
        match self.state {
            PlayerState::Up => self.update_y(),
            PlayerState::Down => self.update_y(),
            PlayerState::Ai => self.on_tick_ai(),
            PlayerState::Idle => self.position.y += 0.0 // How can I do nothing on a match? this is unnecessary
        }
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    fn on_tick_ai(&mut self) {

    }

    fn update_y(&mut self) {
        let mut y = match self.state {
            PlayerState::Up => -self.speed,
            PlayerState::Down => self.speed,
            _ => 0.0
        };

        let new_position = rs_2dcanvas::Position {
            y: self.position.y + y,
            x: self.position.x
        };

        if rs_2dcanvas::check_boundaries(&new_position, &self.boundaries).y == false {
            self.position = new_position; // do we need to cleanup old position?
        }
    }

    fn update_x(&mut self) {
        self.position.x;
    }
}

enum BallState {
    Idle,
    Moving
}

struct Ball {
    state: BallState,
    position: rs_2dcanvas::Position,
    boundaries: rs_2dcanvas::Boundaries,
    direction: rs_2dcanvas::Direction,
    speed: f64,
    speedMod: f64
}

impl Ball {
    pub fn new(position: rs_2dcanvas::Position, boundaries: rs_2dcanvas::Boundaries) -> Ball {
        Ball {
            state: BallState::Idle,
            position: position,
            boundaries: boundaries,
            direction: rs_2dcanvas::Direction { x: 0, y: 0 },
            speed: 2.0,
            speedMod: 0.3
        }
    }

    pub fn on_tick(&mut self)  {
        match self.state {
            BallState::Moving => self.update_position(),
            BallState::Idle => self.state = BallState::Idle // how can this do nothing instead...
        }
    }

    pub fn update_position(&mut self) {
        if self.direction.x == 0  && self.direction.y == 0 {
            let mut rng = rand::thread_rng();
            if rng.gen() {
                self.direction.x = 1;
            } else {
                self.direction.x = -1;
            }
            if rng.gen() {
                self.direction.y = 1;
            } else {
                self.direction.y = -1;
            }
        }

        let mut new_position = self.get_new_position(false);        
        let boundaries_hit = rs_2dcanvas::check_boundaries(&new_position, &self.boundaries);

        if boundaries_hit.y || boundaries_hit.x {
            if boundaries_hit.y {
                self.direction.y *= -1;
            }
            if boundaries_hit.x {
                self.direction.x *= -1;
            }
            new_position = self.get_new_position(true);
        }

        self.position = new_position;
    }

    fn get_new_position(&mut self, speed_increase: bool) -> rs_2dcanvas::Position {
        let mut new_position = rs_2dcanvas::Position {
            y: self.position.y,
            x: self.position.x
        };

        if(speed_increase) {
            self.speed += self.speedMod;
        }

        if self.direction.x > 0 {
            new_position.x += self.speed;
        } else if self.direction.x < 0 {
            new_position.x -= self.speed;
        }

        if self.direction.y > 0 {
            new_position.y += self.speed;
        } else if self.direction.y < 0 {
            new_position.y -= self.speed;
        }

        return new_position;
    }

    pub fn toggle_state(&mut self) {
        match self.state {
            BallState::Moving => self.set_idle(),
            BallState::Idle => self.set_moving()
        }
    }

    fn set_moving(&mut self) {
        self.state = BallState::Moving
    }

    fn set_idle(&mut self) {
        self.state = BallState::Idle;
        // this is obviously bad...
        self.position.x = 1280.0/2.0;
        self.position.y = 720.0/2.0;
        self.direction.x = 0;
        self.direction.y = 0;
    }
}

fn main() {
    let mut engine = rs_2dcanvas::Engine::new();
    let mut player = Player::new(
        false,
        rs_2dcanvas::Position {
            x: 10.0,
            y: 720.0/2.0
        },
        rs_2dcanvas::Boundaries {
            x: [0.0, 10.0],
            y: [10.0, 710.0]
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
            y: 720.0/2.0
        },
        rs_2dcanvas::Boundaries {
            x: [1270.0, 1280.0],
            y: [10.0, 710.0]
        }
    );

    let mut enemy_component = rs_2dcanvas::Rectangle::new(
        enemy.position.clone(),
        rs_2dcanvas::Size { width: 5, height: 20 },
        [1.0, 0.0, 0.0, 1.0]
    );

    let mut ball = Ball::new(
        rs_2dcanvas::Position {
            x: 1280.0/2.0,
            y: 720.0/2.0
        },
        rs_2dcanvas::Boundaries {
            x: [0.0, 1280.0],
            y: [5.0, 715.0]
        }
    );

    let mut ball_component = rs_2dcanvas::Rectangle::new(
        ball.position.clone(),
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
                Key::Space => ball.toggle_state(),
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
            ball.on_tick();
        }

        if let Some(r) = e.render_args() {
            player_component.update_y(player.position.y);
            enemy_component.update_y(enemy.position.y);
            ball_component.update_y(ball.position.y);
            ball_component.update_x(ball.position.x);
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
