use gpgme::{Context, Protocol};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
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
