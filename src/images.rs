use egui::include_image;
use std::sync::LazyLock;

pub static WAKING_IMAGE: egui::ImageSource = include_image!("assets/awake.png");

pub static LEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/left1.png")),
    LazyLock::new(|| include_image!("assets/left2.png")),
];

pub static RIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/right1.png")),
    LazyLock::new(|| include_image!("assets/right2.png")),
];

pub static DOWN_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/down1.png")),
    LazyLock::new(|| include_image!("assets/down2.png")),
];

pub static DOWNLEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/downleft1.png")),
    LazyLock::new(|| include_image!("assets/downleft2.png")),
];

pub static DOWNRIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/downright1.png")),
    LazyLock::new(|| include_image!("assets/downright2.png")),
];

pub static UP_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/up1.png")),
    LazyLock::new(|| include_image!("assets/up2.png")),
];

pub static UPLEFT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/upleft1.png")),
    LazyLock::new(|| include_image!("assets/upleft2.png")),
];

pub static UPRIGHT_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/upright1.png")),
    LazyLock::new(|| include_image!("assets/upright2.png")),
];

pub static WASH_IMAGES: [LazyLock<egui::ImageSource>; 2] = [
    LazyLock::new(|| include_image!("assets/wash1.png")),
    LazyLock::new(|| include_image!("assets/wash2.png")),
];
