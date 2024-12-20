use core::ops::RangeInclusive;

use glam::{ivec2, uvec2, UVec2};

#[derive(Debug, Clone)]
pub struct Circle {
    dx: i32,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Circle {
    #[inline]
    pub fn new(radius: i32) -> Self {
        let mut x = -radius..=radius;

        // Note: an inclusive range contains at least one item.
        let dx = x.next().unwrap();

        let y_range = x.end() - dx.abs();
        let y = -y_range..=y_range;

        Self { dx, x, y }
    }

    #[inline]
    pub fn with_center(self, center: UVec2) -> impl Iterator<Item = UVec2> + Clone {
        self.filter_map(move |p| {
            let x = center.x.checked_add_signed(p.x)?;
            let y = center.y.checked_add_signed(p.y)?;
            Some(uvec2(x, y))
        })
    }
}

impl Iterator for Circle {
    type Item = glam::IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.y.next() {
            Some(y) => Some(ivec2(self.dx, y)),
            None => {
                self.dx = self.x.next()?;

                let y_range = self.x.end() - self.dx.abs();
                self.y = -y_range..=y_range;

                // Note: an inclusive range contains at least one item.
                let y = self.y.next().unwrap();

                Some(ivec2(self.dx, y))
            }
        }
    }
}
