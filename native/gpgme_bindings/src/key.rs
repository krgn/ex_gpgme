use atoms;
use context::GpgmeContext;
use gpgme::KeyListMode;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

pub(crate) struct GpgmeKey(pub gpgme::Key);

unsafe impl Send for GpgmeKey {}
unsafe impl Sync for GpgmeKey {}

pub fn list<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut ctx = res.0.lock().unwrap();

    let mut mode = KeyListMode::empty();
    mode.insert(KeyListMode::LOCAL);

    let mut key_list = vec![];

    ctx.set_key_list_mode(mode).expect("key list mode");
    let mut keys = ctx.keys().expect("keys");

    for key in keys.by_ref().filter_map(|x| x.ok()) {
        for user in key.user_ids() {
            println!("numkeys: {:?}", user.id());
        }

        let key_ref = ResourceArc::new(GpgmeKey(key));

        key_list.push(key_ref)
    }

    Ok((atoms::ok(), key_list).encode(env))
}

pub fn key_id<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeKey> = args[0].decode()?;
    Ok((atoms::ok(), res.0.id().unwrap_or("?")).encode(env))
}

pub fn list_old<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
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
