use gpgme::Protocol;
use rustler::*;

rustler_atoms! {
    atom openpgp;
    atom unsupported_protocol;
}

pub fn from_term<'a>(term: Term<'a>) -> NifResult<Protocol> {
    let input: types::atom::Atom = term.decode()?;
    match input {
        _ if input == openpgp() => Ok(Protocol::OpenPgp),
        _ => Err(Error::Atom("unsupported_protocol")),
    }
}

pub fn as_term(protocol: Protocol) -> types::atom::Atom {
    match protocol {
        Protocol::OpenPgp => openpgp(),
        _ => unsupported_protocol(),
    }
}
