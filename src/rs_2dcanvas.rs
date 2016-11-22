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
pub struct Size {
    pub width: u32, // unsigned int, since width/height should never be negative (?)
    pub height: u32
}

pub struct Engine {
    window: Window,
    gl: GlGraphics,
    // pub renderingQuene: Vec<&'a Rectangle>,
    pub state: State,
    pub resolution: Size
}

impl Engine {
    pub fn new(resolution: Size) -> Engine {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new(
                "RS 2DCanvas",
                [resolution.width, resolution.height]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Engine {
            state: State::Idle,
            resolution: resolution,
            window: window,
            gl: GlGraphics::new(opengl)
        }
    }

    pub fn start<UpdateFunction: Fn()>(&mut self, rendering_quene: Vec<&Rectangle>, app_update: UpdateFunction) {
        self.state = State::Running;
        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&rendering_quene, &r);

            }

            if let Some(u) = e.update_args() {
                app_update();
                // app.update(&u);
            }
        }
        self.state = State::Stopped;
    }

    fn render(&mut self, rendering_quene: &Vec<&Rectangle>, rendering_args: &RenderArgs) {
        use self::graphics::*;
        self.gl.draw(rendering_args.viewport(), |c, gl| { // TODO: look into closures
            clear([0.0, 0.0, 0.0, 0.0], gl);
            for component in rendering_quene.iter() {
                component.render(c, gl);
            }
        });
    }
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
        let transform = c.transform.trans(self.position.x, self.position.y);

        rectangle(self.color, dimensions, transform, gl);

        // println!("Render Asset position: {:?}, size {:?}", self.position, self.size);
    }
}