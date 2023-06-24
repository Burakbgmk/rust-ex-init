use std::io;


fn main() -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");

    //Take input from console
    let stdin = io::stdin();

    let mut activities: Vec<String> = vec![];

    println!("Welcome to To-day planner!");
    let mut filled_hours: Vec<i32> = vec![];
    
    let mut i = 1;
    while i < 24 {
        let mut activity = String::new();
        let mut start_time = String::new();
        let mut duration = String::new();
        println!("Please enter activity!");
        stdin.read_line(&mut activity);
        print!("\x1B[2J\x1B[1;1H");
    
        let mut actvity_start_time: i32 = 0;
        let h = 0;
        while h < 10 {
            println!("Please enter time!");
            stdin.read_line(&mut start_time);
            print!("\x1B[2J\x1B[1;1H");
            actvity_start_time = start_time.trim().to_string().parse::<i32>().unwrap();
            if !filled_hours.contains(&actvity_start_time) {
                break;
            }
            if actvity_start_time < 0 || actvity_start_time > 24 {
                println!("Type an hour between 0-24");
            }
            else {
                println!("This hour is already filled!");
            }
            start_time.clear();
        }

        let mut activity_duration: i32 = 0;
        let l = 0;
        while l < 10 {
            println!("Please enter duration!");
            stdin.read_line(&mut duration);
            print!("\x1B[2J\x1B[1;1H");
            activity_duration = duration.trim().to_string().parse::<i32>().unwrap();

            let mut is_existing = true;
            let mut p = 1;
            while p <= activity_duration {
                let time = &actvity_start_time + &p - 1;
                if filled_hours.contains(&time) {
                    println!("Activity duration overlaps other activity!");
                    duration.clear();
                    break;
                } else if time > 24 {
                    println!("Activity duration beyond today's limit!");
                    duration.clear();
                    break;
                } else {
                    is_existing = false;
                }
                p += 1;
            }
            if !is_existing {
                break;
            }
        }
        
        let acts = format!("Activity of {} will start in {}.00 and will last {} hours",activity,start_time,duration);
        activities.push(acts);
        println!("Activity of {} will start in {}.00 and will last {} hours",activity,start_time,duration);

        
        filled_hours.push(actvity_start_time);
        
        
        let mut k = 1;
        while k <  activity_duration{
            let addition: i32 = &k + &actvity_start_time;
            filled_hours.push(addition);
            k += 1;
        }



        let mut is_continued = String::new();
        let mut j = 1;
        while j < 4 {
            if j > 1 {
                println!("Please type Y or N");
            }
            
            println!("Would you like to continue adding activities? Y or N");
            stdin.read_line(&mut is_continued);
            print!("\x1B[2J\x1B[1;1H");
            if is_continued.trim().eq("Y") || is_continued.trim().eq("N") {
                break;
            }
            j = j + 1;
        }
        if is_continued.trim().eq("N") {
            break;
        }
        if filled_hours.len() > 23 {
            break;
        }

        i = i + 1;
    }
    


    for act in activities {
        println!("{}",act);
    }

    




    Ok(())
}
