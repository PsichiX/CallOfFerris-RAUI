use crate::{
    components::{
        barrel::Barrel,
        enemy::Enemy,
        player::Player,
        tile::{Tile, TileType},
    },
    utils::AssetManager,
};

pub struct Map {
    draw_pos: f32,
    draw_inc: f32,

    pub ground: Vec<Tile>,
    pub enemies: Vec<Enemy>,
    pub total_enemies: i32,
    pub barrels: Vec<Barrel>,

    pub player: Option<Player>,
    pub end: Option<String>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            draw_pos: 0.,
            draw_inc: 64.,

            ground: vec![],
            enemies: vec![],
            total_enemies: 0,
            barrels: vec![],

            player: None,
            end: None,
        }
    }

    pub fn parse(&mut self, map: String, asset_manager: &AssetManager) {
        for line in map.split("\n").collect::<Vec<_>>() {
            let exp = line.split(" ").collect::<Vec<_>>();

            if exp[0].starts_with(".end") {
                self.end = Some(exp[1..].join(" "));
            } else {
                for id in line.chars() {
                    match id {
                        '[' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::LEFT);

                            self.draw_inc = tile.width();
                            self.ground.push(tile);

                            self.draw_pos += self.draw_inc;
                        }

                        '-' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::CENTER);

                            self.draw_inc = tile.width();
                            self.ground.push(tile);

                            self.draw_pos += self.draw_inc;
                        }

                        ']' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::RIGHT);

                            self.draw_inc = tile.width();
                            self.ground.push(tile);

                            self.draw_pos += self.draw_inc;
                        }

                        '_' => {
                            self.draw_inc = 100.0;
                            self.draw_pos += self.draw_inc;
                        }

                        '8' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::CENTER);

                            self.draw_inc = tile.width();

                            self.ground.push(tile);
                            self.enemies.push(Enemy::new(self.draw_pos, asset_manager));

                            self.draw_pos += self.draw_inc;
                            self.total_enemies += 1;
                        }

                        '4' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::CENTER);

                            self.draw_inc = tile.width();

                            self.ground.push(tile);
                            self.player = Some(Player::new(self.draw_pos));

                            self.draw_pos += self.draw_inc;
                        }

                        '*' => {
                            let tile = Tile::new(self.draw_pos, asset_manager, TileType::CENTER);

                            self.draw_inc = tile.width();

                            self.ground.push(tile);
                            self.barrels.push(Barrel::new(self.draw_pos, asset_manager));

                            self.draw_pos += self.draw_inc;
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
