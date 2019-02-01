use std::str::FromStr;

pub const ALL: &[&str] = &[
    "cwd",
    "git",
    "gitstage",
    "host",
    "jobs",
    "nix-shell",
    "perms",
    "ps",
    "root",
    "ssh",
    "time",
    "user",
    "virtualenv",
];

#[derive(PartialEq, Eq)]
pub enum Module {
    Cwd,
    Git,
    GitStage,
    Host,
    Jobs,
    NixShell,
    Perms,
    Ps,
    Root,
    Ssh,
    Time,
    User,
    VirtualEnv,
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cwd"        => Ok(Module::Cwd),
            "git"        => Ok(Module::Git),
            "gitstage"   => Ok(Module::GitStage),
            "host"       => Ok(Module::Host),
            "jobs"       => Ok(Module::Jobs),
            "nix-shell"  => Ok(Module::NixShell),
            "perms"      => Ok(Module::Perms),
            "ps"         => Ok(Module::Ps),
            "root"       => Ok(Module::Root),
            "ssh"        => Ok(Module::Ssh),
            "time"       => Ok(Module::Time),
            "user"       => Ok(Module::User),
            "virtualenv" => Ok(Module::VirtualEnv),
            _          => Err(())
        }
    }
}
