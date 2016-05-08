use conrod::{Point, Scalar};

const PX_PER_EDGE : f64 = 20.0;
// const RES : f64 = 50;

#[derive(Clone)]
enum Phase {
    Outer,
    Inner
}

#[derive(Clone)]
pub struct ArcIter {
    inner_rad: Scalar,
    outer_rad: Scalar,
    start_angle: Scalar,
    end_angle: Scalar,
    center: Point,

    phase: Phase,
    cur_angle: f64,
    angle_incr: f64
}

impl ArcIter {
    pub fn new(center: Point, inner_rad: Scalar, outer_rad: Scalar, start_angle: Scalar, end_angle: Scalar) -> ArcIter {
        ArcIter {
            inner_rad: inner_rad,
            outer_rad: outer_rad,
            start_angle: start_angle,
            end_angle: end_angle,
            center: center,

            phase: Phase::Outer,
            cur_angle: start_angle,
            angle_incr: PX_PER_EDGE / outer_rad
        }
    }
}

impl Iterator for ArcIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        // TODO: allow proper wrapping angle calculation
        if self.cur_angle > self.end_angle {
            // time to reverse and start on the other side
            self.cur_angle = self.end_angle;
            self.angle_incr = -(PX_PER_EDGE / self.inner_rad);
            self.phase = Phase::Inner;
        }

        if self.cur_angle < self.start_angle {
            return None;
        }

        let r = match self.phase {
            Phase::Outer => {
                print!("o");
                self.outer_rad
            },
            Phase::Inner => {
                print!("i");
                self.inner_rad
            }
        };

        let pt : Point = [self.center[0] + (self.cur_angle).cos()*r, self.center[1] + (self.cur_angle).sin()*r];
        self.cur_angle += self.angle_incr;

        Some(pt)
    }
}
