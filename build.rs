use actix_web_static_files::resource_dir;

/**
 * Bundles resources. 
 */
fn main() {
    resource_dir("./src/bundle").build().unwrap();
}