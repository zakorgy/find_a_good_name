use cgmath::Vector2;
use cgmath::InnerSpace;

pub struct MovingComponent {
    position: Vector2<f32>,
    old_pos: Vector2<f32>,
    velocity: Vector2<f32>,
    pub force: Vector2<f32>,
    mass: f32,
    max_speed: f32,
    max_force: f32,
    // heading: Vector2<f32>,
    // side:  Vector2<f32>,
    // max_turn_rate: f32,
}

impl MovingComponent {
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
            force: (0., 0.).into(),
            mass,
            max_speed,
            max_force,
        }
    }

    pub fn update(&mut self, forces: &[Vector2<f32>], reset: bool) {
        let steering_force = self.calculate(forces);
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
    }

    fn calculate(&self, forces: &[Vector2<f32>]) -> Vector2<f32> {
        let mut steering_force = (0., 0.).into();
        if self.force.magnitude2() > 0.00001 {
            self.accumulate_force(&mut steering_force, self.force);
        }
        for force in forces {
            self.accumulate_force(&mut steering_force, *force);
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
