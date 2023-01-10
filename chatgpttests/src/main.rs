extern crate gnuplot;
use gnuplot::{Figure, Color};
use nalgebra::Vector2;

struct Particle {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    mass: f64,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Particle {
        Particle {
            position: Vector2::new(x, y),
            velocity: Vector2::new(vx, vy),
            mass: mass,
        }
    }

    fn acceleration(&self, other: &Particle, g: f64) -> Vector2<f64> {
        let mut acc = self.position - other.position;
        let r = acc.magnitude();
        acc = acc * g * other.mass / (r * r + 1.0);
        acc
    }

    fn update_position(&mut self, other: &mut Particle, g: f64, dt: f64) {
        let k1 = self.acceleration(other, g) * dt;
        let l1 = other.acceleration(self, g) * dt;
        let k2 = (self.acceleration(other, g) + k1 / 2.0) * dt;
        let l2 = (other.acceleration(self, g) + l1 / 2.0) * dt;
        let k3 = (self.acceleration(other, g) + k2 / 2.0) * dt;
        let l3 = (other.acceleration(self, g) + l2 / 2.0) * dt;
        let k4 = (self.acceleration(other, g) + k3) * dt;
        let l4 = (other.acceleration(self, g) + l3) * dt;

        self.position = self.position + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        other.position = other.position + (l1 + 2.0 * l2 + 2.0 * l3 + l4) / 6.0;
    }
}

fn simulate(mut p1: Particle, mut p2: Particle, g: f64, dt: f64, num_steps: usize) -> Vec<(Vector2<f64>, Vector2<f64>)> {
    let mut positions = Vec::new();
    for _ in 0..num_steps {
        p1.update_position(&mut p2, g, dt);
        positions.push((p1.position, p2.position));
    }
    positions
}

fn main() {
    let g = 1.0;
    let dt = 0.1;
    let num_steps = 1000;

    let p1 = Particle::new(0.0, 0.0, 0.0, 0.0, 1.0);
    let p2 = Particle::new(1.0, 0.0, 0.0, 1.0, 1.0);

    let positions = simulate(p1, p2, g, dt, num_steps);

    let mut fg = Figure::new();
    let mut plot = fg.axes2d();

    let p1_positions = positions.iter().map(|(p1, _)| p1).collect::<Vec<_>>();
    let p2_positions = positions.iter().map(|(_, p2)| p2).collect::<Vec<_>>();
    plot.lines(
        p1_positions.iter().map(|p| p.x),
        p1_positions.iter().map(|p| p.y),
        &[Color("red")],
    );
    plot.lines(
        p2_positions.iter().map(|p| p.x),
        p2_positions.iter().map(|p| p.y),
        &[Color("blue")],
    );
    fg.show();
}
