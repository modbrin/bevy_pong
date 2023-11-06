use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use num::clamp;

use crate::utils::*;
use crate::{GameState, LastWinner};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        let board = BoardConfig {
            ball_speed: 400.,
            width: 1000.,
            height: 700.,
            dice_width: 40.,
            dice_length: 150.,
            dice_offset: 100.,
            ball_radius: 20.,
            border_width: 20.,
            winning_score: 2,
            start_delay: 1.5,
        };

        app.insert_resource(board)
            .insert_resource(Score::default())
            .add_event::<PlayerLost>()
            .add_systems(OnEnter(GameState::Gameplay), spawn_board)
            .add_systems(OnExit(GameState::Gameplay), despawn_board)
            .add_systems(
                Update,
                (
                    handle_input,
                    update_dices,
                    update_ball,
                    update_dice_animation,
                    next_round,
                    update_delayed_ball_start,
                ),
            );
    }
}

#[derive(Component)]
struct BoardTag;

#[derive(Component)]
struct Dice {
    axis_input: f32,
    kind: DiceKind,
}

#[derive(Component)]
struct AnimatedDiceBounce {
    speed: f32,
    original_x: f32,
    max_offset: f32,
    offset_reached: bool,
}

impl AnimatedDiceBounce {
    pub fn with_dice(dice_x: f32) -> Self {
        Self {
            speed: 300.0,
            original_x: dice_x,
            max_offset: 5.0,
            offset_reached: false,
        }
    }
}

#[derive(Component)]
struct DelayedBallStart {
    remaining_sec: f32,
}

impl DelayedBallStart {
    pub fn new(remaining_sec: f32) -> Self {
        Self { remaining_sec }
    }
}

#[derive(Default, Resource)]
struct Score {
    left: usize,
    right: usize,
}

impl Score {
    pub fn reset(&mut self) {
        self.left = 0;
        self.right = 0;
    }
}

const INPUT_FACTOR: f32 = 1000.;
const BALL_COLOR: Color = Color::RED;

#[derive(Resource)]
struct BoardConfig {
    /// board dimension from left to right
    width: f32,
    /// board dimensions from top to bottom
    height: f32,
    /// dice visual thickness
    dice_width: f32,
    /// area of dice hit surface
    dice_length: f32,
    /// offset from board edge to dice axis
    dice_offset: f32,
    /// speed of ball travel
    ball_speed: f32,
    /// visual radius of ball
    ball_radius: f32,
    /// visual width of surrounding walls
    border_width: f32,
    /// score for one player to win the game
    winning_score: usize,
    /// secs before ball is launched
    start_delay: f32,
}

impl BoardConfig {
    pub fn max_dice_position(&self, is_top: bool) -> f32 {
        let offset = self.height / 2. - self.dice_length / 2.;
        if is_top {
            offset
        } else {
            -offset
        }
    }
}

#[derive(Default, Component)]
struct Ball {
    is_colliding_y: bool,
    is_colliding_x: bool,
    is_lost: bool,
    velocity_x: f32,
    velocity_y: f32,
}

impl Ball {
    pub fn reset(&mut self) {
        *self = Ball::default();
    }
}

#[derive(Event)]
struct PlayerLost {
    is_right: bool,
}

fn spawn_border(commands: &mut Commands, width: f32, height: f32, position: Vec2) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
            ..default()
        })
        .id()
}

fn spawn_dice(commands: &mut Commands, kind: DiceKind, board: &BoardConfig) -> Entity {
    let position_x = match kind {
        DiceKind::Left => {
            -board.width / 2. + board.dice_offset - board.ball_radius - board.dice_width / 2.
        }
        DiceKind::Right => {
            board.width / 2. - board.dice_offset + board.ball_radius + board.dice_width / 2.
        }
    };
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(board.dice_width, board.dice_length)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position_x, 0., 0.)),
                ..default()
            },
            Dice {
                axis_input: 0.0,
                kind,
            },
        ))
        .id()
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    board: &BoardConfig,
) -> Entity {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(board.ball_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            Ball::default(),
            DelayedBallStart::new(board.start_delay),
        ))
        .id()
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>,
    board: Res<BoardConfig>,
) {
    score.reset();
    let border_width = board.border_width;
    let border_top = spawn_border(
        &mut commands,
        board.width + border_width * 2.,
        border_width,
        Vec2::new(0., board.height / 2. + border_width / 2.),
    );
    commands.entity(border_top).insert(BoardTag);
    let border_bottom = spawn_border(
        &mut commands,
        board.width + border_width * 2.,
        border_width,
        Vec2::new(0., -board.height / 2. - border_width / 2.),
    );
    commands.entity(border_bottom).insert(BoardTag);
    let border_right = spawn_border(
        &mut commands,
        border_width,
        board.height + border_width * 2.,
        Vec2::new(board.width / 2. + border_width / 2., 0.),
    );
    commands.entity(border_right).insert(BoardTag);
    let border_left = spawn_border(
        &mut commands,
        border_width,
        board.height + border_width * 2.,
        Vec2::new(-board.width / 2. - border_width / 2., 0.),
    );
    commands.entity(border_left).insert(BoardTag);

    let left_dice = spawn_dice(&mut commands, DiceKind::Left, &board);
    let right_dice = spawn_dice(&mut commands, DiceKind::Right, &board);
    let ball = spawn_ball(&mut commands, &mut meshes, &mut materials, &board);
    commands.entity(left_dice).insert(BoardTag);
    commands.entity(right_dice).insert(BoardTag);
    commands.entity(ball).insert(BoardTag);
}

fn despawn_board(mut commands: Commands, entities: Query<Entity, With<BoardTag>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn next_round(
    mut commands: Commands,
    mut score: ResMut<Score>,
    board: Res<BoardConfig>,
    mut event_reader: EventReader<PlayerLost>,
    mut ball: Query<(Entity, &mut Ball, &mut Transform)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_winner: ResMut<LastWinner>,
) {
    if let Some(PlayerLost { is_right }) = event_reader.iter().next() {
        if *is_right {
            score.left += 1;
        } else {
            score.right += 1;
        }

        let (entity, mut ball, mut transform) = ball.single_mut();
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        ball.reset();
        commands
            .entity(entity)
            .insert(DelayedBallStart::new(board.start_delay));

        if score.left >= board.winning_score {
            last_winner.player = Some(DiceKind::Left);
            next_state.set(GameState::GameOver)
        } else if score.right >= board.winning_score {
            last_winner.player = Some(DiceKind::Right);
            next_state.set(GameState::GameOver)
        }
    }
}

fn handle_input(mut dices: Query<&mut Dice>, keyboard: Res<Input<KeyCode>>) {
    let mut left_input = 0.0;
    if keyboard.pressed(KeyCode::W) {
        left_input += 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        left_input -= 1.0;
    }

    let mut right_input = 0.0;
    if keyboard.pressed(KeyCode::Up) {
        right_input += 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
        right_input -= 1.0;
    }

    for mut dice in &mut dices {
        match dice.kind {
            DiceKind::Right => dice.axis_input = right_input,
            DiceKind::Left => dice.axis_input = left_input,
        }
    }
}

fn update_dices(
    time: Res<Time>,
    board: Res<BoardConfig>,
    mut dices: Query<(&Dice, &mut Transform)>,
) {
    let dt = time.delta().as_secs_f32();
    let top_y_limit = board.max_dice_position(true);
    let bottom_y_limit = board.max_dice_position(false);

    for (dice, mut transform) in &mut dices {
        transform.translation.y += dt * dice.axis_input * INPUT_FACTOR;
        transform.translation.y = clamp(transform.translation.y, bottom_y_limit, top_y_limit);
    }
}

// returns normalized ball velocity
fn calculate_dice_collision(
    ball_y: f32,
    quarter_ball: f32,
    dice_y: f32,
    half_dice: f32,
    kind: DiceKind,
) -> Option<Vec2> {
    let upper_edge = dice_y + half_dice + quarter_ball;
    let lower_edge = dice_y - half_dice - quarter_ball;
    if ball_y <= upper_edge && ball_y >= lower_edge {
        let ball_shift = (dice_y - ball_y) / (half_dice + quarter_ball);
        let bounce_angle = ball_shift * MAX_BOUNCE_ANGLE;
        let new_v = match kind {
            DiceKind::Right => Vec2::new(-bounce_angle.cos(), -bounce_angle.sin()),
            DiceKind::Left => Vec2::new(bounce_angle.cos(), -bounce_angle.sin()),
        };
        return Some(new_v);
    }
    None
}

fn update_ball(
    mut commands: Commands,
    time: Res<Time>,
    board: Res<BoardConfig>,
    mut ball: Query<(&mut Ball, &mut Transform), Without<Dice>>,
    dices: Query<(Entity, &Transform, &Dice), With<Dice>>,
    mut event_writer: EventWriter<PlayerLost>,
) {
    let dt = time.delta().as_secs_f32();

    if let Ok((mut ball, mut transform)) = ball.get_single_mut() {
        transform.translation.x += ball.velocity_x * dt * board.ball_speed;
        transform.translation.y += ball.velocity_y * dt * board.ball_speed;
        let (ball_x, ball_y) = (transform.translation.x, transform.translation.y);

        let top_y = board.height / 2. - board.ball_radius;
        let bottom_y = -board.height / 2. + board.ball_radius;
        let dice_axis_left = -board.width / 2. + board.dice_offset;
        let dice_axis_right = board.width / 2. - board.dice_offset;
        if !ball.is_colliding_x {
            let half_dice = board.dice_length / 2.;
            let over_right_axis = ball_x > dice_axis_right;
            let over_left_axis = ball_x < dice_axis_left;
            if (over_left_axis || over_right_axis) && !ball.is_lost {
                let check_dice = match (over_right_axis, over_left_axis) {
                    (true, false) => DiceKind::Right,
                    (false, true) => DiceKind::Left,
                    _ => unreachable!(),
                };
                for (entity, transform, dice) in dices.iter() {
                    if dice.kind == check_dice {
                        let dice_x = transform.translation.x;
                        let dice_y = transform.translation.y;
                        let quarter_ball = board.ball_radius / 2.;
                        if let Some(new_v) = calculate_dice_collision(
                            ball_y,
                            quarter_ball,
                            dice_y,
                            half_dice,
                            dice.kind,
                        ) {
                            ball.velocity_x = new_v.x;
                            ball.velocity_y = new_v.y;
                            ball.is_colliding_x = true;
                            commands
                                .entity(entity)
                                .insert(AnimatedDiceBounce::with_dice(dice_x));
                        } else {
                            ball.is_lost = true;
                        }
                    }
                }
            }
        } else {
            if ball_x > dice_axis_left && ball_x < dice_axis_right {
                ball.is_colliding_x = false;
            }
        }
        if !ball.is_colliding_y {
            if ball_y > top_y || ball_y < bottom_y {
                ball.velocity_y *= -1.;
                ball.is_colliding_y = true;
            }
        } else {
            if ball_y < top_y && ball_y > bottom_y {
                ball.is_colliding_y = false;
            }
        }
        if ball.is_lost {
            if ball_x > board.width / 2. {
                event_writer.send(PlayerLost { is_right: true });
            }
            if ball_x < -board.width / 2. {
                event_writer.send(PlayerLost { is_right: false });
            }
        }
    }
}

fn update_dice_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Dice, &mut AnimatedDiceBounce)>,
) {
    let dt = time.delta().as_secs_f32();
    for (entity, mut transform, dice, mut anim) in query.iter_mut() {
        let dice_x = &mut transform.translation.x;
        if !anim.offset_reached {
            match dice.kind {
                DiceKind::Right => {
                    *dice_x += dt * anim.speed;
                    if *dice_x > anim.original_x + anim.max_offset {
                        anim.offset_reached = true;
                    }
                }
                DiceKind::Left => {
                    *dice_x -= dt * anim.speed;
                    if *dice_x < anim.original_x - anim.max_offset {
                        anim.offset_reached = true;
                    }
                }
            }
        } else {
            match dice.kind {
                DiceKind::Right => {
                    *dice_x -= dt * anim.speed;
                    if *dice_x <= anim.original_x {
                        *dice_x = anim.original_x;
                        commands.entity(entity).remove::<AnimatedDiceBounce>();
                    }
                }
                DiceKind::Left => {
                    *dice_x += dt * anim.speed;
                    if *dice_x >= anim.original_x {
                        *dice_x = anim.original_x;
                        commands.entity(entity).remove::<AnimatedDiceBounce>();
                    }
                }
            }
        }
    }
}

fn update_delayed_ball_start(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Ball, &mut DelayedBallStart)>,
) {
    if let Ok((entity, mut ball, mut delay)) = query.get_single_mut() {
        let dt = time.delta().as_secs_f32();
        delay.remaining_sec -= dt;
        if delay.remaining_sec < 0.0 {
            let angle = get_random_starting_angle();
            ball.velocity_x = angle.x;
            ball.velocity_y = angle.y;
            commands.entity(entity).remove::<DelayedBallStart>();
        }
    }
}
