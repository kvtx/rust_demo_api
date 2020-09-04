use crate::models::demo::Demos;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Collect {
    count: i32,
    average: i32,
    all_text: Vec<String>,
    all_names: Vec<String>,
    favorite_favorite: i32,
}

pub fn demolition(res: Result<Vec<Demos>, Error>) -> Result<Collect, Error> {
    let mut collection: Collect = Default::default();
    let mut find_favorite: HashMap<i32, i32> = HashMap::new();
    let mut total: i32 = 0;
    match res {
        Ok(demos) => {
            demos.iter().for_each(|d| {
                total = total + d.favorite_number;
                collection.all_text.push(String::from(&*d.demo_text));
                collection.all_names.push(String::from(&*d.name));
                if let Some(x) = find_favorite.get(&d.favorite_number) {
                    let new_count = x + 1;
                    find_favorite.insert(d.favorite_number, new_count);
                } else {
                    find_favorite.insert(d.favorite_number, 1);
                }
            });
            collection.average = total / (demos.len() as i32);
            collection.favorite_favorite = 0;
            for (number, count) in find_favorite.iter() {
                if count > &collection.favorite_favorite {
                    collection.favorite_favorite = *count;
                }
            }
            Ok(collection)
        }
        Err(err) => Err(err),
    }
}
