mod v1;

pub fn build() -> actix_web::Scope {
    v1::build()
}
