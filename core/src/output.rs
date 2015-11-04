use data::*;
use secstr::SecStr;
use rusterpassword::*;

pub enum OutputFormat {
    Text,
    Binary
}

#[derive(Debug)]
pub enum OutputError {
    SeedGenerationError,
}

pub type OutputResult<T> = Result<T, OutputError>;

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

pub fn process_output(entry_name: &str, master_key: &SecStr, field: &Field) -> OutputResult<(SecStr, OutputFormat)> {
    match *field {
        Field::Derived { counter, ref site_name, ref usage } => {
            let site_seed = try!(gen_site_seed(master_key, &site_name.clone().unwrap_or(entry_name.to_string()), counter)
                                 .map_err(|_| OutputError::SeedGenerationError));
            match *usage {
                DerivedUsage::Password(ref tpl) => Ok((gen_site_password(&site_seed, pick_tpl(tpl)), OutputFormat::Text)),
                DerivedUsage::RawKey => Ok((site_seed, OutputFormat::Binary)),
            }
        },
        Field::Stored { ref data, ref usage } =>
            match *usage {
                StoredUsage::Password => Ok((SecStr::new(data.unsecure().to_vec()), OutputFormat::Text)),
            }
    }
}
