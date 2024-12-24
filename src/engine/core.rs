use super::scene::*;
use crate::{Log, Properties};

pub struct Engine {
    pub(crate) is_running: bool,
    pub scenes: Vec<Scene>,
    pub current_scene: Scene,
    pub delta_time: Option<u32>,
    pub properties: Properties,
    pub(crate) logs: Vec<Log>,
}
impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            is_running: false,
            scenes: get_scenes(),
            current_scene: get_current_scene(),
            delta_time: None,
            properties: Properties {
                pause_on_exit: true,
            },
            logs: Vec::new(),
        }
    }
}
