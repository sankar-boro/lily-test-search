use fst::{IntoStreamer, Set};
use fst::automaton::Levenshtein;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // A convenient way to create sets in memory.
  let keys = vec!["sanjay", "sankar", "sankkr", "sanmar", "santos"];
  let set = Set::from_iter(keys)?;

  // Build our fuzzy query.
  let lev = Levenshtein::new("san", 3)?;

  // Apply our fuzzy query to the set we built.
  let stream = set.search(lev).into_stream();

  let keys = stream.into_strs()?;
  println!("{:?}", keys);
  Ok(())
}