use std::fs::read_to_string;
use bevy_reflect::TypeRegistry;

async fn load_configs<'a, P: AsRef<Path>>(path: P) -> Result<Vec<EntityConfig>, Box<dyn std::error::Error>> {
    let content = read_to_string(path).await?;
    ron::de::from_str(&content)
}