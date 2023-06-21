use std::io;


fn main() -> io::Result<()> {

    //Take input from console
    let stdin = io::stdin();

    println!("Welcome to To-day planner!");
    let mut activity = String::new();
    let mut start_time = String::new();
    let mut duration = String::new();
    let mut filledHours: Vec<i32> = vec![0;0];

    let mut i = 1;
    while i < 24 {
        println!("Please enter activity!");
        stdin.read_line(&mut activity);
    
        let mut actvity_start_time: i32 = 0;
        let h = 0;
        while h < 10 {
            println!("Please enter time!");
            stdin.read_line(&mut start_time);
            actvity_start_time = start_time.to_string().parse::<i32>().unwrap();
            if !filledHours.contains(&actvity_start_time) {
                break;
            }
            println!("This hour is already filled!");
        }

        let mut activity_duration: i32 = 0;
        let l = 0;
        while l < 10 {
            println!("Please enter duration!");
            stdin.read_line(&mut duration);
            activity_duration = duration.to_string().parse::<i32>().unwrap();

            let mut isExisting = true;
            let p = 1;
            while p <= activity_duration {
                let time = &actvity_start_time + &p;
                if filledHours.contains(&time) {
                    println!("Activity duration overlaps other activity!");
                    break;
                } else {
                    isExisting = false;
                }

            }
            if !isExisting {
                break;
            }
        }
        
        println!("Activity of {} will start in {}.00 and will last {} hours",activity,start_time,duration);

        
        filledHours.push(actvity_start_time);
        
        
        let k = 0;
        while k <  activity_duration{
            let addition: i32 = &k + &actvity_start_time;
            filledHours.push(addition);
        }



        let mut is_continued = String::new();
        let mut j = 1;
        while j < 4 {
            if j > 1 {
                println!("Please type Y or N");
            }
            
            println!("Would you like to continue adding activities? Y or N");
            stdin.read_line(&mut is_continued);
            if is_continued.trim().eq("Y") || is_continued.trim().eq("N") {
                break;
            }
            j = j + 1;
        }
        if is_continued.trim().eq("N") {
            break;
        }
        if filledHours.len() > 23 {
            break;
        }

        i = i + 1;
    }
    
  



    




    Ok(())
}
