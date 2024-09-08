pub mod hello_user;
pub use hello_user::*;
pub mod home;
pub use home::*;
mod create_user;
pub use create_user::*;

fn logging(path: &str){
    println!("{}", path);
}