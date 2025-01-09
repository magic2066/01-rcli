mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::process_csv;

#[cfg(test)]
mod tests {

    #[test]
    fn example_test() {
        assert_eq!(2 + 2, 4);
    }
}
