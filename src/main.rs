use bevy::{prelude::*, sprite::Mesh2d};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update, update_colors))
        .run();
}

#[derive(Component)]
struct Lion {
    energy: f32,
    age: f32,
}

#[derive(Component)]
struct Prey {
    energy: f32,
}

#[derive(Component)]
struct MovementAngle(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for _ in 0..20 {
        commands.spawn((
            ColorMesh2dBundle {
                material: colors.add(Color::RED),
                mesh: meshes.add(Mesh::from(Circle::new(4.0))).into(),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    0.0,
                ),
                ..default()
            },
            Lion {
                energy: 0.0,
                age: 5.0,
            },
            MovementAngle(rand::thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0)),
        ));
    }

    for _ in 0..20 {
        commands.spawn((
            ColorMesh2dBundle {
                material: colors.add(Color::GREEN),
                mesh: meshes.add(Mesh::from(Circle::new(4.0))).into(),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    0.0,
                ),
                ..default()
            },
            Prey { energy: 0.0 },
            MovementAngle(rand::thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0)),
        ));
    }
}

fn update_colors(
    mut lions: Query<
        (&Lion, &mut Handle<ColorMaterial>),
        (With<Lion>, Without<Prey>),
    >,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    for (lion, mut material) in lions.iter_mut() {
        if lion.age > 3.0 {
            *material = colors.add(Color::RED);
        }
    }
}

fn update(
    mut commands: Commands,
    mut lions: Query<
        (Entity, &mut Transform, &mut MovementAngle, &mut Lion),
        (With<Lion>, Without<Prey>),
    >,
    mut prey: Query<
        (Entity, &mut Transform, &mut MovementAngle, &mut Prey),
        (With<Prey>, Without<Lion>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let mut positions = vec![];

    for (e, t, _, lion) in lions.iter() {
        if lion.age < 3.0 {
            continue;
        }

        positions.push((e, t.translation));
    }

    for (e, t, ..) in prey.iter() {
        positions.push((e, t.translation));
    }

    for (e, mut transform, mut angle, mut lion) in lions.iter_mut() {
        lion.age += time.delta_seconds();

        if lion.age < 3.0 {
            angle.0 += rand::thread_rng().gen_range(-0.3..=0.3) * time.delta_seconds();
        } else {
            let nearest_target = positions.iter().filter(|c| c.0 != e).min_by(|c1, c2| {
                (c1.1 - transform.translation)
                    .length_squared()
                    .total_cmp(&(c2.1 - transform.translation).length_squared())
            });

            let distance = nearest_target
                .map(|(_, translation)| (*translation - transform.translation).length())
                .unwrap_or(99999.9);

            if distance > 500.0 {
                angle.0 += rand::thread_rng().gen_range(-0.3..=0.3) * time.delta_seconds();
            } else if distance < 5.0 {
                if prey.contains(nearest_target.unwrap().0) {
                    commands
                        .entity(nearest_target.unwrap().0)
                        .despawn_recursive();
                    lion.energy += 1.0;
                } else {
                    commands
                        .entity(nearest_target.unwrap().0)
                        .despawn_recursive();
                    lion.energy += 0.5;
                }

                if lion.energy >= 1.0 {
                    commands.spawn((
                        ColorMesh2dBundle {
                            material: colors.add(Color::ORANGE),
                            mesh: meshes.add(Mesh::from(Circle::new(4.0))).into(),
                            transform: transform.clone(),
                            ..default()
                        },
                        Lion {
                            energy: 0.0,
                            age: 0.0,
                        },
                        MovementAngle(
                            rand::thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0),
                        ),
                    ));

                    lion.energy = 0.0;
                }
            } else {
                angle.0 = (nearest_target.unwrap().1.y - transform.translation.y)
                    .atan2(nearest_target.unwrap().1.x - transform.translation.x);
            }
        }

        transform.translation.x += angle.0.cos() * 80.0 * time.delta_seconds();
        transform.translation.y += angle.0.sin() * 80.0 * time.delta_seconds();

        if transform.translation.x > 500.0 {
            transform.translation.x -= 1000.0;
        }
        if transform.translation.x < -500.0 {
            transform.translation.x += 1000.0;
        }
        if transform.translation.y > 500.0 {
            transform.translation.y -= 1000.0;
        }
        if transform.translation.y < -500.0 {
            transform.translation.y += 1000.0;
        }
    }

    for (_, mut transform, mut angle, mut prey) in prey.iter_mut() {
        angle.0 += rand::thread_rng().gen_range(-0.3..=0.3) * time.delta_seconds();

        transform.translation.x += angle.0.cos() * 60.0 * time.delta_seconds();
        transform.translation.y += angle.0.sin() * 60.0 * time.delta_seconds();

        if transform.translation.x > 500.0 {
            transform.translation.x -= 1000.0;
        }
        if transform.translation.x < -500.0 {
            transform.translation.x += 1000.0;
        }
        if transform.translation.y > 500.0 {
            transform.translation.y -= 1000.0;
        }
        if transform.translation.y < -500.0 {
            transform.translation.y += 1000.0;
        }

        prey.energy += 0.2 * time.delta_seconds();

        if prey.energy >= 1.0 {
            commands.spawn((
                ColorMesh2dBundle {
                    material: colors.add(Color::GREEN),
                    mesh: meshes.add(Mesh::from(Circle::new(4.0))).into(),
                    transform: transform.clone(),
                    ..default()
                },
                Prey { energy: 0.0 },
                MovementAngle(rand::thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0)),
            ));

            prey.energy = 0.0;
        }
    }
}
