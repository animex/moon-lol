use bevy::math::vec3;
use moon_lol::{
    core::{find_path, Configs},
    league::get_struct_from_file,
    logging::setup_file_logging,
};

fn main() {
    let log_path = "moon_lol.log".to_string().into();

    // Set up file logging
    setup_file_logging(&log_path);

    let configs: Configs = get_struct_from_file("configs").unwrap();
    let path = find_path(
        &configs,
        configs.navigation_grid.get_center_pos(),
        configs.navigation_grid.get_center_pos() + vec3(500.0, 0.0, -500.0),
    );

    println!("{:?}", path);
}
