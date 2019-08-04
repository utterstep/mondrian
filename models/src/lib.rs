#[macro_use]
extern crate diesel;

pub mod schema;
pub mod tasks;
pub mod users;

mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
