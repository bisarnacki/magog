use calx_ecs::Entity;
use calx_color::color::*;
use calx_color::Rgba;
use calx_resource::Resource;
use stats::{Intrinsic, Stats};
use stats::Intrinsic::*;
use components::{Brain, Desc, Health, MapMemory};
use world::{Component, Loadout, World};
use mutate::Mutate;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Forms {
    Player,
    Dreg,
    Snake,
    Ooze,
}

/// Forms are the prototypes for the entities you create.
#[derive(Clone, Debug)]
pub struct Form {
    /// Spawn probability weight is the inverse of rarity.
    ///
    /// Rarity zero is a special case that gets zero weight.
    pub rarity: f32,
    /// Minimum depth where the entity will show up.
    ///
    /// More powerful entities only start showing up in large depths.
    pub min_depth: i32,
    //    /// Type of thing.
    //    pub category: FormType,
    /// Actual components to set up the thing.
    pub loadout: Loadout,
}

impl Form {
    pub fn named(name: &str) -> Option<&'static Form> {
        FORMS.iter().find(|x| x.name() == Some(name))
    }

    /// Create a standard form for a living creature.
    pub fn mob(name: &str, brush: &str, power: i32, intrinsics: &[Intrinsic]) -> Form {
        Form {
            rarity: 1.0,
            min_depth: 0,
            // category: FormType::Creature,
            loadout: Loadout::new()
                         .c(Stats::new(power, intrinsics))
                         .c(Desc::new(name, brush))
                         .c(Brain::enemy())
                         .c(Health::new()),
        }
    }

    pub fn rarity(mut self, rarity: f32) -> Form {
        self.rarity = rarity;
        self
    }

    pub fn depth(mut self, min_depth: i32) -> Form {
        self.min_depth = min_depth;
        self
    }

    pub fn player(mut self) -> Form {
        self.loadout.brain = Some(Brain::player());
        self
    }

    /// Return the name of the form if it has one.
    pub fn name<'a>(&'a self) -> Option<&'a str> {
        match self.loadout.desc {
            Some(ref desc) => Some(&desc.name[..]),
            None => None
        }
    }

    pub fn c<C: Component>(mut self, comp: C) -> Form {
        self.loadout = self.loadout.c(comp);
        self
    }
}

lazy_static! {
    pub static ref FORMS: Vec<Form> =
        vec![
        Form::mob("player",     "player",     10, &[Hands]).rarity(0.0).player()
            .c(MapMemory::new()),
        Form::mob("dreg",       "dreg",       1,  &[Hands]),
        Form::mob("snake",      "snake",      1,  &[]),
        ];
}
