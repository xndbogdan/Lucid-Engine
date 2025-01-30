use anyhow::Result;
use glam::Vec2;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vec2Def {
    x: f32,
    y: f32,
}

impl From<Vec2Def> for Vec2 {
    fn from(v: Vec2Def) -> Self {
        Vec2::new(v.x, v.y)
    }
}

#[derive(Debug, Deserialize)]
pub struct MapDef {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub layout: Vec<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct EnemyProperties {
    pub health: i32,
    pub damage: i32,
    pub speed: f32,
    pub attack_range: f32,
    pub chase_range: f32,
}

#[derive(Debug, Deserialize)]
pub struct EnemyDef {
    #[serde(rename = "type")]
    pub enemy_type: String,
    pub position: Vec2Def,
    pub patrol_points: Vec<Vec2Def>,
    pub properties: EnemyProperties,
}

#[derive(Debug, Deserialize)]
pub struct PlayerDef {
    pub spawn: Vec2Def,
    pub direction: Vec2Def,
}

#[derive(Debug, Deserialize)]
pub struct MetadataDef {
    pub author: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub map: MapDef,
    pub enemies: Vec<EnemyDef>,
    pub player: PlayerDef,
    pub metadata: MetadataDef,
}

impl MapFile {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let map_file: MapFile = toml::from_str(&content)?;

        // Validate map dimensions
        if map_file.map.layout.len() != map_file.map.height {
            anyhow::bail!("Map height mismatch");
        }
        for row in &map_file.map.layout {
            if row.len() != map_file.map.width {
                anyhow::bail!("Map width mismatch");
            }
        }

        Ok(map_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let map = MapFile::load("assets/maps/test.toml").unwrap();
        assert_eq!(map.map.width, 8);
        assert_eq!(map.map.height, 8);
        assert_eq!(map.enemies.len(), 1);
        assert_eq!(map.enemies[0].enemy_type, "ranged");
        assert_eq!(map.enemies[0].patrol_points.len(), 4);
    }
}
