use dotenvy::dotenv;
use s3::creds::Credentials;

pub enum AllowedCredentials {
    Env,
    OsSecureEnclave,
    Tpm,
    File,
}

pub fn get_credentials(creds: &AllowedCredentials) -> Credentials {
    match creds {
        AllowedCredentials::Env => {
            dotenv().expect("failed to load .env");
            Credentials::from_env().expect("failed to fetch credentials")
        }
        AllowedCredentials::OsSecureEnclave => todo!(),
        AllowedCredentials::Tpm => todo!(),
        AllowedCredentials::File => todo!(),
    }
}
