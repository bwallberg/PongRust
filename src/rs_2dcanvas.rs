extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

// why do we have to use self:: ?
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

// What exactly does derive(Debug) do!?

#[derive(Debug)]
pub enum State {
    Idle,
    Running,
    Stopped
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub struct Direction {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Boundaries {
    pub x: [f64; 2],
    pub y: [f64; 2]
}

pub struct BoundariesHit {
    pub x: bool,
    pub y: bool
}

#[derive(Debug)]
pub struct Size {
    pub width: u32, // unsigned int, since width/height should never be negative (?)
    pub height: u32
}

pub struct Engine {
    pub state: State,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            state: State::Idle
        }
    }

    pub fn start(&mut self) {
        self.state = State::Running;
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped;
    }

    pub fn render(&mut self, gl: &mut opengl_graphics::GlGraphics, rendering_quene: Vec<&Rectangle>, rendering_args: &RenderArgs) {
        use self::graphics::*;
        gl.draw(rendering_args.viewport(), |c, gl| { // TODO: look into closures
            clear([0.0, 0.0, 0.0, 0.0], gl);
            for component in rendering_quene.iter() {
                component.render(c, gl);
            }
        });
    }
}

pub fn check_boundaries(position: &Position, boundaries: &Boundaries) -> BoundariesHit {
    let mut boundaries_hit = BoundariesHit { x: false, y: false };
    if position.y < boundaries.y[0] || position.y > boundaries.y[1] {
        boundaries_hit.y = true;
    }
    if position.x < boundaries.x[0] || position.x > boundaries.x[1] {
        boundaries_hit.x = true;
    }
    return boundaries_hit;
}

pub fn check_collision(rectangle_a: &Rectangle, rectangle_b: &Rectangle) -> bool {
    let wx_a = rectangle_a.position.x + rectangle_a.size.width as f64;
    let wx_b = rectangle_b.position.x + rectangle_b.size.width as f64;
    let hy_a = rectangle_a.position.y + rectangle_a.size.height as f64;
    let hy_b = rectangle_b.position.y + rectangle_b.size.height as f64;
    
    if rectangle_a.position.x < wx_b && wx_a > rectangle_b.position.x
        && rectangle_a.position.y < hy_b && hy_a > rectangle_b.position.y {
            return true;
    }
    return false;
}

pub struct Rectangle {
    position: Position,
    size: Size,
    color: [f32; 4]
}

impl Rectangle {
    pub fn new(position: Position, size: Size, color: [f32; 4]) -> Rectangle {
        Rectangle {
            position: position,
            size: size,
            color: color
        }
    }
    pub fn update_x(&mut self, x: f64) {
        self.position.x = x;
    }

    pub fn update_y(&mut self, y: f64) {
        self.position.y = y;
    }

    pub fn render(&self, c: self::graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;
        
        let dimensions = rectangle::rectangle_by_corners(0.0, 0.0, self.size.width as f64, self.size.height as f64);
        let transform = c.transform.trans(self.position.x /*- (self.size.width as f64 / 2.0)*/ , self.position.y /*- (self.size.height as f64 / 2.0)*/);

        rectangle(self.color, dimensions, transform, gl);

        // println!("Render Asset position: {:?}, size {:?}", self.position, self.size);
    }
}