struct Health(i32);
struct Name(&'static str);

struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}

impl World {
    fn new() -> Self {
        Self {
            health_components: Vec::new(),
            name_components: Vec::new(),
        }
    }

    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }
}

fn main() {
    let mut world = World::new();

    // icarus health is not looking good
    world.new_entity(Some(Health(-10)), Some(Name("Icarus")));

    // prometheus is very healthy
    world.new_entity(Some(Health(100)), Some(Name("Prometheus")));

    // note that zeus does not have a health component
    world.new_entity(None, Some(Name("Zeus")));
}
