use data_encoding::HEXUPPER;
use ring::{
    digest,
    error::Unspecified,
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::num::NonZeroU32;

fn main() -> Result<(), Unspecified> {
    let n_iter = NonZeroU32::new(10).unwrap();
    let rng = SystemRandom::new();

    let mut salt = [0u8; 12];
    rng.fill(&mut salt)?;

    let password = "Guess Me If You Can!";

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}
