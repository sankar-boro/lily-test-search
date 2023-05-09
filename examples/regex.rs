use fst::automaton::Subsequence;
use fst::{IntoStreamer, Streamer, Map};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::from_iter(vec![
        ("arun", 1),
        ("boro", 2),
        ("lily", 3),
        ("sankar", 4)
    ]).unwrap();

    let matcher = Subsequence::new("r");
    let mut stream = map.search(&matcher).into_stream();

    let mut kvs = vec![];
    while let Some((k, v)) = stream.next() {
        kvs.push((String::from_utf8(k.to_vec())?, v));
    }
    println!("{:?}", kvs);
    // assert_eq!(kvs, vec![
    //     ("a foo bar".to_string(), 1), ("foobar".to_string(), 6),
    // ]);

    Ok(())
}