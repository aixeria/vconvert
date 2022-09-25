use std::{time::{Instant, Duration}, fs::File, io::Write};

use vconvert::{vulnus,soundspace};

fn main() {
    let nw = Instant::now();

    let mut vulnus_map = serde_json::from_slice::<vulnus::Map>(include_bytes!("../vmap.json")).expect("unable to parse map");

    vulnus_map.set_offset(Duration::from_millis(100));

    vulnus_map.resize(5);

    File::create("vmap-out.json").expect("unable to create file").write_all(serde_json::to_string(&vulnus_map).expect("unable to serialize map").as_bytes()).expect("unable to write to file");
    
    let ss_map : soundspace::Map = vulnus_map.into();

    File::create("ssmap-out.txt").expect("unable to create file").write_all(ss_map.to_string().as_bytes()).expect("unable to write to file");

    println!("took {:?}", nw.elapsed());
}
