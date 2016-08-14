extern crate euclid;
#[macro_use]
extern crate glium;
extern crate image;
extern crate vitral;
extern crate serde;
extern crate calx_color;
#[macro_use]
extern crate calx_resource;
extern crate calx_grid;
extern crate world;

mod backend;
mod game_view;
mod sprite;
mod view;

use calx_color::color::*;
pub use euclid::{Point2D, Rect, Size2D};
pub use glium::{DisplayBuild, glutin};
pub use backend::Backend;
pub use calx_resource::{Loadable, Resource, ResourceCache, ResourceStore};
use world::{BrushBuilder, Frame};
use world::World;
use world::query;
use world::terrain;
use world::Location;
use game_view::GameView;

fn init_brushes<V: Copy + Eq>(builder: &mut vitral::Builder<V>) {
    BrushBuilder::new(builder)
        .file("content/assets/floors.png")
        ////
        .tile(0, 0)
        .brush("blank_floor")
        ////
        .color(DARKGREEN)
        .tile(32, 0)
        .brush("grass")
        ////
        .color(DARKGREEN)
        .tile(64, 0)
        .brush("grass2")
        ////
        .color(SLATEGRAY)
        .tile(32, 0)
        .brush("ground")
        ////
        .color(ROYALBLUE)
        .tile(96, 0)
        .brush("water")
        ////
        .file("content/assets/portals.png")
        ////
        .color(LIGHTCYAN)
        .tile(0, 0)
        .frame()
        .tile(32, 0)
        .frame()
        .tile(64, 0)
        .frame()
        .tile(96, 0)
        .frame()
        .tile(128, 0)
        .frame()
        .tile(160, 0)
        .frame()
        .tile(192, 0)
        .frame()
        .tile(224, 0)
        .frame()
        .tile(256, 0)
        .frame()
        .tile(288, 0)
        .frame()
        .tile(320, 0)
        .frame()
        .tile(352, 0)
        .frame()
        .tile(384, 0)
        .brush("gate")
        ////
        .file("content/assets/props.png")
        ////
        .color(RED)
        .splat(32, 0, 32, 32)
        .brush("cursor")
        ////
        .color(SADDLEBROWN)
        .tile(160, 64)
        .color(GREEN)
        .tile(192, 64)
        .brush("tree")
        ////
        .file("content/assets/walls.png")
        ////
        .color(LIGHTSLATEGRAY)
        .wall(0, 0, 32, 0)
        .brush("wall")
        ////
        .file("content/assets/blobs.png")
        ////
        .color(DARKGOLDENROD)
        .blob(0, 0, 0, 32, 0, 64)
        .brush("rock");
}

fn draw_frame(context: &mut backend::Context, offset: Point2D<f32>, frame: &Frame) {
    for splat in frame.iter() {
        context.draw_image(splat.image, offset - splat.offset, splat.color);
    }
}

fn init_terrain() {
    use world::terrain::{Form, Kind, Tile};

    // Void, terrain 0 is special.
    Tile::insert_resource(0, Tile::new("blank_floor", Kind::Block, Form::Void));
    // "Level exit", a visible portal tile.
    Tile::insert_resource(1, Tile::new("gate", Kind::Ground, Form::Gate));
    Tile::insert_resource(2, Tile::new("ground", Kind::Ground, Form::Floor));
    Tile::insert_resource(3, Tile::new("grass", Kind::Ground, Form::Floor));
    Tile::insert_resource(4, Tile::new("water", Kind::Water, Form::Floor));
    Tile::insert_resource(5, Tile::new("tree", Kind::Block, Form::Prop));
    Tile::insert_resource(6, Tile::new("wall", Kind::Block, Form::Wall));
    Tile::insert_resource(7, Tile::new("rock", Kind::Block, Form::Blob));
}

pub fn main() {
    // Construct display and Vitral context.
    let display = glutin::WindowBuilder::new()
                      .build_glium()
                      .unwrap();

    let mut backend = Backend::new(&display);

    let mut context: backend::Context;
    let mut builder = vitral::Builder::new();

    // Initialize game resources.
    init_brushes(&mut builder);
    init_terrain();

    context = builder.build(|img| backend.make_texture(&display, img));

    let font = context.default_font();

    // Initialize worldstate
    let mut view = GameView::new(World::new(1));

    view.world.terrain.set(Location::new(0, 0), 2);
    view.world.terrain.set(Location::new(0, -1), 1);
    view.world.terrain.set(Location::new(1, 0), 3);
    view.world.terrain.set(Location::new(2, 0), 5);
    view.world.terrain.set(Location::new(3, 0), 6);
    view.world.terrain.set(Location::new(4, 0), 6);
    view.world.terrain.set(Location::new(3, 2), 7);
    view.world.terrain.set(Location::new(4, 2), 7);

    // Run game.
    loop {
        context.begin_frame();

        let area = Rect::new(Point2D::new(0.0, 0.0), Size2D::new(640.0, 360.0));
        view.draw(&mut context, &area);

        context.draw_text(font,
                          Point2D::new(4.0, 20.0),
                          [1.0, 1.0, 1.0, 1.0],
                          "Hello, world!");

        if !backend.update(&display, &mut context) {
            return;
        }
    }
}
