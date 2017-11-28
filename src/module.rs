use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum Module {
    Aws,
    Cwd,
    Docker,
    Dotenv,
    Exit,
    Git,
    GitStage,
    Hg,
    Host,
    Jobs,
    None,
    Perlbrew,
    Perms,
    Root,
    Ssh,
    Time,
    User,
    Venv
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "aws"      => Module::Aws,
            "cwd"      => Module::Cwd,
            "docker"   => Module::Docker,
            "dotenv"   => Module::Dotenv,
            "exit"     => Module::Exit,
            "git"      => Module::Git,
            "gitstage" => Module::GitStage,
            "hg"       => Module::Hg,
            "host"     => Module::Host,
            "jobs"     => Module::Jobs,
            "perlbrew" => Module::Perlbrew,
            "perms"    => Module::Perms,
            "root"     => Module::Root,
            "ssh"      => Module::Ssh,
            "time"     => Module::Time,
            "user"     => Module::User,
            "venv"     => Module::Venv,
            _          => Module::None
        })
    }
}
