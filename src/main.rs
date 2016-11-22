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
    speed: u32,
    is_ai: bool,
    state: PlayerState
}

impl Player {
    pub fn new(is_ai: bool, position: rs_2dcanvas::Position) -> Player {
        Player {
            position: position,
            // size: rs_2dcanvas::Size { width: 5, height: 20 },
            speed: 2,
            is_ai: is_ai,
            state: if is_ai == true { PlayerState::Ai } else { PlayerState::Idle }
        }
    }

    pub fn on_tick(&mut self, component: &rs_2dcanvas::Rectangle) {
        component.update_y(2.0);
        println!("wat");
    }
}

fn main() {
    let mut engine = rs_2dcanvas::Engine::new(
        rs_2dcanvas::Size {
            width: 1280, 
            height: 720
        }
    );

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

    println!("State is: {:?}", engine.state);



    // Cannot do two borrows below, perhaps make component a borrow to Player? or how...

    let on_render_vector = vec![
        &player_component,
        &enemy_component
    ];

    engine.start(on_render_vector, () => {
        // this doesn't work
        player.on_tick(&player_component);
        enemy.on_tick(&enemy_component);
    });
    println!("State is: {:?}", engine.state);
}
