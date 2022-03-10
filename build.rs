// https://github.com/ajour/ajour/blob/master/build.rs
extern crate embed_resource;

fn main() {
    #[cfg(windows)]
    embed_resource::compile("resources/windows/res.rc");
}
