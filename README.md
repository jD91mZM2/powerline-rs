# powerline-rs

`powerline-rs` is a rewrite of [powerline-shell](https://github.com/b-ryan/powerline-shell), inspired by [powerline-go](https://github.com/justjanne/powerline-go).

## Why?

Speed! I haven't done any extensive benchmarks, but it appears like
even the debug version of `powerline-rs` is about 20 milliseconds faster than `powerline-go`.
The Rust language is perfect for fast applications, since copying large structures isn't implicit.

## How to install

```
cargo install powerline-rs
```

It's also available on Nix and the [AUR (not officially maintained)](https://aur.archlinux.org/packages/powerline-rs/).

In order to use optional features, add something like `--features "feature1,feature2,etc"` to the end of the command.  
To disable default features, add `--no-default-features` (and manually readd features you want to keep).  

Optional features:

- flame  (Not recommended for production; adds benchmarks)

Default features:

- chrono (Time support for --shell bare)
- git2 (Git support)
- users (Username support)

Then add the following code to your shell:  
[Bash](#bash)  
[Fish](#fish)  
[Zsh](#zsh)

### What's new?

Well, the default modules have changed to not include the username and hostname.
I feel like most people already know that.
But you can always enable it, of course!

### What's optimized?

 - Generally just using Rust.
 - Using `libgit2` over calling and parsing `git` output (Thanks [tbodt](https://github.com/tbodt) for suggesting it!)
 - `libgit2` can be disabled at compile time if you don't plan on using git functionality.
 - Themes are using a simple small `key=value` scripts. No JSON overhead or similar.
 - The output of `powerline-rs` is slightly smaller than the 2 alternatives I mentioned. To be honest, I have no idea why.

### What's removed?

Most of the service-specific modules are deleted. I am very lazy.  
Pull requests are welcome, though.

Also, the `jobs` module won't work with `--shell bare`.

# Add it to your shell

## Bash

```Bash
prompt() {
    PS1="$(powerline-rs --shell bash $?)"
}
PROMPT_COMMAND=prompt
```

## Zsh

```Zsh
prompt() {
    PS1="$(powerline-rs --shell zsh $?)"
}
precmd_functions+=(prompt)
```

## Fish

```Fish
function fish_prompt
    powerline-rs --shell bare $status
end
```

## Ion

*We can't display the success status because ion now forbids the use `$?` from
functions as a bi-product of the new [namespacing
system](https://gitlab.redox-os.org/redox-os/ion/merge_requests/807). This will
of course eventually be resolved.*

```Ion
fn PROMPT
    powerline-rs --shell bare
end
```
