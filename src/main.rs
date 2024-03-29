use std::io;
mod procedures;


fn main() {
    println!("\n         {}   {}   {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

    let mut cmd = String::new();
    loop {
        print!("   &>   ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut cmd).unwrap();
        println!("    {}", procedures::calculate_expression(&cmd[0..cmd.len()-2]));
        cmd.clear();
    }
}
