use failure::Error;
use ruswords::SwordsComic;

fn main() -> Result<(), Error> {
    let swords = SwordsComic::default();
    let comic = swords.get_comic(2)?;

    println!("{:#?}", comic);
    Ok(())
}