use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody},
};
use bevy::{ecs::template::OptionTemplate, image::TextureAtlasTemplate, prelude::*};
use bevy_enhanced_input::prelude::*;

use crate::game::GameplaySet;

const WITCH_SPEED: f32 = 200.0;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<Witch>()
        .add_systems(Update, update_animation.in_set(GameplaySet));
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct Witch;

#[derive(Default, Clone, PartialEq, Eq)]
enum MovementState {
    #[default]
    Idle,
    Walking,
    // Casting,
}
#[derive(Default, Clone, PartialEq, Eq)]
enum FaceDirection {
    Left,
    Right,
    Up,
    #[default]
    Down,
}

#[derive(Component, Default, Clone)]
struct AnimationTicker {
    timer: Timer,
}

impl AnimationTicker {
    fn from_seconds(seconds: f32) -> Self {
        AnimationTicker {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Default, Clone, PartialEq, Eq)]
struct AnimationState {
    state: MovementState,
    facing: FaceDirection,
}

#[derive(Clone, FromTemplate)]
struct Clip {
    image: Handle<Image>,
    frames: usize,
}

#[derive(Component, Clone, FromTemplate)]
struct AnimationClips {
    idle: Clip,
    walking: Clip,
}

impl AnimationClips {
    fn get(&self, state: &MovementState) -> &Clip {
        match state {
            MovementState::Idle => &self.idle,
            MovementState::Walking => &self.walking,
        }
    }
}

#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

fn start_movement(
    movement: On<Fire<Move>>,
    mut witch_components: Query<(&mut LinearVelocity, &mut AnimationState), With<Witch>>,
) {
    let (mut velocity, mut animation_state) = witch_components.get_mut(movement.context).unwrap();
    let delta = movement.value;

    // state of the witch
    // if animation_state.state != MovementState::Casting {
    // get direction and set
    let facing = if delta.x.abs() > delta.y.abs() {
        if delta.x.is_sign_positive() {
            FaceDirection::Right
        } else {
            FaceDirection::Left
        }
    } else {
        if delta.y.is_sign_positive() {
            FaceDirection::Up
        } else {
            FaceDirection::Down
        }
    };

    animation_state.set_if_neq(AnimationState {
        state: MovementState::Walking,
        facing,
    });

    // set the velocity
    *velocity = LinearVelocity(delta);
}

fn end_movement(
    movement: On<Complete<Move>>,
    mut animation_states: Query<(&mut AnimationState, &mut LinearVelocity), With<Witch>>,
) {
    let (mut animation_state, mut velocity) = animation_states.get_mut(movement.context).unwrap();
    animation_state.state = MovementState::Idle;
    *velocity = LinearVelocity(Vec2::ZERO);
}

fn update_animation(
    anim_components: Query<(
        Ref<AnimationState>,
        &AnimationClips,
        &mut AnimationTicker,
        &mut Sprite,
    )>,
    time: Res<Time>,
) {
    for (animation_state, animation_clips, mut ticker, mut sprite) in anim_components {
        let row = match animation_state.facing {
            FaceDirection::Left => 0,
            FaceDirection::Right => 1,
            FaceDirection::Up => 2,
            FaceDirection::Down => 3,
        } as usize;
        let clip = animation_clips.get(&animation_state.state);

        if animation_state.is_changed() {
            sprite.image = clip.image.clone();

            // frames are reset
            ticker.as_mut().timer.reset();
            sprite.texture_atlas.as_mut().unwrap().index = row * clip.frames;
        } else if ticker.timer.tick(time.delta()).just_finished() {
            let current_index = sprite.texture_atlas.as_mut().unwrap().index;
            sprite.texture_atlas.as_mut().unwrap().index =
                row * clip.frames + ((current_index + 1) % clip.frames)
        };
    }
}

impl Witch {
    fn scene() -> impl Scene {
        let layout = TextureAtlasLayout::from_grid(UVec2::new(40, 48), 4, 4, None, None);

        bsn! {
            AnimationTicker::from_seconds(0.15)
            AnimationState {
                state: MovementState::Idle,
                facing: FaceDirection::Down,
            }
            AnimationClips {
                idle: Clip { image: "Witch_Idle.png", frames: 4 },
                walking: Clip { image: "Witch_Walk.png", frames: 4 },
            }
            Sprite {
                // start with idle position looking down
                image: "Witch_Idle.png",
                texture_atlas: OptionTemplate::Some(TextureAtlasTemplate {
                    layout: asset_value(layout),
                    index: 12
                })
            }
            Transform::from_xyz(0.0, 0.0, 12.0)
            RigidBody::Dynamic
            Collider::circle(7.0)  // diameter 14 is a little less than 1 tile width of 16
            LockedAxes::ROTATION_LOCKED
            Actions<Witch> [
                Action::<Move>::new()
                DeadZone
                SmoothNudge
                ~Scale::splat(WITCH_SPEED)
                Bindings [
                    {preset::cardinal::wasd_keys()}
                    --
                    {preset::cardinal::arrows()}
                    --
                    {preset::axial::left_stick()}
                ]
            ]
            on(start_movement)
            on(end_movement)
        }
    }
}
