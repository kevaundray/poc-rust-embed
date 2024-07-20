use rust_embed::Embed;

#[derive(Embed)]
#[folder = "data"]
struct EmbeddedData;

fn main() {
    const DUMMY_TEXT_FILE: &str = "dummy_file.txt";

    let file =
        EmbeddedData::get(DUMMY_TEXT_FILE).expect("expected the file to be embedded in the binary");
    println!(
        "Printing file metadata: {:?}",
        file.metadata.last_modified()
    );
}
