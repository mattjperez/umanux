extern crate adduser;
use adduser::api::UserDBRead;

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )])
    .unwrap();

    let db = adduser::UserDBLocal::load_files(adduser::Files::default()).unwrap();

    for u in db.get_all_users() {
        println!("{}", u);
    }
}
