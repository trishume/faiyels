#[macro_use]
extern crate qmlrs;
extern crate find_folder;

fn main() {
    let mut engine = qmlrs::Engine::new();

    // engine.set_property("factorial", math::Factorial);

    let assets = find_folder::Search::KidsThenParents(3, 5)
                .for_folder("assets").unwrap();
    let main_file = assets.join("main.qml");
    engine.load_local_file(main_file.to_str().unwrap());

    engine.exec();
}
