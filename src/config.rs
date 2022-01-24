
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct UserConfig {
    bar: bool,
    screens: Vec<Screen>,
}

struct Screen {
    stacks: u8,
    columns: Vec<u8>,
}
