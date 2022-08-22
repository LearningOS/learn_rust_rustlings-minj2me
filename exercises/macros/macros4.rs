// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    ()=>{
        println!("Check out my macro!");
    };
    ($e:expr) =>{
        println!("Look at this other macro: {}", $e);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
