use crate::client::SensorData;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InitPhase {
    WaitMonitor,
    WaitFirstPoint,
    WaitSecondPoint,
    Finalize,
}

#[derive(Copy, Clone, Debug)]
pub struct InitData {
    window_size: (f32, f32),
    monitor: SensorData,
    first_point: SensorData,
    second_point: SensorData,
}

impl InitData {
    pub fn new(window_size: (f32, f32)) -> InitData {
        InitData {
            window_size,
            monitor: (0.0, 0.0, 0.0),
            first_point: (0.0, 0.0, 0.0),
            second_point: (0.0, 0.0, 0.0),
        }
    }

    pub fn window_size(&self) -> (f32, f32) {
        self.window_size
    }

    pub fn monitor(&self) -> SensorData {
        self.monitor
    }

    pub fn first_point(&self) -> SensorData {
        self.first_point
    }

    pub fn second_point(&self) -> SensorData {
        self.second_point
    }
    
    pub fn set_monitor(&mut self, data: SensorData) {
        self.monitor = data;
    }
    
    pub fn set_first_point(&mut self, data: SensorData) {
        self.first_point = data;
    }
    
    pub fn set_second_point(&mut self, data: SensorData) {
        self.second_point = data;
    }
}