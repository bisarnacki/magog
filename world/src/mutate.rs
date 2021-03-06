//! Gameplay logic that changes things

use crate::{
    ai::Brain,
    effect::{Damage, Effect},
    msg,
    sector::SECTOR_WIDTH,
    stats::Status,
    volume::Volume,
    Ability, ActionOutcome, Anim, AnimState, Ecs, ExternalEntity, Location, Slot, World,
};
use calx::Dir6;
use calx_ecs::Entity;
use rand::seq::SliceRandom;

/// World-mutating methods that are not exposed outside the crate.
impl World {
    /// Advance world state after player input has been received.
    pub(crate) fn next_tick(&mut self) {
        self.generate_world_spawns();
        self.tick_anims();

        self.ai_main();

        self.clean_dead();
        self.flags.tick += 1;

        // Expiring entities (animation effects) disappear if their time is up.
        let es: Vec<Entity> = self.ecs.anim.ent_iter().cloned().collect();
        for e in es.into_iter() {
            if let Some(anim) = self.anim(e) {
                if anim
                    .anim_done_world_tick
                    .map(|t| t <= self.get_tick())
                    .unwrap_or(false)
                {
                    self.kill_entity(e);
                }
            }
        }
    }

    pub(crate) fn equip_item(&mut self, e: Entity, parent: Entity, slot: Slot) {
        self.spatial.equip(e, parent, slot);
        self.rebuild_stats(parent);
    }

    pub(crate) fn set_player(&mut self, player: Option<Entity>) { self.flags.player = player; }

    /// Compute field-of-view into entity's map memory.
    ///
    /// Does nothing for entities without a map memory component.
    pub(crate) fn do_fov(&mut self, e: Entity) {
        if !self.ecs.map_memory.contains(e) {
            return;
        }

        if let Some(origin) = self.location(e) {
            const DEFAULT_FOV_RANGE: i32 = 7;
            const OVERLAND_FOV_RANGE: i32 = SECTOR_WIDTH;

            // Long-range sight while in overworld.
            let range = if self.is_underground(origin) {
                DEFAULT_FOV_RANGE
            } else {
                OVERLAND_FOV_RANGE
            };

            let fov = self.fov_from(origin, range);

            let memory = &mut self.ecs.map_memory[e];
            memory.seen.clear();

            for &loc in &fov {
                memory.seen.insert(loc);
                memory.remembered.insert(loc);
            }
        }
    }

    /// Access the persistent random number generator.
    pub(crate) fn rng(&mut self) -> &mut crate::Rng { &mut self.rng }

    /// Mutable access to ecs
    pub(crate) fn ecs_mut(&mut self) -> &mut Ecs { &mut self.ecs }

    /// Spawn an effect entity
    pub(crate) fn spawn_fx(&mut self, loc: Location, state: AnimState) -> Entity {
        let e = self.ecs.make();
        self.place_entity(e, loc);

        let mut anim = Anim::default();
        debug_assert!(state.is_transient_anim_state());
        anim.state = state;
        anim.anim_start = self.get_anim_tick();

        // Set the (world clock, not anim clock to preserve determinism) time when animation entity
        // should be cleaned up.
        // XXX: Animations stick around for a bunch of time after becoming spent and invisible,
        // simpler than trying to figure out precise durations.
        anim.anim_done_world_tick = Some(self.get_tick() + 300);

        self.ecs.anim.insert(e, anim);
        e
    }

    /// Remove destroyed entities from system
    pub(crate) fn clean_dead(&mut self) {
        let kill_list: Vec<Entity> = self
            .entities()
            .filter(|&&e| !self.is_alive(e))
            .cloned()
            .collect();

        for e in kill_list {
            self.remove_entity(e);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // High-level commands, actual action can change because of eg. confusion.

    pub(crate) fn entity_melee(&mut self, e: Entity, dir: Dir6) -> ActionOutcome {
        if self.confused_move(e) {
            Some(true)
        } else {
            self.really_melee(e, dir)
        }
    }

    /// The entity spends its action waiting.
    pub(crate) fn idle(&mut self, e: Entity) -> ActionOutcome {
        if self.consume_nutrition(e) {
            if let Some(_regen) = self.tick_regeneration(e) {
                // TODO: animate/message the healing.
            }
        }
        self.end_turn(e);
        Some(true)
    }

    ////////////////////////////////////////////////////////////////////////////////

    pub(crate) fn spawn(&mut self, entity: &ExternalEntity, loc: Location) -> Entity {
        let e = self.inject(entity);
        self.place_entity(e, loc);
        e
    }

    /// Special method for setting the player start position.
    ///
    /// If player already exists and is placed in the world, do nothing.
    ///
    /// If player exists, but is not placed in spatial, teleport the player here.
    ///
    /// If player does not exist, create the initial player entity.
    pub(crate) fn spawn_player(&mut self, loc: Location, spec: &ExternalEntity) {
        if let Some(player) = self.player() {
            if self.location(player).is_none() {
                // Teleport player from limbo.
                self.place_entity(player, loc);
            }
        } else {
            let player = self.inject(&spec);
            // Playerify with the boring component stuff.
            self.ecs_mut().brain.insert(player, Brain::player());
            self.ecs_mut().map_memory.insert(player, Default::default());
            self.set_player(Some(player));
            self.place_entity(player, loc);
        }
    }

    pub(crate) fn apply_effect_to_entity(
        &mut self,
        effect: &Effect,
        target: Entity,
        source: Option<Entity>,
    ) {
        use crate::effect::Effect::*;
        match *effect {
            Hit { amount, damage } => {
                self.damage(target, amount as i32, damage, source);
            }
            Confuse => {
                self.gain_status(target, Status::Confused, 40);
                msg!("[One] [is] confused."; self.subject(target));
            }
        }
    }

    pub(crate) fn apply_effect_to(
        &mut self,
        effect: &Effect,
        loc: Location,
        source: Option<Entity>,
    ) {
        if let Some(mob) = self.mob_at(loc) {
            self.apply_effect_to_entity(effect, mob, source);
        }
    }

    pub(crate) fn apply_effect(
        &mut self,
        effect: &Effect,
        volume: &Volume,
        source: Option<Entity>,
    ) {
        for loc in &volume.0 {
            self.apply_effect_to(effect, *loc, source);
        }
    }

    /// Run autonomous updates on entity that happen each turn
    ///
    /// This runs regardless of the action speed or awakeness status of the entity. The exact same
    /// is run for player and AI entities.
    pub(crate) fn heartbeat(&mut self, e: Entity) { self.tick_statuses(e); }

    pub(crate) fn use_ability(&mut self, _e: Entity, _a: Ability) -> ActionOutcome {
        // TODO
        None
    }

    pub(crate) fn use_item_ability(
        &mut self,
        e: Entity,
        item: Entity,
        a: Ability,
    ) -> ActionOutcome {
        debug_assert!(!a.is_targeted());
        // TODO: Lift to generic ability use method
        if !self.has_ability(item, a) {
            return None;
        }
        let origin = self.location(e)?;

        match a {
            Ability::LightningBolt => {
                const LIGHTNING_RANGE: u32 = 4;
                const LIGHTNING_EFFECT: Effect = Effect::Hit {
                    amount: 12,
                    damage: Damage::Electricity,
                };

                // TODO: Make an API, more efficient lookup of entities within an area

                let targets: Vec<Entity> = self
                    .sphere_volume(origin, LIGHTNING_RANGE)
                    .0
                    .into_iter()
                    .flat_map(|loc| self.entities_at(loc))
                    .filter(|&x| self.is_mob(x) && x != e)
                    .collect();

                if let Some(target) = targets.choose(self.rng()) {
                    msg!("There is a peal of thunder.");
                    let loc = self.location(*target).unwrap();
                    self.apply_effect(&LIGHTNING_EFFECT, &Volume::point(loc), Some(e));
                } else {
                    msg!("The spell fizzles.");
                }
            }
            _ => {
                msg!("TODO cast untargeted spell {:?}", a);
            }
        }
        self.drain_charge(item);
        Some(true)
    }

    pub(crate) fn use_targeted_ability(
        &mut self,
        _e: Entity,
        _a: Ability,
        _dir: Dir6,
    ) -> ActionOutcome {
        // TODO
        None
    }

    pub(crate) fn use_targeted_item_ability(
        &mut self,
        e: Entity,
        item: Entity,
        a: Ability,
        dir: Dir6,
    ) -> ActionOutcome {
        debug_assert!(a.is_targeted());
        if !self.has_ability(item, a) {
            return None;
        }
        let origin = self.location(e)?;

        // TODO: Lift to generic ability use method

        match a {
            Ability::Fireball => {
                const FIREBALL_RANGE: u32 = 9;
                const FIREBALL_RADIUS: u32 = 1;
                const FIREBALL_EFFECT: Effect = Effect::Hit {
                    amount: 6,
                    damage: Damage::Fire,
                };
                let center = self.projected_explosion_center(origin, dir, FIREBALL_RANGE);
                let volume = self.sphere_volume(center, FIREBALL_RADIUS);
                self.apply_effect(&FIREBALL_EFFECT, &volume, Some(e));

                // TODO: Maybe move anim generation to own procedure?
                const PROJECTILE_TIME: u64 = 8;
                for &pt in &volume.0 {
                    let fx = self.spawn_fx(pt, AnimState::Explosion);
                    self.anim_mut(fx).unwrap().anim_start += PROJECTILE_TIME;
                }

                let anim_tick = self.get_anim_tick();
                let projectile = self.spawn_fx(center, AnimState::Firespell);
                {
                    let anim = self.anim_mut(projectile).unwrap();
                    anim.tween_from = origin;
                    anim.tween_start = anim_tick;
                    anim.tween_duration = PROJECTILE_TIME as u32;
                }
            }
            Ability::Confuse => {
                const CONFUSION_RANGE: u32 = 9;

                let center = self.projected_explosion_center(origin, dir, CONFUSION_RANGE);
                self.apply_effect(&Effect::Confuse, &Volume::point(center), Some(e));
            }
            _ => {
                msg!("TODO cast directed spell {:?}", a);
            }
        }
        self.drain_charge(item);
        Some(true)
    }
}
