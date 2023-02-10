use lazy_static::lazy_static;
use sys_locale::get_locale;

lazy_static! {
    pub static ref LOCALES: String = get_locale().unwrap_or_else(|| String::from("en-US"));
}
