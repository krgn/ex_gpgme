use gpgme::{Context, Protocol};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::Mutex;

pub(crate) struct GpgmeContext(pub Mutex<Context>);

unsafe impl Send for GpgmeContext {}
unsafe impl Sync for GpgmeContext {}

pub fn create<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let proto = Protocol::OpenPgp;
    let ctx = Context::from_protocol(proto).expect("context");
    let resource = ResourceArc::new(GpgmeContext(Mutex::new(ctx)));

    Ok((crate::atoms::ok(), resource).encode(env))
}
