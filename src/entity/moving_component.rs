use cgmath::Vector2;
use cgmath::InnerSpace;
use crate::game::FRAMES_PER_SEC;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct Force {
    pub force: Vector2<f32>,
    depleet_time: Cell<i32>,
    magnitude2: f32,
}

impl Force {
    pub fn new(force: Vector2<f32>, depleet_time: i32) -> Self {
        Force {
            force,
            depleet_time: Cell::new(depleet_time),
            magnitude2: force.magnitude2(),
        }
    }

    pub fn update(&self) -> bool {
        if self.depleeted() {
            return true
        }
        self.depleet_time.set(self.depleet_time.get() - 1);
        false
    }

    pub fn has_magnitude(&self) -> bool {
        !self.depleeted() && self.magnitude2 > 0.0001
    }

    pub fn depleeted(&self) -> bool {
        self.depleet_time.get() == 0
    }
}

pub struct MovingComponent {
    position: Vector2<f32>,
    old_pos: Vector2<f32>,
    velocity: Vector2<f32>,
    thrust: Force,
    forces: Vec<Force>,
    mass: f32,
    max_speed: f32,
    max_force: f32,
    // heading: Vector2<f32>,
    // side:  Vector2<f32>,
    // max_turn_rate: f32,
}

impl MovingComponent {
    pub fn thrust(&self) -> &Force {
        &self.thrust
    }

    pub fn set_thrust(&mut self, thrust: Force) {
        self.thrust = thrust;
    }

    pub fn pos(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_pos(&mut self, pos: Vector2<f32>) {
        self.position = pos;
    }

    pub fn reset_pos(&mut self) {
        self.position = self.old_pos;
    }

    pub fn new(mass: f32, max_speed: f32, max_force: f32) -> Self {
        MovingComponent {
            old_pos: (0., 0.).into(),
            position: (0., 0.).into(),
            velocity: (0., 0.).into(),
            thrust: Force::new((0., 0.).into(), 0),
            forces: vec![],
            mass,
            max_speed,
            max_force,
        }
    }

    pub fn update(&mut self, forces: &[Force], reset: bool) -> bool {
        self.forces.extend_from_slice(forces);
        self.thrust.update();
        self.forces.retain(|force| {
            let delete = force.update();
            !delete
        });
        let steering_force = self.calculate();
        let acceleration = steering_force / self.mass;
        if reset {
            self.velocity = (0., 0.).into();
        }
        self.velocity += acceleration;
        if self.velocity.magnitude() > self.max_speed {
            self.velocity.normalize_to(self.max_speed);
        }
        self.old_pos = self.position;
        self.position += self.velocity;
        return acceleration.magnitude2() > 0.000001
    }

    fn calculate(&self) -> Vector2<f32> {
        let mut steering_force = (0., 0.).into();
        if !self.thrust.depleeted() && self.thrust.magnitude2 > 0.000001 {
            self.accumulate_force(&mut steering_force, self.thrust.force);
        }
        for force in &self.forces {
            self.accumulate_force(&mut steering_force, force.force);
        }
        steering_force
    }

    fn accumulate_force(&self, running_total: &mut Vector2<f32>, force_to_add: Vector2<f32>) -> bool {
        let magnitude_so_far = running_total.magnitude();

        let magnitude_remaining = self.max_force - magnitude_so_far;

        if magnitude_remaining <= 0. {
            return false
        }

        let magnitude_to_add = force_to_add.magnitude();

        if magnitude_to_add < magnitude_so_far {
            *running_total += force_to_add;
        } else {
            *running_total += force_to_add.normalize_to(magnitude_remaining);
        }
        true
    }
}
