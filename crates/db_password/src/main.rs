use data_encoding::HEXUPPER;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// --- Password ---
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
	encoded_actual_password: &str,
) -> Result<(), Error> {
	let pbkdf2_iterations = NonZeroU32::new(100_000).unwrap();
	let salt = get_salt(username);
	let actual_password = HEXUPPER.decode(encoded_actual_password.as_bytes()).unwrap();
	pbkdf2::verify(
		PBKDF2_ALG,
		pbkdf2_iterations,
		&salt,
		attempted_password.as_bytes(),
		&actual_password,
	)
	.map_err(|_| Error::WrongUsernameOrPassword)
}

// --- Token ---
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	exp: usize,
	sub: String,
}

fn generate_token(username: &str) -> String {
	let secret = dotenv::var("TOKEN_SECRET").unwrap();
	let exp = SystemTime::now()
		.checked_add(Duration::from_secs(604800)) // 7 days
		.unwrap()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let my_claims = Claims {
		exp: exp as usize,
		sub: username.to_string(), // or email address
	};

	let token = encode(
		&Header::default(),
		&my_claims,
		&EncodingKey::from_secret(secret.as_bytes()),
	)
	.unwrap();

	token
}

// TODO: return Result<(), ErrorKind>?
fn verify_token(username: &str, encoded_token: &str) -> bool {
	let secret = dotenv::var("TOKEN_SECRET").unwrap();
	let validation = Validation {
		sub: Some(username.to_string()),
		..Default::default()
	};
	let token = decode::<Claims>(
		encoded_token,
		&DecodingKey::from_secret(secret.as_bytes()),
		&validation,
	);

	match token {
		Ok(_data) => true,
		Err(err) => match *err.kind() {
			ErrorKind::InvalidToken => false,
			ErrorKind::InvalidIssuer => false,
			ErrorKind::InvalidSignature => false,
			ErrorKind::InvalidSubject => panic!("Invalide subject"),
			_ => false,
		},
	}
}

// --- Main ---
fn main() {
	println!("--- Passwords ---");
	let password = generate_password("username", "password");
	let check_ok = verify_password("username", "password", &password);
	let check_fail = verify_password("username", "pasword", &password);
	println!("Password: {}", &password);
	println!("{:#?}", check_ok);
	println!("{:#?}", check_fail);
	println!("--- Token ---");
	let token = generate_token("username");
	println!("Token: {}", &token);
	println!("{:#?}", verify_token("username", &token));
	println!("{:#?}", verify_token("user", &token));
}
