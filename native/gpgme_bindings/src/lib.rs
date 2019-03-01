#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate gpgme;

use gpgme::{Context, KeyListMode, Protocol};
use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.ExGpgme.Bindings",
    [("list_keys", 0, list_keys)],
    None
}

fn list_keys<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let proto = Protocol::OpenPgp;

    let mut mode = KeyListMode::empty();
    mode.insert(KeyListMode::LOCAL);

    let mut ctx = Context::from_protocol(proto).expect("context");

    ctx.set_key_list_mode(mode).expect("key list mode");
    let mut keys = ctx.keys().expect("keys");
    for key in keys.by_ref().filter_map(|x| x.ok()) {
        println!("keyid   : {}", key.id().unwrap_or("?"));
        println!("fpr     : {}", key.fingerprint().unwrap_or("?"));
        println!(
            "caps    : {}{}{}{}",
            if key.can_encrypt() { "e" } else { "" },
            if key.can_sign() { "s" } else { "" },
            if key.can_certify() { "c" } else { "" },
            if key.can_authenticate() { "a" } else { "" }
        );
        println!(
            "flags   :{}{}{}{}{}{}",
            if key.has_secret() { " secret" } else { "" },
            if key.is_revoked() { " revoked" } else { "" },
            if key.is_expired() { " expired" } else { "" },
            if key.is_disabled() { " disabled" } else { "" },
            if key.is_invalid() { " invalid" } else { "" },
            if key.is_qualified() { " qualified" } else { "" }
        );
        for (i, user) in key.user_ids().enumerate() {
            println!("userid {}: {}", i, user.id().unwrap_or("[none]"));
            println!("valid  {}: {:?}", i, user.validity())
        }
        println!("");
    }

    Ok(atoms::ok().encode(env))
}
