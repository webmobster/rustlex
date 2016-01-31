
extern crate rustlex_codegen;
extern crate syntex;



pub use rustlex_codegen::rt;


pub fn plugin_registrar(reg: &mut syntex::Registry) {
    rustlex_codegen::plugin_registrar(reg);
}
