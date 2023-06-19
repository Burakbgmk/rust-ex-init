use std::io;


fn main() -> io::Result<()> {

    //Take input from console
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input);

    println!("input {} ", user_input);




    Ok(())
}
