use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("cwd-max-depth")
                .long("cwd-max-depth")
                .help("Maximum number of directories to show in path")
                .takes_value(true)
                .value_name("int")
                .default_value("5")
        )
        .arg(
            Arg::with_name("cwd-max-dir-size")
                .long("cwd-max-dir-size")
                .help("Maximum number of letters displayed for each directory in the path.\
                       Setting this to 0 means unlimited.")
                .takes_value(true)
                .value_name("int")
                .default_value("15")
        )
        .arg(
            Arg::with_name("error")
                .help("Exit code of previously executed command")
                .default_value("0")
        )
        .arg(
            Arg::with_name("modules")
                .long("modules")
                .help("The list of modules to load, separated by ','")
                .takes_value(true)
                .value_name("string")
                .possible_values(crate::module::ALL)
                .value_delimiter(",")
                .default_value("ssh,cwd,perms,git,gitstage,nix-shell,root")
        )
        .arg(
            Arg::with_name("newline")
                .long("newline")
                .help("Adds a newline after the prompt")
        )
        .arg(
            Arg::with_name("shell")
                .long("shell")
                .help("Set this to your shell type")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["bare", "bash", "zsh"])
                .default_value("bash")
        )
        .arg(
            Arg::with_name("theme")
                .long("theme")
                .help("Set this to the theme you want to use")
                .takes_value(true)
                .value_name("file")
        )
}
