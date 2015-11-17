use data::*;
use secstr::SecStr;
use rusterpassword::*;
use rustc_serialize::base64::{ToBase64, STANDARD};
use sodiumoxide::crypto::sign::ed25519;
use byteorder::{BigEndian, WriteBytesExt};
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))] use unix_socket::UnixStream;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))] use std::env;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))] use std::io::{Read, Write};
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))] use std::net::Shutdown;

pub enum Output {
    PrivateText(SecStr),
    OpenText(String),
    PrivateBinary(SecStr),
    Ed25519Keypair(Ed25519Usage, ed25519::PublicKey, ed25519::SecretKey),
}

fn pick_tpl(tpl: &PasswordTemplate) -> &'static [&'static str] {
    match *tpl {
       PasswordTemplate::Maximum  => TEMPLATES_MAXIMUM,
       PasswordTemplate::Long     => TEMPLATES_LONG,
       PasswordTemplate::Medium   => TEMPLATES_MEDIUM,
       PasswordTemplate::Short    => TEMPLATES_SHORT,
       PasswordTemplate::Basic    => TEMPLATES_BASIC,
       PasswordTemplate::Pin      => TEMPLATES_PIN,
    }
}

pub fn process_output(entry_name: &str, master_key: &SecStr, field: &Field) -> Result<Output> {
    match *field {
        Field::Derived { counter, ref site_name, ref usage } => {
            let site_seed = try!(gen_site_seed(master_key, &site_name.clone().unwrap_or(entry_name.to_string()), counter)
                                 .map_err(|_| Error::SeedGenerationError));
            match *usage {
                DerivedUsage::Password(ref tpl) =>
                    Ok(Output::PrivateText(gen_site_password(&site_seed, pick_tpl(tpl)))),
                DerivedUsage::Ed25519Key(ref keyusage) => {
                    let edseed = try!(ed25519::Seed::from_slice(site_seed.unsecure()).ok_or(Error::WrongDerivedKeyLength));
                    let (pubkey, seckey) = ed25519::keypair_from_seed(&edseed);
                    Ok(Output::Ed25519Keypair(*keyusage, pubkey, seckey))
                },
                DerivedUsage::RawKey =>
                    Ok(Output::PrivateBinary(site_seed)),
            }
        },
        Field::Stored { ref data, ref usage } =>
            match *usage {
                StoredUsage::Password =>
                    Ok(Output::PrivateText(SecStr::new(data.unsecure().to_vec()))),
                StoredUsage::Text =>
                    Ok(Output::OpenText(try!(String::from_utf8(data.unsecure().to_vec()))))
            }
    }
}

pub fn ssh_public_key_output(keypair: &Output, comment: &str) -> Result<String> {
    if let &Output::Ed25519Keypair(Ed25519Usage::SSH, ed25519::PublicKey(pubkey_bytes), _) = keypair {
        let mut raw = vec![];
        try!(raw.write_u32::<BigEndian>(11));
        raw.extend(b"ssh-ed25519");
        try!(raw.write_u32::<BigEndian>(ed25519::PUBLICKEYBYTES as u32));
        raw.extend(&pubkey_bytes);
        Ok("ssh-ed25519 ".to_string() + &raw.to_base64(STANDARD) + " " + comment)
    } else { Err(Error::InappropriateFormat) }
}

pub fn ssh_private_key_agent_message(keypair: &Output, comment: &str) -> Result<SecStr> {
    if let &Output::Ed25519Keypair(Ed25519Usage::SSH, ed25519::PublicKey(pubkey_bytes), ed25519::SecretKey(seckey_bytes)) = keypair {
        let mut msg = vec![17u8];
        try!(msg.write_u32::<BigEndian>(11));
        msg.extend(b"ssh-ed25519");
        try!(msg.write_u32::<BigEndian>(ed25519::PUBLICKEYBYTES as u32));
        msg.extend(&pubkey_bytes);
        try!(msg.write_u32::<BigEndian>(ed25519::SECRETKEYBYTES as u32));
        msg.extend(seckey_bytes.iter()); // LOL, there's no iterator for &[u8, 64] because 64 is a lot
        try!(msg.write_u32::<BigEndian>(comment.as_bytes().len() as u32));
        msg.extend(comment.as_bytes());
        Ok(SecStr::new(msg))
    } else { Err(Error::InappropriateFormat) }
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))]
pub fn ssh_agent_send_message(msg: SecStr) -> Result<()> {
    if let Some(sock_path) = env::var_os("SSH_AUTH_SOCK") {
        let mut stream = try!(UnixStream::connect(sock_path));
        try!(stream.write_u32::<BigEndian>(msg.unsecure().len() as u32));
        try!(stream.write(msg.unsecure()));
        let mut response = vec![0; 5];
        try!(stream.read(&mut response));
        try!(stream.shutdown(Shutdown::Both));
        Ok(())
    } else { Err(Error::SSHAgentSocketNotFound) }
}

#[cfg(any(not(unix), target_os = "android", target_os = "ios"))]
pub fn ssh_agent_send_message(_: SecStr) -> Result<()> {
    Err(Error::NotAvailableOnPlatform)
}
