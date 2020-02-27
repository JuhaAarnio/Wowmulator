use std::io;
fn main() {
    let mut maximum_hitpoints = String::new();
    println!("Type in your maximum hitpoints");
    io::stdin().read_line(&mut maximum_hitpoints).expect("Failed to read the value");
    let maximum_hitpoints : f32 = maximum_hitpoints.trim().parse().unwrap();

    let mut stagger_percentage = String::new();
    println!("Type in your stagger percentage");
    io::stdin().read_line(&mut stagger_percentage).expect("Failed to read the value");
    let stagger_percentage : f32 = stagger_percentage.trim().parse().unwrap();
    if stagger_percentage < 1.0 {
        println!("Incorrect stagger percentage, please input stagger as full percentage value, e.g. 
        75, not 0.75");
    }
    if stagger_percentage > 100.0 {
        println!("Abnormally high stagger percentage, please check your input");
    }
    let stagger_percentage = stagger_percentage / 100.0;

    println!("Type in damage taken per second");
    let mut damage_taken_per_second = String::new();
    io::stdin().read_line(&mut damage_taken_per_second).expect("Failed to read the value");
    let damage_taken_per_second : f32 = damage_taken_per_second.trim().parse().unwrap();
    if damage_taken_per_second == 0.0 {
        println!("No damage taken, simulation might never finish!");
    }
    println!("{}:{}",damage_taken_per_second, stagger_percentage);
    let maximum_stagger = (damage_taken_per_second / stagger_percentage) * 10.0;
    let stagger_cap = maximum_hitpoints * 10.0;
    let mut current_stagger : f32 = 0.0;
    let mut time : u8 = 0;

    while current_stagger < maximum_stagger {
        time = time + 1;
        let previous_stagger : f32 = current_stagger;
        if current_stagger <= stagger_cap {
            current_stagger += damage_taken_per_second * stagger_percentage; 
            current_stagger = current_stagger - current_stagger / 10.0;
            println!("{}",current_stagger as u32);  
        }
        else {
            current_stagger = stagger_cap;
            println!("Hit stagger cap!");
        }
        if current_stagger == previous_stagger || current_stagger == stagger_cap {
            println!("Reached maximum stagger of {} in {} seconds", current_stagger as u32, time);
            break;
        }
    }
}
