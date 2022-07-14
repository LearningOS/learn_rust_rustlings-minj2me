// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

macro_rules! my_macro {
    ($str:expr)=>{
        {
            let mut string = String::from("Hello");
            string.push_str(" ");
            string.push_str($str);
            string
        }
    };
}

/*macro_rules! add_as {
    (
       //repeated block
        $($a:expr)
        //seperator
        ,
        //zero or more fields
        *
    )=>{
        {
            //to handler the case without any arguments
            0
            //block to be repeated
            $(+$a)*
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        //println!("{}",my_macro!("world!"));
        //assert_eq!(1,1);
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
