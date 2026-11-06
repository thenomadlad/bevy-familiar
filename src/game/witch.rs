use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

const WITCH_SPEED: f32 = 200.0;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<Witch>();
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct Witch;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Movement;

fn apply_movement(movement: On<Fire<Movement>>, mut players: Query<&mut Transform, With<Witch>>) {
    let mut transform = players.get_mut(movement.context).unwrap();
    transform.translation += movement.value.extend(0.0);
}

impl Witch {
    fn scene() -> impl Scene {
        bsn! {
            Witch
            Mesh2d(asset_value(Triangle2d::new(vec2(-15.0, 0.0), vec2(15.0, 0.0), vec2(0.0, 20.0))))
            MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 0.2, 0.0)))
            Transform::from_xyz(0.0, 0.0, 12.0)
            Actions<Witch> [
                Action::<Movement>::new()
                DeadZone
                SmoothNudge
                DeltaScale
                ~Scale::splat(WITCH_SPEED)
                Bindings [
                    {preset::cardinal::wasd_keys()}
                    --
                    {preset::cardinal::arrows()}
                    --
                    {preset::axial::left_stick()}
                ]
            ]
            on(apply_movement)
        }
    }
}
