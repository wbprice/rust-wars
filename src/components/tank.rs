pub struct Tank {
    pub faction: Faction
    pub x: i8,
    pub y: i8
}

impl Tank {
    fn new (faction: Faction, x: i8, y: i8) -> Tank {
        Tank {
            faction,
            x,
            y
        }
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Self>;
}