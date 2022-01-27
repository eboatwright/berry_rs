use hecs::World;

pub struct Scene {
	pub name: &'static str,
	pub world: World,
}

impl Default for Scene {
	fn default() -> Scene {
		Scene {
			name: "default",
			world: World::new(),
		}
	}
}

impl Scene {
	pub fn new(name: &'static str) -> Scene {
		Scene {
			name,
			world: World::new(),
		}
	}
}