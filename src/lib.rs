#[macro_use] extern crate lazy_static;

mod memes;

lazy_static! {
    pub static ref MEMES: Vec<&'static str> = {
        memes::get_memes()
    };
}
