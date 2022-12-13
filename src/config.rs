use lazy_static::lazy_static;

lazy_static! {
    pub static ref JWT_SECRET: String = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
}
