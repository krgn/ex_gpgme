#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate gpgme;

mod atoms;
mod context;
mod key;
mod protocol;

use context::GpgmeContext;
use key::GpgmeKey;
use rustler::{Env, Term};

rustler_export_nifs! {
    "Elixir.ExGpgme.Native",
    [("key_list", 1, key::list),
     ("key_id", 1, key::key_id),
     ("context_create", 2, context::create),
     ("context_info", 1, context::info)],
    Some(on_load)
}

pub fn on_load<'a>(env: Env<'a>, args: Term<'a>) -> bool {
    println!("Loading ex_gpgme with args: {:?}", args);
    resource_struct_init!(GpgmeContext, env);
    resource_struct_init!(GpgmeKey, env);
    true
}
