use atoms;
use context::GpgmeContext;
use gpgme::KeyListMode;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

pub(crate) struct GpgmeKey(pub gpgme::Key);

unsafe impl Send for GpgmeKey {}
unsafe impl Sync for GpgmeKey {}

mod signature {
    rustler_atoms! {
        atom signer_key;
        atom signer;
        atom algorithm;
        atom expired;
        atom creation_time;
        atom expiration_time;
        atom invalid;
        atom revoked;
        atom exportable;
        atom status;
    }
}

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

pub fn key_can_certify<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let can_certify = key.0.can_certify().encode(env);
    Ok((atoms::ok(), can_certify).encode(env))
}

pub fn key_can_authenticate<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let can_authenticate = key.0.can_authenticate().encode(env);
    Ok((atoms::ok(), can_authenticate).encode(env))
}

pub fn key_has_secret<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let has_secret = key.0.has_secret().encode(env);
    Ok((atoms::ok(), has_secret).encode(env))
}

pub fn key_is_revoked<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let is_revoked = key.0.is_revoked().encode(env);
    Ok((atoms::ok(), is_revoked).encode(env))
}

pub fn key_is_expired<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let is_expired = key.0.is_expired().encode(env);
    Ok((atoms::ok(), is_expired).encode(env))
}

pub fn key_is_disabled<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let is_disabled = key.0.is_disabled().encode(env);
    Ok((atoms::ok(), is_disabled).encode(env))
}

pub fn key_is_invalid<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let is_invalid = key.0.is_invalid().encode(env);
    Ok((atoms::ok(), is_invalid).encode(env))
}

pub fn key_is_qualified<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let key: ResourceArc<GpgmeKey> = args[0].decode()?;
    let is_qualified = key.0.is_qualified().encode(env);
    Ok((atoms::ok(), is_qualified).encode(env))
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

        let mut signatures: Vec<Term<'a>> = Vec::new();

        for signature in user_id.signatures() {
            let mut sig = Term::map_new(env);

            sig = sig.map_put(
                signature::signer_key().encode(env),
                signature
                    .signer_key_id()
                    .map(|s| s.encode(env))
                    .unwrap_or(atoms::none().encode(env)),
            )?;

            sig = sig.map_put(
                signature::signer().encode(env),
                signature
                    .signer_user_id()
                    .map(|s| s.encode(env))
                    .unwrap_or(atoms::none().encode(env)),
            )?;

            sig = sig.map_put(
                signature::algorithm().encode(env),
                signature
                    .algorithm()
                    .name()
                    .map(|s| s.encode(env))
                    .unwrap_or(atoms::unknown().encode(env)),
            )?;

            sig = sig.map_put(
                signature::creation_time().encode(env),
                signature
                    .creation_time()
                    .map(|t| {
                        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .map(|t| t.as_secs().encode(env))
                            .unwrap_or(atoms::unknown().encode(env))
                    })
                    .unwrap_or(atoms::unknown().encode(env)),
            )?;

            sig = sig.map_put(
                signature::expiration_time().encode(env),
                signature
                    .expiration_time()
                    .map(|t| {
                        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .map(|t| t.as_secs().encode(env))
                            .unwrap_or(atoms::unknown().encode(env))
                    })
                    .unwrap_or(atoms::unknown().encode(env)),
            )?;

            sig = sig.map_put(
                signature::invalid().encode(env),
                signature.is_invalid().encode(env),
            )?;

            sig = sig.map_put(
                signature::revoked().encode(env),
                signature.is_revocation().encode(env),
            )?;

            sig = sig.map_put(
                signature::exportable().encode(env),
                signature.is_exportable().encode(env),
            )?;

            sig = sig.map_put(
                signature::status().encode(env),
                format!("{:?}", signature.status()).encode(env),
            )?;

            signatures.push(sig);
        }

        map = map.map_put(user_id::signatures().encode(env), signatures.encode(env))?;

        list.push(map)
    }
    Ok((atoms::ok(), list).encode(env))
}
