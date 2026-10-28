//! One uniform scale for the entire canvas, with inverse mapping for input.
use macroquad::prelude::{Camera2D, Rect, Vec2, vec2};

pub const WIDTH: f32 = 1440.0;
pub const HEIGHT: f32 = 900.0;

#[derive(Debug, Clone, Copy)]
pub struct DisplayFit {
    pub screen: Vec2,
    pub scale: f32,
    pub bounds: Rect,
}

impl DisplayFit {
    pub fn new(width: f32, height: f32) -> Self {
        let screen = vec2(width.max(1.0), height.max(1.0));
        let scale = (screen.x / WIDTH).min(screen.y / HEIGHT);
        let size = vec2(WIDTH, HEIGHT) * scale;
        Self {
            screen,
            scale,
            bounds: Rect::new(
                (screen.x - size.x) / 2.0,
                (screen.y - size.y) / 2.0,
                size.x,
                size.y,
            ),
        }
    }
    pub fn to_game(self, point: Vec2) -> Option<Vec2> {
        if point.x < self.bounds.x
            || point.y < self.bounds.y
            || point.x >= self.bounds.x + self.bounds.w
            || point.y >= self.bounds.y + self.bounds.h
        {
            return None;
        }
        Some((point - vec2(self.bounds.x, self.bounds.y)) / self.scale)
    }
    pub fn camera(self) -> Camera2D {
        Camera2D {
            target: vec2(WIDTH / 2.0, HEIGHT / 2.0),
            zoom: vec2(
                2.0 * self.scale / self.screen.x,
                2.0 * self.scale / self.screen.y,
            ),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fitting_never_stretches_or_crops() {
        for (w, h) in [
            (1440.0, 900.0),
            (1920.0, 1080.0),
            (2560.0, 1080.0),
            (1080.0, 1920.0),
            (720.0, 450.0),
            (3024.0, 1964.0),
        ] {
            let f = DisplayFit::new(w, h);
            assert!((f.bounds.w / f.bounds.h - WIDTH / HEIGHT).abs() < 0.00001);
            assert!(f.bounds.w <= w + 0.001 && f.bounds.h <= h + 0.001);
            assert!(f.bounds.x >= 0.0 && f.bounds.y >= 0.0);
            assert!((f.bounds.x * 2.0 + f.bounds.w - w).abs() < 0.001);
            assert!((f.bounds.y * 2.0 + f.bounds.h - h).abs() < 0.001);
        }
    }
    #[test]
    fn clicks_map_to_the_same_controls_at_every_size() {
        for (w, h) in [
            (1440.0, 900.0),
            (1920.0, 1080.0),
            (2560.0, 1080.0),
            (1080.0, 1920.0),
            (720.0, 450.0),
        ] {
            let f = DisplayFit::new(w, h);
            for p in [
                vec2(44.0, 164.0),
                vec2(756.0, 429.0),
                vec2(1340.0, 258.0),
                vec2(960.0, 390.0),
            ] {
                let screen = p * f.scale + vec2(f.bounds.x, f.bounds.y);
                assert!(f.to_game(screen).unwrap().distance(p) < 0.001);
            }
            assert!(f.to_game(vec2(f.bounds.x - 1.0, h / 2.0)).is_none());
            assert!(f.to_game(vec2(w / 2.0, f.bounds.y - 1.0)).is_none());
        }
    }
    #[test]
    fn minimized_windows_keep_a_finite_transform() {
        let f = DisplayFit::new(0.0, 0.0);
        assert!(f.scale.is_finite() && f.scale > 0.0);
    }
}
