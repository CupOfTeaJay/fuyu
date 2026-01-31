///! TODO: Document.

use hachiya::DynamicApp;

#[stabby::export]
pub extern "C" fn main(_app: stabby::boxed::Box<DynamicApp>) {
    println!("hello from minimal.so! again!");
}
