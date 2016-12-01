use std;
use inquirer::list;
use inquirer::Error::UserAborted;

pub fn select_item<'a, T>(header: &'a str, items: Vec<(&'a String, &'a T)>) -> &'a T {
    if items.len() == 1 {
        return &items[0].1;
    }

    match list(header, &items) {
        Ok(result) => result,
        Err(UserAborted) => {
            std::process::exit(1);
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
