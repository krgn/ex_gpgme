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
     ("key_user_ids", 1, key::key_user_ids),
     ("key_subkeys", 1, key::key_subkeys),
     ("key_fingerprint", 1, key::key_fingerprint),
     ("key_can_encrypt", 1, key::key_can_encrypt),
     ("key_can_sign", 1, key::key_can_sign),
     ("key_can_certify", 1, key::key_can_certify),
     ("key_can_authenticate", 1, key::key_can_authenticate),
     ("key_has_secret"  , 1, key::key_has_secret)  ,
     ("key_is_revoked"  , 1, key::key_is_revoked)  ,
     ("key_is_expired"  , 1, key::key_is_expired)  ,
     ("key_is_disabled" , 1, key::key_is_disabled) ,
     ("key_is_invalid"  , 1, key::key_is_invalid)  ,
     ("key_is_qualified", 1, key::key_is_qualified),
     ("context_create", 2, context::create),
     ("context_import", 2, context::import),
     ("context_encrypt", 3, context::encrypt),
     ("context_decrypt", 3, context::decrypt),
     ("context_info", 1, context::info)],
    Some(on_load)
}

pub fn on_load<'a>(env: Env<'a>, args: Term<'a>) -> bool {
    println!("Loading ex_gpgme with args: {:?}", args);
    resource_struct_init!(GpgmeContext, env);
    resource_struct_init!(GpgmeKey, env);
    true
}
