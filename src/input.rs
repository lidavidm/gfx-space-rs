use glutin;

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            forward: false,
            backward: false,
            left: false,
            right: false,
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
}
