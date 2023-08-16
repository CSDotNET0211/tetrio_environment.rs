pub struct PlayerOptions {
    arr: f32,
    das: f32,
    dcd: f32,
    sdf: f32,
    safelock: f32,
    cancel: bool,
}

impl PlayerOptions {
    pub fn new(arr: f32, das: f32, dcd: f32, sdf: f32, safelock: f32, cancel: bool) -> Self {
        PlayerOptions {
            arr,
            das,
            dcd,
            sdf,
            safelock,
            cancel,
        }
    }
}
