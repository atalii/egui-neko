// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use egui::{self, NumExt, Pos2, Rect, Vec2, widgets};

mod images;
use images::*;

/// This struct is used as a handle on a cat that may be drawn to follow a cursor. See examples for
/// usage.
pub struct Neko {
    bookkeeping: NekoBookkeeping,
    animation: Box<dyn Animation>,
}

/// The NekoBookkeeping struct stores the state of the cat such that it can follow the cursor
/// appropriately.
struct NekoBookkeeping {
    pub pos: Pos2,
    pub last_cursor_pos: Pos2,
    /// Speed of the cat in pixels/update.
    pub speed: f32,
    pub ticker: usize,
}

trait Animation {
    /// Draw the current animation frame or request that a different animation be used.
    fn display(
        &mut self,
        ui: &mut egui::Ui,
        books: &mut NekoBookkeeping,
    ) -> Option<Box<dyn Animation>>;
}

struct SleepingNeko;
struct RunningNeko {
    direction: Direction,
}

impl Neko {
    /// Create a new cat.
    pub fn new() -> Self {
        return Self {
            bookkeeping: NekoBookkeeping {
                pos: (0f32, 0f32).into(),
                last_cursor_pos: (0f32, 0f32).into(),
                speed: 12f32,
                ticker: 0,
            },
            animation: Box::new(SleepingNeko),
        };
    }

    /// Draw the cat to the ui.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        self.bookkeeping.ticker = self.bookkeeping.ticker.wrapping_add(1);

        if let Some(cursor_pos) = ui.input(|state| state.pointer.latest_pos()) {
            self.bookkeeping.last_cursor_pos = cursor_pos;
        };

        while let Some(animation) = self.animation.display(ui, &mut self.bookkeeping) {
            self.animation = animation;
        }
    }
}

impl Animation for SleepingNeko {
    fn display(
        &mut self,
        ui: &mut egui::Ui,
        books: &mut NekoBookkeeping,
    ) -> Option<Box<dyn Animation>> {
        // If the cursor has gotten far enough away, wake up.
        if books.last_cursor_pos.distance(books.pos) >= books.speed * 3. {
            return Some(Box::new(RunningNeko {
                // Since the first thing that happens in the RunningNeko `display` call is to set
                // the direction, we're fine just setting this arbitrarily.
                direction: Direction::RIGHT,
            }));
        }

        let min = books.pos;
        let max = (min + Vec2::new(32f32, 32f32)).at_most(ui.max_rect().max);
        let image = WASH_IMAGES[(books.ticker / 10) % 2].clone();
        ui.put(Rect { min, max }, widgets::Image::new(image));
        None
    }
}

impl RunningNeko {
    /// Move one frame in the correct direction.
    fn step(&mut self, books: &mut NekoBookkeeping) {
        let direction = (books.last_cursor_pos - books.pos).angle();
        books.pos += Vec2::angled(direction) * books.speed;
        self.direction = Direction::from_angle(direction)
    }
}

impl Animation for RunningNeko {
    fn display(
        &mut self,
        ui: &mut egui::Ui,
        books: &mut NekoBookkeeping,
    ) -> Option<Box<dyn Animation>> {
        // If we're close enough to the cursor, go to sleep.
        if books.last_cursor_pos.distance(books.pos) < books.speed * 3. {
            return Some(Box::new(SleepingNeko));
        }

        // Don't move on each frame.
        if books.ticker % 10 == 0 {
            self.step(books);
        }

        let min = books.pos;
        let max = (min + Vec2::new(32f32, 32f32)).at_most(ui.max_rect().max);

        let image = match self.direction {
            Direction::RIGHT => &RIGHT_IMAGES,
            Direction::LEFT => &LEFT_IMAGES,
            Direction::DOWN => &DOWN_IMAGES,
            Direction::DOWNLEFT => &DOWNLEFT_IMAGES,
            Direction::DOWNRIGHT => &DOWNRIGHT_IMAGES,
            Direction::UP => &UP_IMAGES,
            Direction::UPLEFT => &UPLEFT_IMAGES,
            Direction::UPRIGHT => &UPRIGHT_IMAGES,
        }[(books.ticker / 10) % 2]
            .clone(); // Cloning looks bad, but internally, an ImageSource will utilize Cows, so it should be okay.

        ui.put(Rect { min, max }, widgets::Image::new(image));
        None
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    DOWNLEFT,
    DOWNRIGHT,
    UP,
    UPLEFT,
    UPRIGHT,
}

impl Direction {
    fn from_angle(angle: f32) -> Self {
        use std::f32::consts::PI;

        // Arctan returns something in [-pi, pi].
        assert!(angle >= -PI);
        assert!(angle <= PI);

        // Angles go clockwise. :(
        if (0.0..=(PI / 8.)).contains(&angle) {
            Self::RIGHT
        } else if ((PI / 8.)..=(3. * PI / 8.)).contains(&angle) {
            Self::DOWNRIGHT
        } else if ((3. * PI / 8.)..=(5. * PI / 8.)).contains(&angle) {
            Self::DOWN
        } else if ((5. * PI / 8.)..=(7. * PI / 8.)).contains(&angle) {
            Self::DOWNLEFT
        } else if ((7. * PI / 8.)..=PI).contains(&angle) {
            Self::LEFT
        } else if (-PI..=(-7. * PI / 8.)).contains(&angle) {
            Self::LEFT
        } else if ((-7. * PI / 8.)..=(-5. * PI / 8.)).contains(&angle) {
            Self::UPLEFT
        } else if ((-5. * PI / 8.)..=(-3. * PI / 8.)).contains(&angle) {
            Self::UP
        } else if ((-3. * PI / 8.)..=(-1. * PI / 8.)).contains(&angle) {
            Self::UPRIGHT
        } else if ((-1. * PI / 8.)..=0.0).contains(&angle) {
            Self::RIGHT
        } else {
            panic!("Checks are exhaustive. Didn't match {angle}")
        }
    }
}
