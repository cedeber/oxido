use data_encoding::HEXUPPER;
use dotenv;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

// @see https://briansmith.org/rustdoc/ring/pbkdf2/index.html

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

// Normally these parameters would be loaded from a configuration file.
// Check .env file for an example
fn _db_salt_component() -> String {
    let rng = rand::SystemRandom::new();
    let mut salt: Credential = [0u8; CREDENTIAL_LEN];

    rng.fill(&mut salt).unwrap();
    HEXUPPER.encode(&salt)
}

#[derive(Debug)]
enum Error {
    WrongUsernameOrPassword,
}

fn get_salt(username: &str) -> Vec<u8> {
    let salt_component = HEXUPPER
        .decode(&dotenv::var("DB_PASSWORD_SALT").unwrap().as_bytes())
        .unwrap();
    let mut salt = Vec::with_capacity(salt_component.len() + username.as_bytes().len());
    salt.extend(&salt_component);
    salt.extend(username.as_bytes());
    salt
}

fn generate_password(username: &str, password: &str) -> String {
    let pbkdf2_iterations = NonZeroU32::new(100_000).unwrap();
    let salt = get_salt(username);
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        pbkdf2_iterations,
        &salt,
        password.as_bytes(),
        &mut to_store,
    );
    HEXUPPER.encode(&to_store)
}

// actual_password should comes from the DB
fn verify_password(
    username: &str,
    attempted_password: &str,
    actual_password_hash: &str,
) -> Result<(), Error> {
    let pbkdf2_iterations = NonZeroU32::new(100_000).unwrap();
    let salt = get_salt(username);
    let actual_password = HEXUPPER.decode(actual_password_hash.as_bytes()).unwrap();
    pbkdf2::verify(
        PBKDF2_ALG,
        pbkdf2_iterations,
        &salt,
        attempted_password.as_bytes(),
        &actual_password,
    )
    .map_err(|_| Error::WrongUsernameOrPassword)
}

fn main() {
    let password_hash = generate_password("username", "password");
    let check_ok = verify_password("username", "password", &password_hash);
    let check_fail = verify_password("username", "pasword", &password_hash);
    println!("Password hash: {}", &password_hash);
    println!("{:#?}", check_ok);
    println!("{:#?}", check_fail);
}
