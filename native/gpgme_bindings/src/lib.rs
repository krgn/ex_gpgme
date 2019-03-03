#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate gpgme;

use gpgme::{Context, KeyListMode, Protocol};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::Mutex;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

mod proto {
    rustler_atoms! {
        atom openpgp;
    }
}

rustler_export_nifs! {
    "Elixir.ExGpgme.Bindings",
    [("list_keys", 1, list_keys),
     ("create_context", 0, create_context)],
    Some(on_load)
}

pub(crate) struct GpgmeContext(Mutex<Context>);

unsafe impl Send for GpgmeContext {}
unsafe impl Sync for GpgmeContext {}

pub fn on_load<'a>(env: Env<'a>, _: Term<'a>) -> bool {
    resource_struct_init!(GpgmeContext, env);
    true
}

fn create_context<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let proto = Protocol::OpenPgp;
    let ctx = Context::from_protocol(proto).expect("context");
    let resource = ResourceArc::new(GpgmeContext(Mutex::new(ctx)));

    Ok((atoms::ok(), resource).encode(env))
}

fn list_keys<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut ctx = res.0.lock().unwrap();

    let mut mode = KeyListMode::empty();
    mode.insert(KeyListMode::LOCAL);

    let mut key_list = vec![];

    ctx.set_key_list_mode(mode).expect("key list mode");
    let mut keys = ctx.keys().expect("keys");
    for key in keys.by_ref().filter_map(|x| x.ok()) {
        let mut map = Term::map_new(env);

        map = map.map_put("keyid".encode(env), key.id().unwrap_or("?").encode(env))?;
        map = map.map_put(
            "fpr".encode(env),
            key.fingerprint().unwrap_or("?").encode(env),
        )?;
        map = map.map_put("can_encrypt".encode(env), key.can_encrypt().encode(env))?;
        map = map.map_put("can_sign".encode(env), key.can_sign().encode(env))?;
        map = map.map_put("can_certify".encode(env), key.can_certify().encode(env))?;
        map = map.map_put(
            "can_authenticate".encode(env),
            key.can_authenticate().encode(env),
        )?;
        map = map.map_put("has_secret".encode(env), key.has_secret().encode(env))?;
        map = map.map_put("is_revoked".encode(env), key.is_revoked().encode(env))?;
        map = map.map_put("is_expired".encode(env), key.is_expired().encode(env))?;
        map = map.map_put("is_disabled".encode(env), key.is_disabled().encode(env))?;
        map = map.map_put("is_invalid".encode(env), key.is_invalid().encode(env))?;
        map = map.map_put("is_qualified".encode(env), key.is_qualified().encode(env))?;

        let mut users = vec![];

        for (_i, user) in key.user_ids().enumerate() {
            let mut user_map = Term::map_new(env);
            user_map = user_map.map_put(
                "userid".encode(env),
                user.id().unwrap_or("[none]").encode(env),
            )?;
            user_map =
                user_map.map_put("valid".encode(env), user.validity().to_string().encode(env))?;
            users.push(user_map);
        }
        map = map.map_put("users".encode(env), users.encode(env))?;

        key_list.push(map);
    }

    Ok((atoms::ok(), key_list).encode(env))
}
