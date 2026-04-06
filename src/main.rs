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

    match instance.create_session(
        1,
        21,
        SessionConfig::new(true, true, false, false, None).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    match instance.create_session(
        1,
        23,
        SessionConfig::new(true, true, true, true, None).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    println!("{:#?}", instance.players);
    println!("{:#?}", instance.session_manager);
}
