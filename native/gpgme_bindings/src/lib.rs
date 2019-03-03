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

use context::GpgmeContext;
use key::GpgmeKey;
use rustler::{Env, Term};

rustler_export_nifs! {
    "Elixir.ExGpgme.Bindings",
    [("list_keys", 1, key::list),
     ("key_id", 1, key::key_id),
     ("create_context", 0, context::create)],
    Some(on_load)
}

pub fn on_load<'a>(env: Env<'a>, _: Term<'a>) -> bool {
    resource_struct_init!(GpgmeContext, env);
    resource_struct_init!(GpgmeKey, env);
    true
}
