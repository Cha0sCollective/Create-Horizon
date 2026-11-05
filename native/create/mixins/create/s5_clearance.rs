// Copyright (c) horizon-rust contributors
// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// Bounded S5 adapter inside game::piston_push, reusing its actual body,
// collision, movement-budget and apply paths. No public collision provider.
use super::*;

pub(crate) struct S5PushPlan {
    moves: Vec<(Target, Vec3, Vec3, Vec3)>,
}

impl Game<'_> {
    /// A pending failed-write recovery may need to put a solid block back in
    /// its original cell. Refuse that retry while any authoritative body is
    /// inside the full cube; the owner retains the item and retries later.
    pub(crate) fn create_s5_body_clear(&mut self, pos: BlockPos, bounds: Aabb) -> bool {
        let steps = self.push_step_heights();
        for target in self.push_targets() {
            if self
                .push_body(target, pos, &steps)
                .is_some_and(|body| bounds.intersects(body.bounds))
            {
                return false;
            }
        }
        true
    }

    /// Plan every body's movement before changing any body. A blocked escape
    /// rejects the carrier advance and leaves body positions/budgets unchanged.
    /// Only the packet's inert full-cube, horizontal one-cell carrier calls this.
    pub(crate) fn create_s5_plan_push(
        &mut self,
        pos: BlockPos,
        piston: MovingPiston,
        progress: f32,
        final_bounds: Aabb,
    ) -> Option<S5PushPlan> {
        let cube = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        self.create_s5_plan_push_shapes(pos, piston, progress, &[cube], &[final_bounds])
    }

    /// The finite actuator moves a head and at most two declared cargo blocks.
    /// The fourth final bound is the returning pole's original cell. Plan the
    /// complete finite shape against each body with one movement budget/commit.
    pub(crate) fn create_s5_plan_push_shapes(
        &mut self,
        pos: BlockPos,
        piston: MovingPiston,
        progress: f32,
        boxes: &[Aabb],
        final_bounds: &[Aabb],
    ) -> Option<S5PushPlan> {
        if boxes.is_empty() || final_bounds.is_empty() || boxes.len() > 3 || final_bounds.len() > 4
        {
            return None;
        }
        let delta = progress - piston.progress;
        if delta < 0.0 || !delta.is_finite() {
            return None;
        }
        let direction = piston.movement_direction();
        let steps = self.push_step_heights();
        let mut plans = Vec::new();
        for target in self.push_targets() {
            let Some(body) = self.push_body(target, pos, &steps) else {
                continue;
            };
            let amount = if delta > 0.0 {
                piston_push::collided_amount(
                    boxes,
                    pos,
                    piston.shape_state(),
                    progress,
                    direction,
                    body.bounds,
                )
            } else {
                0.0
            };
            if amount <= 0.0 {
                if final_bounds
                    .iter()
                    .any(|bound| bound.intersects(body.bounds))
                {
                    return None;
                }
                continue;
            }
            let requested = piston_push::step_vector(direction).scale(amount);
            // Copy the same per-body budget the ordinary push would use. The
            // real budget is changed only after all destinations are safe.
            let mut budget = match target {
                Target::Player => self.player.as_ref().map(|p| p.piston),
                Target::Guest(conn) => self.lan.as_ref().and_then(|host| {
                    host.hub
                        .guests
                        .iter()
                        .find(|g| g.conn == conn)
                        .and_then(|g| g.motion.as_deref())
                        .map(|motion| motion.piston)
                }),
                Target::Entity(EntityHandle::Mob(index)) => Some(self.mobs[index].piston),
                _ => Some(PistonMovement::default()),
            };
            let Some(ref mut budget) = budget else {
                return None;
            };
            let Ok(limited) = budget.limit(requested, || Ok::<i64, ()>(self.world.game_time))
            else {
                return None;
            };
            let resolved = with_piston_noclip(direction, || {
                pushed_movement(
                    SceneCollisions {
                        world: &self.world,
                        binding: self.collision,
                        subject: None,
                    },
                    self.border.geometry(),
                    [body.position.x, body.position.z],
                    body.bounds,
                    limited,
                    body.step_height,
                    body.on_ground,
                )
            });
            let Ok(resolved) = resolved else {
                return None;
            };
            if final_bounds
                .iter()
                .any(|bound| bound.intersects(body.bounds.move_by(resolved)))
            {
                return None;
            }
            plans.push((target, requested, limited, resolved));
        }
        Some(S5PushPlan { moves: plans })
    }

    /// Caller accepts the carrier pose/materialization before this infallible
    /// commit, with no tick/await or body mutation between planning and commit.
    pub(crate) fn create_s5_commit_push(&mut self, plan: S5PushPlan) {
        let mut accumulators = std::collections::HashMap::new();
        for (target, requested, limited, resolved) in plan.moves {
            let actual = self.limit_piston(target, requested, &mut accumulators);
            debug_assert_eq!(actual, Some(limited));
            if let Target::Entity(EntityHandle::ArmorStand(index)) = target {
                self.armor_stands[index].on_ground = limited.y < 0.0 && limited.y != resolved.y;
            }
            self.apply_push(target, resolved);
            // The derived carrier is not an independently ticking client
            // piston. Use the existing authoritative player correction path.
            if let Target::Guest(conn) = target {
                if let Some(host) = self.lan.as_mut() {
                    if let Some(index) = host.hub.guests.iter().position(|g| g.conn == conn) {
                        let position = host.hub.guests[index].position;
                        host.hub.teleport_keeping_motion(index, position);
                    }
                }
            }
        }
    }
}
