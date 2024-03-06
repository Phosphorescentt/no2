use crate::traits::{EventHandler, FrameRenderer};

pub struct HomeState;

impl Default for HomeState {
    fn default() -> Self {
        HomeState
    }
}

impl EventHandler for HomeState {}

impl FrameRenderer for HomeState {}
