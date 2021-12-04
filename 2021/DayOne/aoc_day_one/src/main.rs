mod sonar;
mod data;

fn main() {
    board_ship();
    map_depth_chart();
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
    println!("\tChecking sonar pings... \n\t...\n\t...\n\t...\n");
    let readings = data::get_sonar_readings();
    println!("\tNumber of readings: {}", readings.iter().count());
    println!("\tNumber of fuzzy increases: {}", sonar::get_depth_increases(readings.clone()));
    println!("\tNumber of precise increases: {}\n\n", sonar::get_depth_increase_with_precision(readings.clone()));
}