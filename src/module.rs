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
        match s {
            "aws"      => Ok(Module::Aws),
            "cwd"      => Ok(Module::Cwd),
            "docker"   => Ok(Module::Docker),
            "dotenv"   => Ok(Module::Dotenv),
            "exit"     => Ok(Module::Exit),
            "git"      => Ok(Module::Git),
            "gitstage" => Ok(Module::GitStage),
            "hg"       => Ok(Module::Hg),
            "host"     => Ok(Module::Host),
            "jobs"     => Ok(Module::Jobs),
            "perlbrew" => Ok(Module::Perlbrew),
            "perms"    => Ok(Module::Perms),
            "root"     => Ok(Module::Root),
            "ssh"      => Ok(Module::Ssh),
            "time"     => Ok(Module::Time),
            "user"     => Ok(Module::User),
            "venv"     => Ok(Module::Venv),
            _          => Err(())
        }
    }
}
