pub mod structs {
    pub struct Vector2 {
        x: i32,
        y: i32,
    }

    impl Vector2 {
        pub fn new(x: i32, y: i32) -> Self {
            Vector2 { x, y }
        }
    }
}
