use midgard::params::{ContentsRadii, WorldGeneratorParameters};
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_map},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, World},
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::TomTom;
use ui_lib::RunnableUi;

pub fn get_world_generator_parameters() -> WorldGeneratorParameters {
    WorldGeneratorParameters {
        time_progression_minutes: 60,
        contents_radii: ContentsRadii {
            trees_in_forest: 5,
            trees_in_hill: 5,
            trees_in_mountain: 5,
            bushes_in_plains: 5,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub struct Ai {
    ui: Box<dyn RunnableUi>,
    robot: Robot,
    counter: u32,
}

impl Ai {
    pub fn new(ui: Box<dyn RunnableUi>) -> Self {
        Self {
            ui,
            robot: Robot::new(),
            counter: 0,
        }
    }

    pub fn run(&mut self, world: &mut World) {
        if self.counter == 0 {
            let _ = Spotlight::illuminate(self, world, 15);
            self.counter = 1;
        } else if self.counter == 1 {
            let _ = TomTom::go_to_tile(
                self,
                world,
                false,
                None,
                Some(rust_eze_tomtom::plain::PlainContent::Tree),
            );

            self.counter = 2;;
        } else if self.counter == 2 {
            let _ = TomTom::go_to_tile(
                self,
                world,
                false,
                None,
                Some(rust_eze_tomtom::plain::PlainContent::Bush),
            );

            self.counter = 1;
        }
    }
}

impl Runnable for Ai {
    fn process_tick(&mut self, world: &mut World) {
        self.run(world);
        self.ui.process_tick(world);
    }

    fn handle_event(&mut self, event: Event) {
        self.ui.handle_event(event);
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
