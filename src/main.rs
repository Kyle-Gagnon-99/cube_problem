use clap::{ArgAction, Arg, command};
use kiss3d::{window::Window, nalgebra};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let matches = command!()
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .help("Sets the level of verbosity")
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new("visualize")
            .long("visualize")
            .help("Visualize the output")
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    // If the verbose flag is set, set the environment filter to debug, otherwise set it to info
    let verbose = matches.get_flag("verbose");
    let env_filter = match verbose {
        true => "debug",
        false => "info",
    };

    // Set up the tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(env_filter)
        .finish();

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut window = Window::new("Kiss3d: cube");

    let mut cube = window.add_cube(1.0, 1.0, 1.0);

    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(kiss3d::light::Light::StickToCamera);

    while window.render() {
        cube.prepend_to_local_rotation(&nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::y_axis(), 0.01));
    }

    info!("Starting up");
}
