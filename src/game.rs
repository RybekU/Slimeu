use crate::engine::ResizeStrategy;
use legion::prelude::*;
use quicksilver::geom::Vector;

//320 x 180 or 480 x 270
pub const DIMENSIONS: Vector = Vector { x: 320., y: 180. };

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
    resources
}

fn init_schedule() -> Schedule {
    let test_button_state = SystemBuilder::new("test_button_state")
        .read_resource::<EventCache>()
        .write_resource::<ButtonsState>()
        .build(move |_, _, (event_cache, button_state), _| {
            button_state.update(&event_cache);
            if button_state.is_pressed(Button::Up) {
                debug!("Holding UP!");
            }
            if button_state.pressed(Button::Jump) {
                debug!("Wow you just pressed the Jump button.");
            }
            if button_state.released(Button::Jump) {
                debug!("Congrats on releasing the Jump button");
            }
        });

    Schedule::builder()
        .add_system(test_button_state)
        // .flush()
        .build()
}
