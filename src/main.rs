use remyan::*;

fn main() {
    let mut instance = App::new();
    instance
        .register_new_player(25, String::from("Andres"))
        .unwrap();

    instance
        .register_new_player(23, String::from("Sergio"))
        .unwrap();

    instance
        .register_new_player(22, String::from("Martin"))
        .unwrap();

    instance.handle_login(22).unwrap_or_else(|err| println!("{}", err));

    instance.create_session(
        1,
        22,
        SessionConfig::new(true, true, false, false, None).unwrap(),
    ).unwrap_or_else(|err| println!("{}", err));
}
