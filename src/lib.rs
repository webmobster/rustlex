#![cfg_attr(not(feature = "with-syntex"), feature(plugin_registrar,rustc_private))]

extern crate rustlex_codegen;
#[cfg(not(feature = "with-syntex"))] extern crate rustc_plugin;
#[cfg(feature = "with-syntex")] extern crate syntex;



pub use rustlex_codegen::rt;


#[cfg(feature = "with-syntex")]
pub fn plugin_registrar(reg: &mut syntex::Registry) {
    rustlex_codegen::plugin_registrar(reg);
}

#[plugin_registrar]
#[cfg(not(feature = "with-syntex"))]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    rustlex_codegen::plugin_registrar(reg);
}
