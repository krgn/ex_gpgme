use atoms;
use gpgme::{Context, PinentryMode, Protocol, SignatureNotationFlags};
use key::GpgmeKey;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::io::prelude::*;
use std::sync::Mutex;

pub(crate) struct GpgmeContext(pub Mutex<Context>);

unsafe impl Send for GpgmeContext {}
unsafe impl Sync for GpgmeContext {}

impl std::convert::From<Context> for GpgmeContext {
    fn from(ctx: Context) -> GpgmeContext {
        GpgmeContext(Mutex::new(ctx))
    }
}

mod keys {
    rustler_atoms! {
        atom home;
        atom version;
        atom protocol;
        atom required_version;
        atom path;
    }
}

fn create_wrapped(proto: Protocol, path: &str) -> Result<GpgmeContext, rustler::Error> {
    match Context::from_protocol(proto) {
        Ok(mut ctx) => {
            ctx.set_armor(true);
            ctx.set_text_mode(true);

            ctx.set_pinentry_mode(PinentryMode::Loopback)
                .expect("could not set pinentry mode");
            ctx.set_engine_home_dir(path)
                .expect("Fatal: could not set home dir");
            Ok(ctx.into())
        }
        Err(_err) => Err(rustler::Error::Atom("context_initialization")),
    }
}

/// Create a new Ggpme Context and return it to erlang as a reference.
pub fn create<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let proto: Protocol = crate::protocol::from_term(args[0])?;
    let path: String = args[1].decode()?;
    let context = create_wrapped(proto, &path)?;
    let resource = ResourceArc::new(context);
    Ok((crate::atoms::ok(), resource).encode(env))
}

pub fn info<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let context = res.0.lock().unwrap();
    let info = context.engine_info();

    let mut map = Term::map_new(env);

    map = map.map_put(
        keys::home().encode(env),
        info.home_dir().unwrap().encode(env),
    )?;

    map = map.map_put(keys::path().encode(env), info.path().unwrap().encode(env))?;

    map = map.map_put(
        keys::version().encode(env),
        info.version().unwrap().encode(env),
    )?;

    map = map.map_put(
        keys::protocol().encode(env),
        crate::protocol::as_term(info.protocol()).encode(env),
    )?;

    map = map.map_put(
        keys::required_version().encode(env),
        info.required_version().unwrap().encode(env),
    )?;

    Ok((crate::atoms::ok(), map).encode(env))
}

mod import_result {
    use atoms;
    use gpgme::ImportResult;
    use rustler::{Encoder, Env, NifResult, Term};

    mod imports {
        use atoms;
        use gpgme::Import;
        use rustler::{Encoder, Env, NifResult, Term};

        rustler_atoms! {
            atom fingerprint;
            atom result;
            atom status;
        }

        pub fn as_map<'a>(env: Env<'a>, import: Import) -> NifResult<Term<'a>> {
            let mut map = Term::map_new(env);
            map = map.map_put(
                fingerprint().encode(env),
                import
                    .fingerprint()
                    .map(|f| f.encode(env))
                    .unwrap_or(atoms::none().encode(env)),
            )?;

            let import_result = match import.result() {
                Ok(_) => atoms::ok(),
                _ => atoms::error(),
            };

            let import_status = format!("{:?}", import.status());

            map = map.map_put(result().encode(env), import_result.encode(env))?;
            map = map.map_put(status().encode(env), import_status.encode(env))?;

            Ok(map)
        }
    }

    rustler_atoms! {
        atom without_user_id;
        atom new_user_ids;
        atom new_subkeys;
        atom new_signatures;
        atom new_revocations;
        atom considered;
        atom imported;
        atom unchanged;
        atom secret_considered;
        atom secret_imported;
        atom secret_unchanged;
        atom not_imported;
        atom imports;
    }

    pub fn from<'a>(env: Env<'a>, result: ImportResult) -> NifResult<Term<'a>> {
        let mut map = Term::map_new(env);

        map = map.map_put(
            without_user_id().encode(env),
            result.without_user_id().encode(env),
        )?;

        map = map.map_put(
            new_user_ids().encode(env),
            result.new_user_ids().encode(env),
        )?;

        map = map.map_put(
            new_revocations().encode(env),
            result.new_revocations().encode(env),
        )?;

        map = map.map_put(
            secret_considered().encode(env),
            result.secret_considered().encode(env),
        )?;

        map = map.map_put(
            secret_imported().encode(env),
            result.secret_imported().encode(env),
        )?;

        map = map.map_put(
            secret_unchanged().encode(env),
            result.secret_unchanged().encode(env),
        )?;

        map = map.map_put(
            not_imported().encode(env),
            result.not_imported().encode(env),
        )?;

        map = map.map_put(unchanged().encode(env), result.unchanged().encode(env))?;
        map = map.map_put(new_subkeys().encode(env), result.new_subkeys().encode(env))?;
        map = map.map_put(considered().encode(env), result.considered().encode(env))?;
        map = map.map_put(imported().encode(env), result.imported().encode(env))?;

        let mut import_results = Vec::<Term<'a>>::new();

        for import_result in result.imports() {
            let mut result_map = imports::as_map(env, import_result)?;
            import_results.push(result_map)
        }

        map = map.map_put(imports().encode(env), import_results.encode(env))?;

        Ok((atoms::ok(), map).encode(env))
    }
}

pub fn import<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let data: String = args[1].decode()?;
    let mut context = res.0.lock().unwrap();
    match context.import(data) {
        Ok(result) => import_result::from(env, result),
        Err(_err) => Err(rustler::Error::Atom("error")),
    }
}

pub fn encrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let fingerprint: String = args[1].decode()?;
    let data: String = args[2].decode()?;
    let mut context = res.0.lock().unwrap();
    let key = context.get_key(&fingerprint).unwrap();
    let mut encrypted = Vec::new();
    context
        .encrypt_with_flags(
            Some(&key),
            data,
            &mut encrypted,
            gpgme::EncryptFlags::ALWAYS_TRUST,
        )
        .unwrap();
    let ascii = String::from_utf8(encrypted).unwrap().encode(env);
    Ok((atoms::ok(), ascii).encode(env))
}

pub fn encrypt_symmetric<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let passphrase_str: String = args[1].decode()?;
    let passphrase: &[u8] = passphrase_str.as_bytes();
    let data: String = args[2].decode()?;
    let mut context = res.0.lock().unwrap();
    let mut encrypted = Vec::new();
    context.with_passphrase_provider(
        |_req: gpgme::PassphraseRequest, out: &mut Write| match out.write_all(passphrase) {
            Ok(()) => Ok(()),
            Err(_) => Err(gpgme::Error::from_code(32)),
        },
        |ctx| {
            ctx.encrypt_symmetric_with_flags(
                data,
                &mut encrypted,
                gpgme::EncryptFlags::ALWAYS_TRUST,
            )
            .unwrap();
        },
    );
    let ascii = String::from_utf8(encrypted).unwrap().encode(env);
    Ok((atoms::ok(), ascii).encode(env))
}

pub fn decrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let passphrase_str: String = args[1].decode()?;
    let ciphertext_str: String = args[2].decode()?;
    let passphrase: &[u8] = passphrase_str.as_bytes();
    let ciphertext: &[u8] = ciphertext_str.as_bytes();
    let mut context = res.0.lock().unwrap();
    let mut plaintext = Vec::new();
    context.with_passphrase_provider(
        |_req: gpgme::PassphraseRequest, out: &mut Write| match out.write_all(passphrase) {
            Ok(()) => Ok(()),
            Err(_) => Err(gpgme::Error::from_code(32)),
        },
        |ctx| {
            ctx.decrypt(ciphertext, &mut plaintext).unwrap();
        },
    );
    let binary = String::from_utf8(plaintext).unwrap().encode(env);
    Ok((atoms::ok(), binary).encode(env))
}

pub fn find_key<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut context = res.0.lock().unwrap();
    let query: String = args[1].decode()?;
    match context.get_key(query) {
        Ok(key) => {
            let wrapped: ResourceArc<GpgmeKey> = ResourceArc::new(key.into());
            Ok((atoms::ok(), wrapped).encode(env))
        }
        Err(_err) => Err(rustler::Error::Atom("not_found")),
    }
}

mod signature_notation {
    rustler_atoms! {
        atom name;
        atom value;
        atom critical;
        atom human_readable;
    }
}

pub fn signature_notations<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let context = res.0.lock().unwrap();
    let mut notations: Vec<Term<'a>> = Vec::new();
    for notation in context.signature_notations() {
        let mut map = Term::map_new(env);
        map = map.map_put(
            signature_notation::name().encode(env),
            notation
                .name()
                .map(|s| s.encode(env))
                .unwrap_or(atoms::none().encode(env)),
        )?;
        map = map.map_put(
            signature_notation::value().encode(env),
            notation
                .value()
                .map(|s| s.encode(env))
                .unwrap_or(atoms::none().encode(env)),
        )?;
        map = map.map_put(
            signature_notation::critical().encode(env),
            notation.is_critical().encode(env),
        )?;
        map = map.map_put(
            signature_notation::human_readable().encode(env),
            notation.is_human_readable().encode(env),
        )?;
        notations.push(map)
    }
    Ok((atoms::ok(), notations).encode(env))
}

pub fn add_signature_notation<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut context = res.0.lock().unwrap();
    let name: String = args[1].decode()?;
    let value: String = args[2].decode()?;
    let flags: Vec<Term<'a>> = args[3].decode()?;
    let parsed_flags: SignatureNotationFlags =
        flags.iter().fold(SignatureNotationFlags::empty(), |a, t| {
            let result: NifResult<rustler::types::atom::Atom> = t.decode();
            match result {
                Ok(t) if t == signature_notation::human_readable() => {
                    a | SignatureNotationFlags::HUMAN_READABLE
                }
                Ok(t) if t == signature_notation::critical() => {
                    a | SignatureNotationFlags::CRITICAL
                }
                _ => a,
            }
        });
    // let flags = SignatureNotationFlags::HUMAN_READABLE | SignatureNotationFlags::CRITICAL;
    match context.add_signature_notation(name, value, parsed_flags) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(_err) => Err(rustler::Error::Atom("error")),
    }
}

pub fn clear_signature_notations<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut context = res.0.lock().unwrap();
    context.clear_signature_notations();
    Ok(atoms::ok().encode(env))
}

pub fn sender<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let context = res.0.lock().unwrap();
    match context.sender() {
        Ok(s) if s.len() == 0 => Ok(atoms::none().encode(env)),
        Ok(s) => Ok(s.encode(env)),
        Err(_err) => Err(rustler::Error::Atom("error")),
    }
}

pub fn set_sender<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let sender: String = args[1].decode()?;
    let mut context = res.0.lock().unwrap();
    match context.set_sender(sender) {
        Ok(s) => Ok(atoms::ok().encode(env)),
        Err(_err) => Err(rustler::Error::Atom("error")),
    }
}

pub fn clear_sender<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<GpgmeContext> = args[0].decode()?;
    let mut context = res.0.lock().unwrap();
    match context.clear_sender() {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(_err) => Err(rustler::Error::Atom("error")),
    }
}
