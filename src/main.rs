use std::{f64::consts::FRAC_PI_2, time::Duration};

use libjaka::JakaMini2;
use robot_behavior::behavior::*;
use rsbullet::{Mode, RsBullet};

fn main() -> anyhow::Result<()> {
    let mut physics = RsBullet::new(Mode::Gui)?;

    let mut robot = physics
        .robot_builder::<JakaMini2>("exam_robot")
        .base([0., 0., 0.])
        .load()?;

    robot.move_joint(&[FRAC_PI_2; _])?;

    loop {
        physics.step()?;
    }

}
