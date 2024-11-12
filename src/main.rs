use minibevy::Resource;
use minirapier::Ray;

pub struct Commands;
impl Commands {
    fn insert_resource<R: Resource>(&self, _resource: R) {}
}
pub fn setup_cursor(commands: Commands) {
    commands.insert_resource(Res.into());
}

struct Res;
impl Resource for Res {}

fn main() {}
