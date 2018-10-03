#[macro_use]
extern crate log;

extern crate dirs;
extern crate id3;
extern crate postgres;

pub mod music_manager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
