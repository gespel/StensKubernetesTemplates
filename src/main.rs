use sktlib::core::deployment::Deployment;
use sktlib::core::service::Service;
use sktlib::stringifyer::build_string;

fn main() {
    let s = build_string(
        Service::new(
            "test".to_string(),
            "test-backend".to_string()
        )
    );
    let d = build_string(
        Deployment::new(
            "test".to_string(),
            "test-image".to_string()
        )
    );
    println!("{}", s);
    println!("{}", d);
}
