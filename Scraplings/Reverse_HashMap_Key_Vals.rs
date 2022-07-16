// My main problem with this code is that the length of the HashMap (and so the array holding it's key-value pairs) must be a constant
// and so must be known at compile time and can't be evaluated with a simple `.len()`
// I'm wondering if there's a way to have a code that reverses the key-value pairs of a HashMap without the size of the HashMap being a constant.
// Done various researches and couldn't find any code implementation of anything similar online nor could I find a built-in method to reverse the key-value pairs
// Any help (even if it's just about polishing the current code without actually working on the above issue above is apprecited).

use std::collections::HashMap;
use std::convert::TryInto;

type StrHash<'a> = HashMap<&'a str, &'a str>; // Type alias so I don't have to type it out all again later

const HASH_LEN: usize = 3;
const KEY_VALS: [(&str, &str); HASH_LEN] = [
  ("key 1", "value 1"),
  ("key 2", "value 2"),
  ("key 3", "value 3"),
];

/*  HashMap::from() only accepts an array of tuples but to reverse the tuples values I have to iterate over it
    and then collect the values into a vector, that's the sole purpose of this function; to convert a vector to an fixed size array.
*/
fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
  v.try_into()
    .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// Function used to reverse the tuple's values
fn rev_tuple<'a>(tuple: (&'a str, &'a str)) -> (&'a str, &'a str) {
  (tuple.1, tuple.0)
}

/*  Function used to reverse the HashMap
    I clone the HashMap to not have the type of my the HashMap set as mut
*/
fn rev_hash_map<'a>(hash_map: StrHash) -> StrHash {
  let mut cloned_hash = hash_map.clone();
  let data: [(&str, &str); HASH_LEN] = vec_to_arr(
    cloned_hash
      .drain()
      .into_iter()
      .map(|x| rev_tuple(x))
      .collect(),
  );
  return HashMap::from(data);
}

fn main() {
  let hash: StrHash = HashMap::from(KEY_VALS);
  println!("{:#?}", rev_hash_map(hash));
}
