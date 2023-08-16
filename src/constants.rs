pub mod constants {
    use crate::enums::MinoType;
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;

    use crate::structs::structs::Vector2;
    use once_cell::sync::Lazy;

    pub static DIFFS: Lazy<[Vector2; 7]> = Lazy::new(|| {
        [
            Vector2::new(1, 1),
            Vector2::new(1, 1),
            Vector2::new(0, 1),
            Vector2::new(1, 1),
            Vector2::new(1, 1),
            Vector2::new(1, 1),
            Vector2::new(1, 1),
        ]
    });

    pub static SHAPES: Lazy<[[[Vector2; 4]; 4]; 7]> = Lazy::new(|| {
        [
            [
                //Z
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                ],
                [
                    Vector2::new(2, 0),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(1, 2),
                ],
                [
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                    Vector2::new(2, 2),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(0, 2),
                ],
            ],
            //L
            [
                [
                    Vector2::new(2, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                    Vector2::new(2, 2),
                ],
                [
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(0, 2),
                ],
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                ],
            ],
            //O
            [
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                ],
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                ],
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                ],
                [
                    Vector2::new(0, 0),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                ],
            ],
            //S
            [
                [
                    Vector2::new(1, 0),
                    Vector2::new(2, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(2, 2),
                ],
                [
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(0, 2),
                    Vector2::new(1, 2),
                ],
                [
                    Vector2::new(0, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                ],
            ],
            //I
            [
                [
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(3, 1),
                ],
                [
                    Vector2::new(2, 0),
                    Vector2::new(2, 1),
                    Vector2::new(2, 2),
                    Vector2::new(2, 3),
                ],
                [
                    Vector2::new(0, 2),
                    Vector2::new(1, 2),
                    Vector2::new(2, 2),
                    Vector2::new(3, 2),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                    Vector2::new(1, 3),
                ],
            ],
            //J
            [
                [
                    Vector2::new(0, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(2, 0),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                ],
                [
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(2, 2),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(0, 2),
                    Vector2::new(1, 2),
                ],
            ],
            //T
            [
                [
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(1, 2),
                ],
                [
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(2, 1),
                    Vector2::new(1, 2),
                ],
                [
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(1, 1),
                    Vector2::new(1, 2),
                ],
            ],
        ]
    });

    pub const MINO_BAG: &'static [MinoType; 7] = &[
        MinoType::Z,
        MinoType::L,
        MinoType::O,
        MinoType::S,
        MinoType::I,
        MinoType::J,
        MinoType::T,
    ];
}
