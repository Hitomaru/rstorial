use rstorial::domains::work::Work;
#[macro_use]
extern crate log;
fn main() {
    env_logger::init();
    let work = Work::new(
        "akeyume",
        "Hitomaru Horino",
        "明ける世界の夢見る機械",
        "test",
        "./"
    );
    work.init().unwrap();

    let path = Path::new("./akeyume/description.yml");
    let work = Work::load_description(path).unwrap();
    println!("{}", work);

    work.add_episode("第1章").unwrap();
}