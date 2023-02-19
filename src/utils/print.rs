pub fn print_console(
    message: &str,
    severity: i8,
)
{
    //we don't even need to pass a preprocess directive it juust fucking does it for us 
    //i love rust
    let location = std::panic::Location::caller();
    println!("Called from file {} line {}", location.file(), location.line());

    match severity {
        0 => { println!("!!(ERROR)!!") }
        1 => { println!("<(WARNING)>") }
        2 => { println!("(INFO)") }
        _ => { println!("Invalid severity parameter.") }
    }

    println!("{}", message);
}