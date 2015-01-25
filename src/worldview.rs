use std::iter::range_step;
use std::collections::HashMap;
use util::{V2, Rgb, timing};
use util::color::*;
use backend::{Canvas, CanvasUtil};
use world::TerrainType;
use world::{Location, Chart};
use world::{FovStatus};
use world::{Entity};
use world::{Light};
use viewutil::{chart_to_screen, cells_on_screen, level_z_to_view};
use viewutil::{FLOOR_Z, BLOCK_Z, DEPTH_Z_MODIFIER, PIXEL_UNIT};
use drawable::{Drawable};
use tilecache;
use tilecache::tile::*;

pub fn draw_world<C: Chart+Copy>(chart: &C, ctx: &mut Canvas, damage_timers: &HashMap<Entity, u32>) {
    // Draw stuff at most this deep.
    static MIN_DRAWN_DEPTH: i32 = -3;

    for depth in range_step(0, MIN_DRAWN_DEPTH, -1) {
        let mut hole_seen = false;
        for pt in cells_on_screen() {
            // Displace stuff deeper down to compensate for the projection
            // that shifts lower z-levels off-screen.
            let pt = pt + V2(depth, depth);
            let screen_pos = chart_to_screen(pt);
            let loc = *chart + pt;
            let depth_loc = Location { z: loc.z + depth as i8, ..loc };
            if !hole_seen && depth_loc.terrain().is_hole() { hole_seen = true; }
            // XXX: Grab FOV and light from zero z layer. Not sure what the
            // really right approach here is.
            let cell_drawable = CellDrawable::new(
                depth_loc, depth, loc.fov_status(), loc.light(), damage_timers);
            cell_drawable.draw(ctx, screen_pos);
        }
        // Don't draw the lower level unless there was at least one hole.
        if !hole_seen { return; }
    }
}

/// Drawable representation of a single map location.
pub struct CellDrawable<'a> {
    pub loc: Location,
    pub depth: i32,
    pub fov: Option<FovStatus>,
    pub light: Light,
    damage_timers: &'a HashMap<Entity, u32>,
}

impl<'a> Drawable for CellDrawable<'a> {
    fn draw(&self, ctx: &mut Canvas, offset: V2<f32>) {
        match self.fov {
            Some(_) => {
                self.draw_cell(ctx, offset)
            }
            None => {
                let (front_of_wall, is_door) = classify(self);
                if front_of_wall && !is_door {
                    self.draw_tile(ctx, CUBE, offset, BLOCK_Z, &BLACK);
                } else if !front_of_wall {
                    self.draw_tile(ctx, BLANK_FLOOR, offset, FLOOR_Z, &BLACK);
                }
            }
        }

        fn classify(c: &CellDrawable) -> (bool, bool) {
            let mut front_of_wall = false;
            let mut is_door = false;
            let nw = c.loc + V2(-1, 0);
            let ne = c.loc + V2(0, -1);

            for &p in vec![nw, ne].iter() {
                let t = p.terrain();
                if t.is_wall() {
                    front_of_wall = true;
                    if !t.blocks_walk() { is_door = true; }
                }
            }
            (front_of_wall, is_door)
        }
    }
}

impl<'a> CellDrawable<'a> {
    pub fn new(
        loc: Location,
        depth: i32,
        fov: Option<FovStatus>,
        light: Light,
        damage_timers: &'a HashMap<Entity, u32>) -> CellDrawable<'a> {
        CellDrawable {
            loc: loc,
            depth: depth,
            fov: fov,
            light: light,
            damage_timers: damage_timers,
        }
    }

    fn draw_tile(&'a self, ctx: &mut Canvas, idx: usize, offset: V2<f32>, z: f32, color: &Rgb) {
        self.draw_tile2(ctx, idx, offset, z, color, &BLACK);
    }

    /// Draw edge lines to floor tile if there are chasm tiles to the back.
    fn floor_edges(&'a self, ctx: &mut Canvas, offset: V2<f32>, color: &Rgb) {
        self.draw_tile(ctx, FLOOR_FRONT, offset, FLOOR_Z, color);

        // Shift edge offset from block top level to floor level.
        let offset = offset + V2(0, PIXEL_UNIT / 2).map(|x| x as f32);

        if (self.loc + V2(-1, -1)).terrain().is_hole() {
            self.draw_tile(ctx, BLOCK_N, offset, FLOOR_Z, color);
        }
        if (self.loc + V2(-1, 0)).terrain().is_hole() {
            self.draw_tile(ctx, BLOCK_NW, offset, FLOOR_Z, color);
        }
        if (self.loc + V2(0, -1)).terrain().is_hole() {
            self.draw_tile(ctx, BLOCK_NE, offset, FLOOR_Z, color);
        }
    }

    fn draw_floor(&'a self, ctx: &mut Canvas, idx: usize, offset: V2<f32>, color: &Rgb, edges: bool) {
        // Gray out the back color for lower-depth floor to highlight that
        // it's not real floor.
        let back_color = Rgb::new(
            0x10 * -self.depth as u8,
            0x10 * -self.depth as u8,
            0x10 * -self.depth as u8);
        self.draw_tile2(ctx, idx, offset, FLOOR_Z, color, &back_color);
        if edges {
            self.floor_edges(ctx, offset, color);
        }
    }

    fn draw_tile2(&'a self, ctx: &mut Canvas, idx: usize, offset: V2<f32>, z: f32,
                  color: &Rgb, back_color: &Rgb) {
        let map_color = if self.depth == 0 {
            Rgb::new(0x33, 0x22, 0x00) } else { Rgb::new(0x22, 0x11, 0x00) };

        let (mut color, mut back_color) = match self.fov {
            // XXX: Special case for the solid-black objects that are used to
            // block out stuff to not get recolored. Don't use total black as
            // an actual object color, have something like #010101 instead.
            Some(FovStatus::Remembered) if *color != BLACK => (BLACK, map_color),
            _ => (*color, *back_color),
        };
        if self.fov == Some(FovStatus::Seen) {
            color = self.light.apply(&color);
            back_color = self.light.apply(&back_color);
            if self.depth != 0 && color != BLACK {
                color = Rgb::new(
                    (color.r as f32 * 0.5) as u8,
                    (color.g as f32 * 0.5) as u8,
                    (color.b as f32 * 0.5) as u8);
            }
        }
        let z = z + self.depth as f32 * DEPTH_Z_MODIFIER;

        let offset = offset + level_z_to_view(self.depth).map(|x| x as f32);
        ctx.draw_image(tilecache::get(idx), offset, z, &color, &back_color);
    }

    fn draw_cell(&'a self, ctx: &mut Canvas, offset: V2<f32>) {
        if !self.loc.terrain().is_hole() {
            self.draw_terrain(ctx, offset);
        }

        if self.fov == Some(FovStatus::Seen) && self.depth == 0 {
            // Sort mobs on top of items for drawing.
            let mut es = self.loc.entities();
            es.sort_by(|a, b| a.is_mob().cmp(&b.is_mob()));
            for e in es.iter() {
                self.draw_entity(ctx, offset, e);
            }
        }
    }

    fn draw_terrain(&'a self, ctx: &mut Canvas, offset: V2<f32>) {
        let k = Kernel::new(|loc| loc.terrain(), self.loc);
        let front_of_hole = k.nw.is_hole() || k.n.is_hole() || k.ne.is_hole();

        match k.center {
            TerrainType::Void => {
                //self.draw_tile(ctx, BLANK_FLOOR, offset, FLOOR_Z, &BLACK);
            },
            TerrainType::Water => {
                self.draw_floor(ctx, WATER, offset, &ROYALBLUE, true);
            },
            TerrainType::Shallows => {
                self.draw_floor(ctx, SHALLOWS, offset, &CORNFLOWERBLUE, true);
            },
            TerrainType::Magma => {
                self.draw_tile2(ctx, MAGMA, offset, FLOOR_Z, &DARKRED, &YELLOW);
                self.floor_edges(ctx, offset, &YELLOW);
            },
            TerrainType::Tree => {
                // A two-toner, with floor, using two z-layers
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, TREE_TRUNK, offset, BLOCK_Z, &SADDLEBROWN);
                self.draw_tile(ctx, TREE_FOLIAGE, offset, BLOCK_Z, &GREEN);
            },
            TerrainType::Floor => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
            },
            TerrainType::Chasm => {
                self.draw_tile(ctx, CHASM, offset, FLOOR_Z, &DARKSLATEGRAY);
            },
            TerrainType::Grass => {
                self.draw_floor(ctx, FLOOR, offset, &DARKGREEN, true);
            },
            TerrainType::Grass2 => {
                self.draw_floor(ctx, GRASS, offset, &DARKGREEN, true);
            },
            TerrainType::Downstairs => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, DOWNSTAIRS, offset, BLOCK_Z, &SLATEGRAY);
            },
            TerrainType::Rock => {
                blockform(self, ctx, &k, offset, BLOCK, &DARKGOLDENROD);
            }
            TerrainType::Wall => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, WALL, &LIGHTSLATEGRAY, true);
            },
            TerrainType::RockWall => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, ROCKWALL, &LIGHTSLATEGRAY, true);
            },
            TerrainType::Fence => {
                // The floor type beneath the fence tile is visible, make it grass
                // if there's grass behind the fence. Otherwise make it regular
                // floor.
                if !front_of_hole {
                    if k.n == TerrainType::Grass || k.ne == TerrainType::Grass || k.nw == TerrainType::Grass {
                        self.draw_floor(ctx, GRASS, offset, &DARKGREEN, true);
                    } else {
                        self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                    }
                }
                wallform(self, ctx, &k, offset, FENCE, &DARKGOLDENROD, false);
            },
            TerrainType::Bars => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, BARS, &GAINSBORO, false);
            },
            TerrainType::Stalagmite => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, STALAGMITE, offset, BLOCK_Z, &DARKGOLDENROD);
            },
            TerrainType::Window => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, WINDOW, &LIGHTSLATEGRAY, false);
            },
            TerrainType::Door => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, DOOR, &LIGHTSLATEGRAY, true);
                wallform(self, ctx, &k, offset, DOOR + 4, &SADDLEBROWN, false);
            },
            TerrainType::OpenDoor => {
                if !front_of_hole { self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, false); }
                wallform(self, ctx, &k, offset, DOOR, &LIGHTSLATEGRAY, true);
            },
            TerrainType::Table => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, TABLE, offset, BLOCK_Z, &DARKGOLDENROD);
            },
            TerrainType::Fountain => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, FOUNTAIN, offset, BLOCK_Z, &GAINSBORO);
            },
            TerrainType::Altar => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, ALTAR, offset, BLOCK_Z, &GAINSBORO);
            },
            TerrainType::Barrel => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, BARREL, offset, BLOCK_Z, &DARKGOLDENROD);
            },
            TerrainType::Grave => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, GRAVE, offset, BLOCK_Z, &SLATEGRAY);
            },
            TerrainType::Stone => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, STONE, offset, BLOCK_Z, &SLATEGRAY);
            },
            TerrainType::Menhir => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, MENHIR, offset, BLOCK_Z, &SLATEGRAY);
            },
            TerrainType::DeadTree => {
                self.draw_floor(ctx, FLOOR, offset, &SLATEGRAY, true);
                self.draw_tile(ctx, TREE_TRUNK, offset, BLOCK_Z, &SADDLEBROWN);
            },
            TerrainType::TallGrass => {
                self.draw_tile(ctx, TALLGRASS, offset, BLOCK_Z, &GOLD);
            },
        }

        fn blockform(c: &CellDrawable, ctx: &mut Canvas, k: &Kernel<TerrainType>, mut offset: V2<f32>, idx: usize, color: &Rgb) {
            if c.depth != 0 {
                c.draw_tile(ctx, idx, offset, BLOCK_Z, color);
                // Double blockforms in sub-levels.
                offset = offset + V2(0, -PIXEL_UNIT/2).map(|x| x as f32);
            }
            c.draw_tile(ctx, BLOCK_DARK, offset, BLOCK_Z, &BLACK);
            c.draw_tile(ctx, idx, offset, BLOCK_Z, color);
            // Back lines for blocks with open floor behind them.
            if !k.nw.is_wall() {
                c.draw_tile(ctx, BLOCK_NW, offset, BLOCK_Z, color);
            }
            if !k.n.is_wall() {
                c.draw_tile(ctx, BLOCK_N, offset, BLOCK_Z, color);
            }
            if !k.ne.is_wall() {
                c.draw_tile(ctx, BLOCK_NE, offset, BLOCK_Z, color);
            }
        }

        fn wallform(c: &CellDrawable, ctx: &mut Canvas, k: &Kernel<TerrainType>, offset: V2<f32>, idx: usize, color: &Rgb, opaque: bool) {
            let (left_wall, right_wall, block) = wall_flags_lrb(k);
            if block {
                if opaque {
                    c.draw_tile(ctx, CUBE, offset, BLOCK_Z, &BLACK);
                } else {
                    c.draw_tile(ctx, idx + 2, offset, BLOCK_Z, color);
                    return;
                }
            }
            if left_wall && right_wall {
                c.draw_tile(ctx, idx + 2, offset, BLOCK_Z, color);
            } else if left_wall {
                c.draw_tile(ctx, idx, offset, BLOCK_Z, color);
            } else if right_wall {
                c.draw_tile(ctx, idx + 1, offset, BLOCK_Z, color);
            } else if !block || !k.s.is_wall() {
                // NB: This branch has some actual local kernel logic not
                // handled by wall_flags_lrb.
                let idx = if k.n.is_wall() && (!k.nw.is_wall() || !k.ne.is_wall()) {
                    // TODO: Walltile-specific XY-walls
                    XYWALL
                } else {
                    idx + 3
                };
                c.draw_tile(ctx, idx, offset, BLOCK_Z, color);
            }
        }

        // Return code:
        // (there is a wall piece to the left front of the tile,
        //  there is a wall piece to the right front of the tile,
        //  there is a solid block in the tile)
        fn wall_flags_lrb(k: &Kernel<TerrainType>) -> (bool, bool, bool) {
            if k.nw.is_wall() && k.n.is_wall() && k.ne.is_wall() {
                // If there is open space to east or west, even if this block
                // has adjacent walls to the southeast or the southwest, those
                // will be using thin wall sprites, so this block needs to have
                // the corresponding wall bit to make the wall line not have
                // gaps.
                (!k.w.is_wall() || !k.sw.is_wall(), !k.e.is_wall() || !k.se.is_wall(), true)
            } else {
                (k.nw.is_wall(), k.ne.is_wall(), false)
            }
        }
    }

    fn draw_entity(&'a self, ctx: &mut Canvas, offset: V2<f32>, entity: &Entity) {
        // SPECIAL CASE: The serpent mob has an extra mound element that
        // doesn't bob along with the main body.
        static SERPENT_ICON: usize = 94;

        let body_pos =
            if entity.is_bobbing() {
                offset + *(timing::cycle_anim(
                        0.3f64,
                        &[V2(0.0, 0.0), V2(0.0, -1.0)]))
            } else { offset };

        if let Some((icon, mut color)) = entity.get_icon() {
            // Damage blink animation.
            if let Some(&t) = self.damage_timers.get(entity) {
                color = if t % 2 == 0 { WHITE } else { BLACK };
            }

            if icon == SERPENT_ICON {
                // Body
                self.draw_tile(ctx, icon, body_pos, BLOCK_Z, &color);
                // Ground mound, doesn't bob.
                self.draw_tile(ctx, icon + 1, offset, BLOCK_Z, &color);
            } else {
                self.draw_tile(ctx, icon, body_pos, BLOCK_Z, &color);
            }
        }
    }
}

/// 3x3 grid of terrain cells. Use this as the input for terrain tile
/// computation, which will need to consider the immediate vicinity of cells.
struct Kernel<C> {
    n: C,
    ne: C,
    e: C,
    nw: C,
    center: C,
    se: C,
    w: C,
    sw: C,
    s: C,
}

impl<C: Clone> Kernel<C> {
    pub fn new<F>(get: F, loc: Location) -> Kernel<C>
        where F: Fn(Location) -> C {
        Kernel {
            n: get(loc + V2(-1, -1)),
            ne: get(loc + V2(0, -1)),
            e: get(loc + V2(1, -1)),
            nw: get(loc + V2(-1, 0)),
            center: get(loc),
            se: get(loc + V2(1, 0)),
            w: get(loc + V2(-1, 1)),
            sw: get(loc + V2(0, 1)),
            s: get(loc + V2(1, 1)),
        }
    }
}
