use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::{random, thread_rng};

const DICK_VARIANT_COUNT: u8 = 5;

#[derive(Component)]
pub struct Dick {
    pub variant: u8,
    pub scale: f32,
    pub flipped: bool,
    pub rotation: f32,
}

pub struct BagOfDicks {
    dicks: Vec<Dick>,
}

impl BagOfDicks {
    pub fn new() -> Self {
        Self {
            dicks: BagOfDicks::random_dicks(),
        }
    }

    fn random_dicks() -> Vec<Dick> {
        let mut dicks = Vec::with_capacity(5);
        for i in 1..=DICK_VARIANT_COUNT {
            dicks.push(Dick {
                variant: i,
                scale: 0.5 * random::<f32>() + 1.0,
                flipped: random::<bool>(),
                rotation: (random::<f32>() - 0.5) * 6.0,
            });
        }
        dicks.shuffle(&mut thread_rng());
        dicks
    }
}

impl Iterator for BagOfDicks {
    type Item = Dick;
    fn next(&mut self) -> Option<Dick> {
        if self.dicks.is_empty() {
            self.dicks = BagOfDicks::random_dicks()
        }
        self.dicks.pop()
    }
}
