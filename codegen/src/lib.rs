extern crate quasi;
extern crate syntex;
extern crate syntex_syntax as syntax;



#[macro_use] extern crate log;
extern crate bit_set;
extern crate fsa;

use syntax::ast::{Ident, TokenTree};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult};


mod analysis;
mod lexer;
mod parser;
mod regex;
mod unicode;
pub mod rt;

// the main rustlex macro
pub fn rustlex<'a>(cx: &'a mut ExtCtxt, sp: Span, ident:Ident, args: Vec<TokenTree>)
        -> Box<MacResult+'a> {
    let mut p = ::syntax::parse::new_parser_from_tts(
        cx.parse_sess,
        cx.cfg.clone(),
        args
    );

    let def = parser::parse(ident, &mut p)
        .unwrap_or_else( |_| panic!("error while parsing lexer"));
    let lex = lexer::Lexer::new(def, cx);
    lex.gen_code(cx, sp)
}

pub fn plugin_registrar(reg: &mut syntex::Registry) {
    reg.add_ident_macro("rustlex", rustlex);
}
