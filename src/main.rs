mod rs_2dcanvas;

enum PlayerState {
    Idle,
    Up,
    Down,
    Ai
}

struct Player {
    // position: rs_2dcanvas::Position,
    // size: rs_2dcanvas::Size,
    speed: u32,
    is_ai: bool,
    component: rs_2dcanvas::Rectangle,
    state: PlayerState
}

impl Player {
    pub fn new(is_ai: bool, position: rs_2dcanvas::Position) -> Player {
        let mut color: [f32; 4];
        if is_ai == true {
            color = [1.0, 0.0, 0.0, 1.0];
        } else {
            color = [0.0, 1.0, 0.0, 1.0];
        }
        Player {
            // size: rs_2dcanvas::Size { width: 5, height: 20 },
            speed: 2,
            is_ai: is_ai,
            component: rs_2dcanvas::Rectangle::new(
                position,
                rs_2dcanvas::Size { width: 5, height: 20 },
                color
            ),
            state: if is_ai == true { PlayerState::Ai } else { PlayerState::Idle }
        }
    }
}

fn main() {
    let mut engine = rs_2dcanvas::Engine::new(
        rs_2dcanvas::Size {
            width: 1280, 
            height: 720
        }
    );

    let player = Player::new(
        false,
        rs_2dcanvas::Position {
            x: 10.0,
            y: 10.0
        }
    );

    let enemy = Player::new(
        true,
        rs_2dcanvas::Position {
            x: 1270.0,
            y: 10.0
        }
    );

    println!("State is: {:?}", engine.state);
    engine.start(vec![
        &player.component,
        &enemy.component
    ]);
    println!("State is: {:?}", engine.state);
}
