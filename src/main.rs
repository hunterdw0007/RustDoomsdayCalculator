use std::io;

fn main() -> io::Result<()>{
    doomsday_calc::print_instructions();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
