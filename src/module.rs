use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum Module {
    Cwd,
    Git,
    GitStage,
    Host,
    Jobs,
    Perms,
    Root,
    Ssh,
    Time,
    User
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cwd"      => Ok(Module::Cwd),
            "git"      => Ok(Module::Git),
            "gitstage" => Ok(Module::GitStage),
            "host"     => Ok(Module::Host),
            "jobs"     => Ok(Module::Jobs),
            "perms"    => Ok(Module::Perms),
            "root"     => Ok(Module::Root),
            "ssh"      => Ok(Module::Ssh),
            "time"     => Ok(Module::Time),
            "user"     => Ok(Module::User),
            _          => Err(())
        }
    }
}
