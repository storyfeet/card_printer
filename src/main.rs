extern crate lazyf;
extern crate svg;

use lazyf::cfg::Cfg;

fn main() {

    let cfg = Cfg::load_first("-c","conf.lz");
    println!("Hello, world!");
}
