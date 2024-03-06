use crate::traits::{EventHandler, FrameRenderer};

pub struct EndState;

impl EventHandler for EndState {}

impl FrameRenderer for EndState {}
