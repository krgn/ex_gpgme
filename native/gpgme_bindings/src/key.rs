use atoms;
use context::GpgmeContext;
use gpgme::KeyListMode;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

pub(crate) struct GpgmeKey(pub gpgme::Key);

unsafe impl Send for GpgmeKey {}
unsafe impl Sync for GpgmeKey {}

mod validity {
    use gpgme::Validity;

    rustler_atoms! {
        atom unknown;
        atom undefined;
        atom never;
        atom marginal;
        atom full;
        atom ultimate;
    }

    pub fn from(validity: Validity) -> rustler::types::atom::Atom {
        match validity {
            Validity::Unknown => unknown(),
            Validity::Undefined => undefined(),
            Validity::Never => never(),
            Validity::Marginal => marginal(),
            Validity::Full => full(),
            Validity::Ultimate => ultimate(),
        }
    }
}

mod key_origin {
    use gpgme::KeyOrigin;

    rustler_atoms! {
        atom unknown;
        atom key_server;
        atom dane;
        atom wkd;
        atom url;
        atom file;
        atom _self;
        atom other;
    }

    pub fn from(origin: KeyOrigin) -> rustler::types::atom::Atom {
        match origin {
            KeyOrigin::Unknown => unknown(),
            KeyOrigin::KeyServer => key_server(),
            KeyOrigin::Dane => dane(),
            KeyOrigin::Wkd => wkd(),
            KeyOrigin::Url => url(),
            KeyOrigin::File => file(),
            KeyOrigin::Self_ => _self(),
            KeyOrigin::Other(_) => other(),
        }
    }
}

mod user_id {
    rustler_atoms! {
        atom name;
        atom email;
        atom comment;
        atom validity;
        atom revoked;
        atom invalid;
        atom origin;
        atom tofu_info;
        atom signatures;
    }
}

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

pub fn key_fingerprint<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let fingerprint = key
        .0
        .fingerprint()
        .map(|f| f.encode(env))
        .unwrap_or(atoms::none().encode(env));
    Ok((atoms::ok(), fingerprint).encode(env))
}

pub fn key_can_encrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let can_encrypt = key.0.can_encrypt().encode(env);
    Ok((atoms::ok(), can_encrypt).encode(env))
}

pub fn key_can_sign<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let can_sign = key.0.can_sign().encode(env);
    Ok((atoms::ok(), can_sign).encode(env))
}

pub fn key_user_ids<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let mut list: Vec<Term<'a>> = Vec::new();

    for user_id in key.0.user_ids() {
        let mut map = Term::map_new(env);

        map = map.map_put(
            user_id::name().encode(env),
            user_id
                .name()
                .map(|s| s.encode(env))
                .unwrap_or(atoms::none().encode(env)),
        )?;

        map = map.map_put(
            user_id::email().encode(env),
            user_id
                .email()
                .map(|s| s.encode(env))
                .unwrap_or(atoms::none().encode(env)),
        )?;

        map = map.map_put(
            user_id::comment().encode(env),
            user_id
                .comment()
                .map(|s| s.encode(env))
                .unwrap_or(atoms::none().encode(env)),
        )?;

        map = map.map_put(
            user_id::revoked().encode(env),
            user_id.is_revoked().encode(env),
        )?;

        map = map.map_put(
            user_id::invalid().encode(env),
            user_id.is_invalid().encode(env),
        )?;

        map = map.map_put(
            user_id::origin().encode(env),
            key_origin::from(user_id.origin()).encode(env),
        )?;

        map = map.map_put(
            user_id::validity().encode(env),
            validity::from(user_id.validity()).encode(env),
        )?;

        map = map.map_put(user_id::tofu_info().encode(env), atoms::none().encode(env))?;

        let signatures: Vec<&str> = Vec::new();
        map = map.map_put(user_id::signatures().encode(env), signatures.encode(env))?;

        list.push(map)
    }
    Ok((atoms::ok(), list).encode(env))
}

// pub fn list_old<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//     let res: ResourceArc<GpgmeContext> = args[0].decode()?;
//     let mut ctx = res.0.lock().unwrap();

//     let mut mode = KeyListMode::empty();
//     mode.insert(KeyListMode::LOCAL);

//     let mut key_list = vec![];

//     ctx.set_key_list_mode(mode).expect("key list mode");
//     let mut keys = ctx.keys().expect("keys");
//     for key in keys.by_ref().filter_map(|x| x.ok()) {
//         let mut map = Term::map_new(env);

//         map = map.map_put("keyid".encode(env), key.id().unwrap_or("?").encode(env))?;
//         map = map.map_put(
//             "fpr".encode(env),
//             key.fingerprint().unwrap_or("?").encode(env),
//         )?;
//         map = map.map_put("can_encrypt".encode(env), key.can_encrypt().encode(env))?;
//         map = map.map_put("can_sign".encode(env), key.can_sign().encode(env))?;
//         map = map.map_put("can_certify".encode(env), key.can_certify().encode(env))?;
//         map = map.map_put(
//             "can_authenticate".encode(env),
//             key.can_authenticate().encode(env),
//         )?;
//         map = map.map_put("has_secret".encode(env), key.has_secret().encode(env))?;
//         map = map.map_put("is_revoked".encode(env), key.is_revoked().encode(env))?;
//         map = map.map_put("is_expired".encode(env), key.is_expired().encode(env))?;
//         map = map.map_put("is_disabled".encode(env), key.is_disabled().encode(env))?;
//         map = map.map_put("is_invalid".encode(env), key.is_invalid().encode(env))?;
//         map = map.map_put("is_qualified".encode(env), key.is_qualified().encode(env))?;

//         let mut users = vec![];

//         for (_i, user) in key.user_ids().enumerate() {
//             let mut user_map = Term::map_new(env);
//             user_map = user_map.map_put(
//                 "userid".encode(env),
//                 user.id().unwrap_or("[none]").encode(env),
//             )?;
//             user_map =
//                 user_map.map_put("valid".encode(env), user.validity().to_string().encode(env))?;
//             users.push(user_map);
//         }
//         map = map.map_put("users".encode(env), users.encode(env))?;

//         key_list.push(map);
//     }

//     Ok((atoms::ok(), key_list).encode(env))
// }
