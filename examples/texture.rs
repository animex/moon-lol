use std::fs::File;
use std::io::{BufReader, Cursor, Read};

use binrw::BinRead;
use league_file::LeagueTexture;

fn main() {
    let path = "assets/ASSETS/Characters/Fiora/Skins/Base/Particles/Fiora_Base_Passive_color-rampdown32.tex";
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    let len = buf.len();
    println!("{:?}", len);
    let mut reader = Cursor::new(buf);
    let texture = LeagueTexture::read(&mut reader).unwrap();
    println!("{:?}", texture);
}
