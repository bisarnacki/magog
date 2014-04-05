#![feature(globs)]

extern crate cgmath;
extern crate glutil;
extern crate color;
extern crate calx;
extern crate stb;
extern crate collections;
extern crate serialize;
extern crate num;
extern crate rand;
extern crate world;
extern crate toml;

use std::io::{IoResult, File, Open, Write, BufferedReader, BufferedWriter};
use std::path::Path;
use glutil::glrenderer::GlRenderer;
use color::rgb::consts::*;
use cgmath::point::{Point2};
use cgmath::vector::{Vec2};
use calx::app::App;
use calx::key;
use calx::renderer::Renderer;
use calx::renderer;
use world::area::{Area, Location};
use world::area;
use world::transform::Transform;
use world::fov;
use world::mob::Mob;
use world::state;
use world::areaview;
use world::areaview::Kernel;
use world::sprite;

static VERSION: &'static str = include!("../../gen/git_version.inc");

pub struct State {
    area: ~Area,
    pos: Location,
}

impl state::State for State {
    fn transform(&self) -> Transform { Transform::new(self.pos) }
    fn fov(&self, _loc: Location) -> fov::FovStatus { fov::Seen }
    fn drawable_mob_at<'a>(&'a self, _loc: Location) -> Option<&'a Mob> { None }
    fn area<'a>(&'a self) -> &'a Area { &*self.area }
}

impl State {
    pub fn new() -> State {
        State {
            area: ~Area::new(area::Void),
            pos: Location::new(0i8, 0i8),
        }
    }

    pub fn from_file(path: &str) -> Option<State> {
        let mut rd = BufferedReader::new(File::open(&Path::new(path)));
        let toml_value: toml::Value = match toml::parse_from_buffer(&mut rd) {
            Ok(v) => v,
            Err(e) => { println!("Toml parse error {}", e.to_str()); return None; }
        };
        let ascii_map = match toml::from_toml(toml_value) {
            Ok(s) => s,
            Err(e) => { println!("Toml decode error {}", e.to_str()); return None; }
        };
        Some(State {
            area: ~Area::from_ascii_map(&ascii_map),
            pos: Location::new(0i8, 0i8),
        })
    }

    pub fn save(&self, path: &str) -> IoResult<()> {
        let file = File::open_mode(&Path::new(path), Open, Write).unwrap();
        let mut wr = BufferedWriter::new(file);
        let obj = self.area.build_asciimap();
        try!(writeln!(&mut wr, "{}", obj.to_str()));
        Ok(())
    }
}

pub fn main() {
    let mut app : App<GlRenderer> = App::new(640, 360, format!("Map editor ({})", VERSION));
    areaview::init_tiles(&mut app);

    let mut state = match State::from_file("map.txt") {
        Some(s) => s,
        None => State::new()
    };

    let mut brush = 0;

    state.area.set(Location::new(0i8, 0i8), area::Floor);

    while app.alive {
        loop {
            match app.r.pop_key() {
                Some(key) => {
                    match key.code {
                        key::ESC => { app.quit(); }
                        key::F12 => { app.r.screenshot("/tmp/shot.png"); }
                        key::NUM_1 => { brush += area::TERRAINS.len() - 1; brush %= area::TERRAINS.len(); }
                        key::NUM_2 => { brush += 1; brush %= area::TERRAINS.len(); }
                        key::UP => { state.pos = state.pos + Vec2::new(-1, -1); }
                        key::DOWN => { state.pos = state.pos + Vec2::new(1, 1); }
                        key::LEFT => { state.pos = state.pos + Vec2::new(-1, 1); }
                        key::RIGHT => { state.pos = state.pos + Vec2::new(1, -1); }
                        _ => (),
                    }
                }
                _ => { break; }
            }
        }

        areaview::draw_area(&state, &mut app);

        for spr in
            areaview::terrain_sprites(
                &Kernel::new_default(area::TERRAINS[brush], area::Void),
                &Point2::new(32f32, 32f32)).iter() {
            spr.draw(&mut app);
        }

        let mouse = app.r.get_mouse();
        let xf = Transform::new(state.pos);
        let cursor_chart_loc = xf.to_chart(&mouse.pos);
        app.r.draw_tile(areaview::CURSOR_BOTTOM, &xf.to_screen(cursor_chart_loc), sprite::FLOOR_Z, &FIREBRICK, renderer::ColorKeyDraw);
        app.r.draw_tile(areaview::CURSOR_TOP, &xf.to_screen(cursor_chart_loc), sprite::BLOCK_Z, &FIREBRICK, renderer::ColorKeyDraw);
        if mouse.left {
            state.area.set(cursor_chart_loc, area::TERRAINS[brush]);
        }
        if mouse.right {
            state.area.set(cursor_chart_loc, state.area.default);
        }

        app.r.flush();
    }
    state.save("map.txt");
}
