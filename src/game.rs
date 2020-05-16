use crate::engine::ResizeStrategy;
use legion::prelude::*;
use quicksilver::geom::Vector;

//320 x 180 or 480 x 270
pub const DIMENSIONS: Vector = Vector { x: 320., y: 180. };
pub const UPDATE_RATE: f32 = 60.;

// test button system
use crate::engine::input::Button;
use crate::engine::ButtonsState;
use quicksilver::lifecycle::EventCache;

// images
use fxhash::FxHashMap;
use quicksilver::graphics::Image;
type ImageStorage = FxHashMap<String, Image>;

// collisions
use crate::phx::CollisionWorld;

pub struct Game {
    pub universe: Universe,
    pub resources: Resources,
    pub resize_strategy: ResizeStrategy,
    // world might become state specific
    pub world: World,
    // schedule most definitely will become state specific
    pub schedule: Schedule,
    pub images: ImageStorage,
}

impl Game {
    pub fn new() -> Self {
        let universe = Universe::new();
        let world = universe.create_world();

        // Put all game-level resources in
        let resources = init_resources();

        let schedule = init_schedule();

        let resize_strategy = ResizeStrategy::Stretch;

        // Texture is not thread safe can't put as resource for now!
        let images = ImageStorage::default();

        Game {
            universe,
            resources,
            world,
            schedule,
            resize_strategy,
            images,
        }
    }
}

fn init_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(EventCache::default());
    resources.insert(ButtonsState::default());
    resources.insert(CollisionWorld::new(0.02));
    resources.insert(crate::phx::PositionCorrection::default());
    resources.insert(crate::phx::ActiveCollisions::with_capacity(25));
    resources
}

fn init_schedule() -> Schedule {
    use crate::phx::Velocity;
    use crate::Player;
    let test_button_state = SystemBuilder::new("test_button_state")
        .read_resource::<EventCache>()
        .write_resource::<ButtonsState>()
        .write_resource::<crate::phx::CollisionWorld>()
        .write_resource::<crate::phx::CollisionHandle>()
        .with_query(<(Read<Player>, Write<Velocity>)>::query())
        .build(move |_, mut world, (event_cache, button_state, cworld, handle), query| {
            button_state.update(&event_cache);
            // if button_state.is_pressed(Button::Up) {
            //     debug!("Holding UP!");
            // }
            if button_state.pressed(Button::Jump) {
                debug!("Wow you just pressed the Jump button.");
            }
            if button_state.released(Button::Jump) {
                debug!("Congrats on releasing the Jump button");
                let something = handle.clone();
                cworld.remove(&[something]);
            }
            const KEYS: &'static [(Button, f32, f32)] = &[
                (Button::Up, 0., -1.),
                (Button::Down, 0., 1.),
                (Button::Left, -1., 0.),
                (Button::Right, 1., 0.),
            ];
            let dir = KEYS.into_iter().fold((0., 0.), |acc, (button, x, y)| {
                if button_state.is_pressed(*button) {
                    (acc.0 + x, acc.1 + y)
                } else {
                    acc
                }
            });
            for (_, mut vel) in query.iter_mut(&mut world) {
                vel.src.x = dir.0 * 64.;
                vel.src.y = dir.1 * 64.;
            }
        });

    Schedule::builder()
        .add_system(test_button_state)
        .add_system(crate::phx::movement::movement())
        // from here onwards Position shouldn't be modified by non-ncollide related systems
        .add_system(crate::phx::sync_ncollide())
        .add_system(crate::phx::ncollide_update())
        .add_system(crate::phx::collision::correct_position())
        // here the position is already corrected... OR IS IT?
        .build()
}
