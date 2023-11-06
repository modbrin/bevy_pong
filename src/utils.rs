use std::f32::consts::PI;

use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DiceKind {
    Left,
    Right,
}

pub static FONT_PATH: &str = "fonts/Minimal5x7.ttf";

pub const MAX_BOUNCE_ANGLE: f32 = 5. * PI / 12.;

pub fn get_random_starting_angle() -> Vec2 {
    let step = Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();
    let swing = step.sample(&mut rng);
    let is_right = rng.gen::<bool>();
    let bounce_angle = swing * MAX_BOUNCE_ANGLE;
    if is_right {
        return Vec2::new(-bounce_angle.cos(), -bounce_angle.sin());
    } else {
        return Vec2::new(bounce_angle.cos(), -bounce_angle.sin());
    }
}

pub fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                height: Val::Percent(15.0),
                width: Val::Percent(65.0),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}
