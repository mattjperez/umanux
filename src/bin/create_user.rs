use std::path::PathBuf;

extern crate clap;
use clap::{App, Arg};

extern crate adduser;
use adduser::api::UserDBWrite;

fn main() {
    env_logger::init();
    let matches = App::new("Create a new linux user")
        .version("0.1.0")
        .author("Franz Dietrich <dietrich@teilgedanken.de>")
        .about("Create a linux user do not use this in production (yet)")
        .arg(
            Arg::new("username")
                .short('n')
                .long("username")
                .value_name("USERNAME")
                .about("the new users name")
                .takes_value(true)
                .required(true),
        ) /*
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )*/
        .get_matches();

    let mf = adduser::Files {
        passwd: Some(PathBuf::from("./passwd")),
        shadow: Some(PathBuf::from("./shadow")),
        group: Some(PathBuf::from("./group")),
    };

    let mut db = adduser::UserDBLocal::load_files(mf).unwrap();

    let _user_res: Result<&adduser::User, adduser::UserLibError> = db.new_user(
        adduser::api::CreateUserArgs::builder()
            .username(matches.value_of("username").unwrap())
            // .delete_home(adduser::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );

    let user = adduser::User::default()
        .username("fest".into())
        .shell_path("/bin/mash".into())
        .clone();

    println!("{}", user);

    //db.new_user().expect("failed to create the user");
}
