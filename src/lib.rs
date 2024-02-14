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

pub struct Ai {
    ui: Box<dyn RunnableUi>,
    robot: Robot,
    first: bool,
}

impl Ai {
    pub fn new(ui: Box<dyn RunnableUi>) -> Self {
        Self {
            ui,
            robot: Robot::new(),
            first: true,
        }
    }

    pub fn run(&mut self, world: &mut World) {
        if self.first {
            self.first = false;

            Spotlight::illuminate(self, world, 7);

            TomTom::go_to_tile(
                self,
                world,
                false,
                None,
                Some(rust_eze_tomtom::plain::PlainContent::Bush),
            );
        } else {
            go(self, world, robotics_lib::interface::Direction::Down);
            go(self, world, robotics_lib::interface::Direction::Right);
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
