// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use egui::{self, NumExt, Pos2, Rect, Vec2, include_image, widgets};
use std::sync::LazyLock;

static LEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/left1.png")),
    LazyLock::new(|| include_image!("assets/left2.png")),
];

static RIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/right1.png")),
    LazyLock::new(|| include_image!("assets/right2.png")),
];

static DOWN_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/down1.png")),
    LazyLock::new(|| include_image!("assets/down2.png")),
];

static DOWNLEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/downleft1.png")),
    LazyLock::new(|| include_image!("assets/downleft2.png")),
];

static DOWNRIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/downright1.png")),
    LazyLock::new(|| include_image!("assets/downright2.png")),
];

static UP_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/up1.png")),
    LazyLock::new(|| include_image!("assets/up2.png")),
];

static UPLEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/upleft1.png")),
    LazyLock::new(|| include_image!("assets/upleft2.png")),
];

static UPRIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/upright1.png")),
    LazyLock::new(|| include_image!("assets/upright2.png")),
];

static WASH_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/wash1.png")),
    LazyLock::new(|| include_image!("assets/wash2.png")),
];

/// The [Neko] struct stores the state of the cat such that it can follow the cursor appropriately.
pub struct Neko {
    pos: Pos2,
    last_cursor_pos: Pos2,
    /// Speed of the cat in pixels/frame.
    speed: f32,
    state: usize,
    ticker: usize,
    direction: Direction,
}

impl Neko {
    /// Create a new cat.
    pub fn new() -> Self {
        return Self {
            pos: (0f32, 0f32).into(),
            last_cursor_pos: (0f32, 0f32).into(),
            speed: 12f32,
            state: 0,
            ticker: 0,
            direction: Direction::RIGHT, // TODO: sleeping, wake animation
        };
    }

    /// Draw the cat to the ui.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        self.ticker = self.ticker.wrapping_add(1);

        // Don't move on each render.
        if let Some(cursor_pos) = ui.input(|state| state.pointer.latest_pos()) {
            self.last_cursor_pos = cursor_pos;
        };

        if self.last_cursor_pos.distance(self.pos) < self.speed * 3. {
            self.display_sitting(ui);
        } else {
            if self.ticker % 10 == 0 {
                self.direction = self.step();
            }
            self.display_running(ui, self.direction);
        }
    }

    /// Move one frame in the correct direction.
    fn step(&mut self) -> Direction {
        let direction = (self.last_cursor_pos - self.pos).angle();
        self.pos += Vec2::angled(direction) * self.speed;

        self.state = self.state.wrapping_add(1);
        Direction::from_angle(direction)
    }

    /// Draw ourselves onto the screen. This is a helper method for [draw].
    fn display_running(&mut self, ui: &mut egui::Ui, direction: Direction) {
        let min = self.pos;
        let max = (min + Vec2::new(32f32, 32f32)).at_most(ui.max_rect().max);

        let image = match direction {
            Direction::RIGHT => &RIGHT_IMAGES,
            Direction::LEFT => &LEFT_IMAGES,
            Direction::DOWN => &DOWN_IMAGES,
            Direction::DOWNLEFT => &DOWNLEFT_IMAGES,
            Direction::DOWNRIGHT => &DOWNRIGHT_IMAGES,
            Direction::UP => &UP_IMAGES,
            Direction::UPLEFT => &UPLEFT_IMAGES,
            Direction::UPRIGHT => &UPRIGHT_IMAGES,
        }[self.state % 2]
            .clone(); // Cloning looks bad, but internally, an ImageSource will utilize Cows, so it should be okay.

        ui.put(Rect { min, max }, widgets::Image::new(image));
    }

    fn display_sitting(&mut self, ui: &mut egui::Ui) {
        let min = self.pos;
        let max = (min + Vec2::new(32f32, 32f32)).at_most(ui.max_rect().max);
        let image = WASH_IMAGES[self.state % 2].clone();
        ui.put(Rect { min, max }, widgets::Image::new(image));
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
