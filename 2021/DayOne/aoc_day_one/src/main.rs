mod sonar;
mod navigation;
mod data;

fn main() {
    board_ship();
    map_depth_chart();
    plot_destination_position();
}

fn board_ship() {
    println!("\n\n\t\tAdvent of Code 2021!\n\n");
    println!("\t\t                             |`-:_");
    println!("\t\t    ,----....____            |    `+.");
    println!("\t\t   (             ````----....|___   |");
    println!("\t\t    \\     _                      ````----....____");
    println!("\t\t     \\    _)                                     ```---.._");
    println!("\t\t      \\                                                   \\");
    println!("\t\t    )`.\\  )`.   )`.   )`.   )`.   )`.   )`.   )`.   )`.   )`.   )`.");
    println!("\t\t  -'   `-'   `-'   `-'   `-'   `-'   `-'   `-'   `-'   `-'   `-'   `");
    println!("\t\t\tASCII art by shimrod on https://ascii.co.uk/art/submarine");
    println!("\n\n\t\tWelcome aboard, Captain. Let's do some Christmas themed nautical nonsense.\n\n\n")
}

fn map_depth_chart() {
    println!("Day One: Measure the Deep\n");
    println!("\tChecking sonar pings... \n\t\t...\n\t\t...\n\t\t...");
    let readings = data::get_sonar_readings();
    println!("\tNumber of readings: {}", readings.iter().count());
    println!("\tNumber of fuzzy increases: {}", sonar::get_depth_increases(readings.clone()));
    println!("\tNumber of precise increases: {}\n\n", sonar::get_depth_increase_with_precision(readings.clone()));
}

fn plot_destination_position() {
    println!("Day Two: Plot Destination\n");
    println!("\tEvaluating NavRoute... \n\t\t...\n\t\t...\n\t\t...");
    println!("\tDestination Coordinates: {}", navigation::calculate_end_position(data::get_nav_route()));
    println!("\t\t...\n\t\t...");
    println!("\tError! Destination outside navigable waters\n");
    println!("\tRecalculating...\n\t\t...\n\t\t...\n\t\t...");
    println!("\tFactoring in Control Upgrades...\n\t\t...\n\t\t...\n\t\t...");
    println!("\t\t...\n\t\t...");
    println!("\t*Actual* destination coordinates: {}\n\n", navigation::calculate_actual_end_position(data::get_nav_route()));
}