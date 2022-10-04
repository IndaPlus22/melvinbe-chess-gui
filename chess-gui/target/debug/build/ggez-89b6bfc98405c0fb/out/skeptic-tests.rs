extern crate skeptic;
#[test] fn generativeart_sect_project_setup_line_17() {
    let s = &format!(r####"
{}"####, r####"use ggez::*;


struct State {}


impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        graphics::present(ctx)?;

        Ok(())

    }

}


fn main() {

    let state = State {};

    let cb = ggez::ContextBuilder::new("generative_art", "awesome_person");

    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_73() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"graphics::Mesh::new_circle(

    ctx,

    graphics::DrawMode::fill(),

    mint::Point2{x: 200.0, y: 300.0},

    100.0,

    0.1,

    graphics::Color::WHITE,

)?;

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_105() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler<GameError> for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {

    graphics::clear(ctx, graphics::Color::BLACK);

    let circle = graphics::Mesh::new_circle(

        ctx,

        graphics::DrawMode::fill(),

        mint::Point2{x: 200.0, y: 300.0},

        100.0,

        0.1,

        graphics::Color::WHITE,

    )?;

    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

    graphics::present(ctx)?;

    Ok(())

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_129() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"graphics::Mesh::new_rectangle(

    ctx,

    graphics::DrawMode::fill(),

    graphics::Rect::new(500.0, 250.0, 200.0, 100.0),

    graphics::Color::WHITE,

)?;

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_150() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler<GameError> for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {

    graphics::clear(ctx, graphics::Color::BLACK);

    let rect = graphics::Mesh::new_rectangle(

        ctx,

        graphics::DrawMode::fill(),

        graphics::Rect::new(500.0, 250.0, 200.0, 100.0),

        graphics::Color::WHITE,

    )?;

    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    graphics::present(ctx)?;

    Ok(())

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_180() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"struct State {

    shapes: Vec<Shape>,

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_191() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"enum Shape {

    Circle(mint::Point2<f32>, f32),

    Rectangle(graphics::Rect),

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_200() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl event::EventHandler<GameError> for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

{}

"####, r####"fn main() {

    let mut shapes = Vec::new();

    shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(

        10.0,

        10.0,

        50.0,

        100.0,

    )));

    shapes.push(Shape::Circle(

        mint::Point2{x: 400.0, y: 40.0},

        30.0,

    ));

    let state = State { shapes: shapes };

    let c = conf::Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("generative_art", "awesome_person")

        .default_conf(c)

        .build()

        .unwrap();

    event::run(ctx, event_loop, state);

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_226() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler<GameError> for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {

    graphics::clear(ctx, graphics::Color::BLACK);

    for shape in &self.shapes {

        // Make the shape...

        let mesh = match shape {

            &Shape::Rectangle(rect) => {

                graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::WHITE)?

            }

            &Shape::Circle(origin, radius) => {

                graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::Color::WHITE)?

            }

        };


        // ...and then draw it.

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

    }

    graphics::present(ctx)?;

    Ok(())

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_266() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"use oorandom::Rand32;

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_271() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"    let mut rng = Rand32::new(4); // Random number chosen by fair die roll

shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(

    rng.rand_float() * 800.0,

    rng.rand_float() * 600.0,

    rng.rand_float() * 800.0,

    rng.rand_float() * 600.0,

)));

shapes.push(Shape::Circle(

    mint::Point2{

        x: rng.rand_float() * 800.0,

        y: rng.rand_float() * 600.0,

    },

    rng.rand_float() * 300.0,

));

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_293() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"let mut shapes = Vec::new();

for _ in 0..8 {

    if rng.rand_i32() >= 0 {

        shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(

            rng.rand_float() * 800.0,

            rng.rand_float() * 600.0,

            rng.rand_float() * 800.0,

            rng.rand_float() * 600.0,

        )));

    } else {

        shapes.push(Shape::Circle(

            mint::Point2{

                x: rng.rand_float() * 800.0,

                y: rng.rand_float() * 600.0,

            },

            rng.rand_float() * 300.0,

        ));

    }

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_59() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"use ggez::*;

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_83() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"struct State {}


impl ggez::event::EventHandler<GameError> for State {

  fn update(&mut self, ctx: &mut Context) -> GameResult {

      Ok(())

  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {

      Ok(())

  }

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_109() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

struct State {{

}}

{}
"####, r####"pub fn main() {

    let state = State {};

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_156() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler<GameError> for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ctx, event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"let c = conf::Conf::new();

let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")

    .default_conf(c)

    .build()

    .unwrap();

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_171() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler<GameError> for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ctx, event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"event::run(ctx, event_loop, state);

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_210() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"struct State {

    dt: std::time::Duration,

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_220() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler<GameError> for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ctx, event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"let state = State {

    dt: std::time::Duration::new(0, 0),

};

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_229() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl event::EventHandler<GameError> for State {{
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn update(&mut self, ctx: &mut Context) -> GameResult {

    self.dt = timer::delta(ctx);

    Ok(())

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn helloggez_sect_ggez_line_237() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler<GameError> for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {

    println!("Hello ggez! dt = {}ns", self.dt.as_nanos());

    Ok(())

}

"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn readme_sect_what_is_this_line_117() {
    let s = &format!(r####"
{}"####, r####"use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}
"####);
    skeptic::rt::compile_test(r#"C:\Users\melvi\.cargo\registry\src\github.com-1ecc6299db9ec823\ggez-0.6.1"#, r#"c:\Users\melvi\Documents\GitHub\IndaPlus\melvinbe-chess-gui\chess-gui\target\debug\build\ggez-89b6bfc98405c0fb\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

