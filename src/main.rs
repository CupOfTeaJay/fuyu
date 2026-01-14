use hachiya::cargo::{Manifest, get_manifest};

fn main() {
    let manifest: Manifest = get_manifest("examples/mods");
    println!("{:#?}", manifest);
}
