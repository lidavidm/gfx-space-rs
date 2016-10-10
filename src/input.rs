use glutin;

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    // Mouse coordinates
    pub win_x: i32,
    pub win_y: i32,
    pub world_x: f32,
    pub world_y: f32,
}

impl Input {
    pub fn new() -> Input {
        Input {
            forward: false,
            backward: false,
            left: false,
            right: false,
            win_x: 0,
            win_y: 0,
            world_x: 0.0,
            world_y: 0.0,
        }
    }

    pub fn key_pressed(&mut self,
                       state: glutin::ElementState,
                       code: glutin::ScanCode,
                       vcode: Option<glutin::VirtualKeyCode>) {
        match code {
            25 => self.forward = state == glutin::ElementState::Pressed,
            39 => self.backward = state == glutin::ElementState::Pressed,
            38 => self.left = state == glutin::ElementState::Pressed,
            40 => self.right = state == glutin::ElementState::Pressed,
            _ => {},
        }
    }

    pub fn mouse_moved(&mut self, win_x: i32, win_y: i32, world_x: f32, world_y: f32) {
        self.win_x = win_x;
        self.win_y = win_y;
        self.world_x = world_x;
        self.world_y = world_y;
    }
}
