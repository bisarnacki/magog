#![crate_name="world"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(tuple_indexing)]
#![feature(if_let)]
#![comment = "Display independent world logic for Magog"]

extern crate num;
extern crate rand;
extern crate serialize;
extern crate calx;

pub use entity::{Entity};
pub use flags::{camera, set_camera, get_tick};
pub use geom::{HexGeom};
pub use location::{Location, Chart, Unchart};
pub use msg::{pop_msg};
pub use world::{init_world, load, save};
pub use fov::{Fov};

pub mod action;
pub mod dir6;
pub mod mob;
pub mod terrain;

mod area;
mod comp;
mod entity;
mod ecs;
mod egg;
mod flags;
mod fov;
mod geom;
mod geomorph;
mod geomorph_data;
mod location;
mod mapgen;
mod map_memory;
mod msg;
mod rng;
mod spatial;
mod world;

#[deriving(Eq, PartialEq, Show)]
pub enum FovStatus {
    Seen,
    Remembered,
}

/// General type of a game entity.
#[deriving(Eq, PartialEq, Clone, Show, Encodable, Decodable)]
pub enum EntityKind {
    /// An active, mobile entity like the player or the NPCs.
    MobKind(mob::MobType),
    /// An entity that can be picked up and used in some way.
    ItemKind, // TODO ItemType data.
    /// A background item that doesn't do much.
    PropKind,
    /// A static object that does things when stepped on.
    NodeKind,
}

/// Landscape type. Also serves as bit field in order to produce habitat masks
/// for entity spawning etc.
#[deriving(Eq, PartialEq, Clone, Show, Encodable, Decodable)]
pub enum Biome {
    Overland = 0b1,
    Dungeon  = 0b10,

    // For things showing up at a biome.
    Anywhere = 0b11111111,
}

impl Biome {
    pub fn default_terrain(self) -> terrain::TerrainType {
        match self {
            Overland => terrain::Tree,
            Dungeon => terrain::Rock,
            _ => terrain::Void,
        }
    }
}

#[deriving(Eq, PartialEq, Show, Clone, Encodable, Decodable)]
pub struct AreaSpec {
    pub biome: Biome,
    pub depth: int,
}

impl AreaSpec {
    pub fn new(biome: Biome, depth: int) -> AreaSpec {
        AreaSpec { biome: biome, depth: depth }
    }

    /// Return whether a thing with this spec can be spawned in an environment
    /// with the given spec.
    pub fn can_hatch_in(&self, environment: &AreaSpec) -> bool {
        self.depth >= 0 && self.depth <= environment.depth &&
        (self.biome as int & environment.biome as int) != 0
    }
}

/// Various one-off signals the game sends to the UI layer.
#[deriving(Clone, Show)]
pub enum Msg {
    Text(String),
    // TODO: Type of effect.
    Explosion(Location),
    Damage(Entity),
    Gib(Location),
}