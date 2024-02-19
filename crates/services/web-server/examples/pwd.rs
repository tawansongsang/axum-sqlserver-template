// use lib_auth::pwd::{self, scheme::Scheme, ContentToHash};
// use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("hello from pwd example");
    // let password_salt = Uuid::parse_str("FFCB9D04-A7B6-4839-BB6A-389179D01D60").unwrap();
    // let content_to_hash = ContentToHash {
    //     content: "lgd".to_string(),
    //     salt: password_salt,
    // };

    // println!("{:?} {:?}", content_to_hash.content, content_to_hash.salt);

    // let password_hash = pwd::hash_pwd(content_to_hash).await.unwrap();

    // println!("password_hash: {}", password_hash);

    // let content_to_hash = ContentToHash {
    //     content: "lgd".to_string(),
    //     salt: password_salt,
    // };

    // let password_ref = "#02#$argon2id$v=19$m=19456,t=2,p=1$/8udBKe2SDm7ajiRedAdYA$AAd/h6w7RbRz5OIv5kzqbGlliWQ1xOFfQ0LUG/Hbz90";
    // // let scheme_status = Scheme02;

    // let valid = pwd::validate_pwd(content_to_hash, password_ref.to_string()).await;
    // println!("valid: {:?}", valid);

    // // println!("{:?}", scheme_status);

    // println!("---------------demo2@demo.com--------------");

    // let password_salt = Uuid::parse_str("DFDB91F9-EB1D-4837-BE7C-DCEDE944FDA9").unwrap();
    // let content_to_hash = ContentToHash {
    //     content: "demo2".to_string(),
    //     salt: password_salt,
    // };

    // println!("{:?} {:?}", content_to_hash.content, content_to_hash.salt);

    // let password_hash = pwd::hash_pwd(content_to_hash).await.unwrap();

    // println!("password_hash: {}", password_hash);
}
