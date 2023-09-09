pub mod blocklist;
pub mod constants;
pub mod discordhelpers;
pub mod spotify;


#[macro_export]
macro_rules! quote {
    ($target:expr, $($arg:tt)+) => {
        format!(">>> {}", format!($target,  $($arg)+))
    };

    ($($arg:tt)+) => {
        format!(">>> {}", format!("{}", $($arg)+))
    };
}

// #[macro_export]
// macro_rules! multi_quote {
//     ($target:expr, $($arg:tt)+) => {
//         format!(">>> {}", format!($target,  $($arg)+))
//     };
//
//     ($($arg:tt)+) => {
//         format!(">>> {}", format!("{}", $($arg)+))
//     };
// }

#[macro_export]
macro_rules! url {
    ($target:tt, $arg:tt) => {
        format!("[{}]({})", $target,  $arg)
    };
}



#[tokio::test]
async fn it_works() {
    use super::*;

    println!("{}", quote!("test"));
    println!("{}", quote!("test {}", "123"));
    println!("{}", url!("test", "https://wandbox.org/"));
}
