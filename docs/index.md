---
title: Oro Jackson
---

> customizable, single executable, plugin based, very fast, static site generator (written in rust btw)

## Quickstart

(Currently) You just need to install [nix](https://nixos.org/) and setup [nix-flakes](https://nixos.wiki/wiki/Flakes) in order to easily run oro-jackson.<br/>
Make sure you have installed the aforementioned things before continuing. You can follow [this guide](https://dev.to/arnu515/getting-started-with-nix-and-nix-flakes-mml).

Then, in your terminal of choice, enter the following command

```bash
nix run github:lovedeepsingh-07/oro-jackson
```

Depending upon your system, the above process can take a considerable amount of time (~5mins) and resources because it is isolating the nix-based dependencies for oro-jackson.<br/>
After this process completes running, you will see something like this in your command-line.

```
Usage: oro-jackson <COMMAND>

Commands:
  create

  build
          Build the content
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help
```

These are the various sub-commands that oro-jackson offers for you to run. Running the `--help` flag with these sub-commands lets you see the other arguments that you have to provide in order to run these sub-commands

You can run either sub-command by running the following

```
nix run github:lovedeepsingh-07/oro-jackson -- <sub-command>
```

- **create**: This sub-command allows you to create a simple oro-jackson project in the current working directory. Running `--help` flag with this sub-command will show the following

  ```
  Usage: oro-jackson create

  Options:
    -h, --help
            Print help
  ```

  This is because you don't need to add any additional flags in order for this sub-command to run, it will run without any flags and will create `config.toml` and `theme.css` files and `content` folder in your current working directory

- **build**: This sub-command allows you to actually build your oro-jackson project into a static-site. Running `--help` flag with this sub-command will show the following

  ```
  Usage: oro-jackson build [OPTIONS] --config <CONFIG> --theme <THEME> --content <CONTENT> --output <OUTPUT>

  Options:
        --config <CONFIG>
            path location of your config.toml file
        --theme <THEME>
            path location of your theme.css file
        --content <CONTENT>
            path location of your content folder
        --output <OUTPUT>
            path location of output folder
        --serve
            serve the content and watch for changes
    -h, --help
            Print help
  ```

  As you can see that this sub-command requires alot of flags in order to run.
