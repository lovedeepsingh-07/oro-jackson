---
title: Oro Jackson
---

> customizable, single executable, plugin based, very fast, static site generator (written in rust btw)

# Quickstart

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

# Architecture

> NOTE: The following section explains the basic fundamental idea of how the project works, there are things that you might not understand if you are reading this for the first time, but that is intended. You are not supposed to understand this implementation of the project because it is for the CURRENT stage of MY project. This section is written only to give you the basic idea of how you might go about writing a similar project, not to provide an actual guide for writing the project. It is encouraged and IMPORTANT that you figure out what your implementation of the project might be because simply copying this would be unfruitful and annoying to deal with in the later stages of development.

```mermaid
graph LR
    MarkdownContent(Markdown Content)
    Transformers([Transformers])
    Emitters([Emitters])
    OutputHTML(Output HTML)

    MarkdownContent --> Transformers --> Emitters --> OutputHTML
```

Above diagram explains the basic architecture of how oro-jackson works.<br/>
The program runs 3 main loops for: preparing content, transforming prepared content, emitting transformed content. In between these main loops there are some other loops as well that do other basic but functionally required stuff.

```rust
pub fn parse(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {}
pub fn emit(ctx: &context::Context, parsed_files: &Vec<oj_file::OjFile>) -> eyre::Result<(), error::Error> {}
```

The above two functions are responsible for running the 3 main loops and communicating resulting data between them.

## Context

While the actual content navigates through the various loops and stages between reading a markdown file and emitting an HTML file, another data structure navigates along with it: "Context".<br/>
It basically provides additional helpfull information alongside the data itself. It looks something like following:

```rust
pub struct BuildArgs {
    pub content: vfs::VfsPath,
    pub output: vfs::VfsPath,
    pub serve: bool,
    pub cli_args: cli::Build,
}

pub struct Context {
    pub config: config::Config,
    pub theme: String,
    pub build_args: BuildArgs,
    pub is_rebuild: bool,
    pub build_path: vfs::VfsPath,
    pub transformer_plugins: Vec<plugins::Transformer>,
    pub emitter_plugins: Vec<plugins::Emitter>,
}
```

## Preparing Content

"Preparing" the content just means reading the input directory to convert all of its files into a useful data strcture of the application to easily process and transport.

```rust
pub fn prepare_content(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {}
pub fn prepare_folder_content(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {}
```

Above are two functions that work together in order to recursively read all the files in the provided input directory and convert them into an abstracted `OjFile` format, which looks something like this:

```rust
pub struct OjFile {
    pub frontmatter: frontmatter::Frontmatter,
    pub input_path: vfs::VfsPath,
    pub output_path: vfs::VfsPath,
    pub content: String,
}
```

This is nothing but a basic abstraction over the actual files themselves. All this does is make life much easier when dealing with files later on in the process.

## Plugins

Plugins are basically functions of different types that are stored and run.

## Transforming Content

After we have properly read and processed all the files from the input folder, we have to apply the needed transformation on the contents of those files. These transformations are applied using "TransformerPlugins" which are of following structure:

```rust
pub type Transformer = for<'a> fn(
    ctx: &'a context::Context,
    content_files: &'a mut Vec<oj_file::OjFile>,
) -> eyre::Result<&'a mut Vec<oj_file::OjFile>, error::Error>;
```

The transformer plugins are applied in the ["parse"](#architecture) function mentioned above one by one.
The way these plugins are applied is also very special and important to the overall functionality.

In the "parse" function, we run a loop that goes through all the available(or enabled) transformer plugin one-by-one, the current plugin that loop then runs a loop over all the files in the "content_files" vector and applied that specific transformation to each and every one of those files. This happens until the current transformation is applied to all the files in the vector.

The order in which plugins are applied is very important, and we only move onto the next plugin once the current plugin has been applied on all the files. The importance of this order can be understood by the following example:<br/>
Imagine that we have some markdown files and the transformer plugins that are enabled are

- **"Markdown"**: converts markdown into HTML
- **"WikiLinks"**: parses the markdown links with respect to the input folder root and writes the parsed links back into markdown
- **"Sanitize"**: sanitizes the HTML

Do you think the order in which these plugins act is not important? Ofcourse it is! <br/>
The only order in which these plugins can be applied is "WikiLinks", "Markdown", and then "Sanitize". Any other order will mess up the functioning and either throw a runtime error or cause unintended output to be emitted.

This is why we use a vector to store the transformer and emitter plugins in the ["Context"](#context) struct. When building that struct, we store the transformer and emitter plugins in the order they should be applied.

## Emitting Content

After we have successfully applied all the transformations to the parsed content, we have to write those transformed files onto the disk. We do that by using "EmitterPlugins" that look something like:

```rust
pub type Emitter = fn(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error>;
```

These plugins follow the same "one-by-one" method as transformer plugins. This is because we might have to generate static assets for some plugins that detect the presence of those assets to work. Anything that writes data to the disk, will be an emitter plugin.

In order to add additional functionality such as styles and templates for various pages such as different pages for folders and files, you can use a template engine. I personally prefer using [leptos](https://leptos.dev/) to generate HTML using "to_html" function on leptos components. I use leptos because it offers a components based approach to creating UIs which is kind of the standard in web development and I am used to that as well.

> NOTE: In order for "to_html" function to work you need to enable the "ssr" feature of the leptos crate

# Hosting

Wherever you can use nix flakes to run programs, you can use oro-jackson.<br/>
I am using [Cloudflare Pages](https://pages.cloudflare.com/) this docs website. My setup is extremely easy to replicate for any project.

I am using the following github action to generate the static assets using oro-jackson as a nix flake and then upload those assets to cloudflare to be hosted using the [wrangler-cli](https://developers.cloudflare.com/workers/wrangler/).

> NOTE: You need to create a project on cloudflare before using this github-action and make sure the name of the project and other credentials are correctly setup.

```yaml
name: ci
on:
  push:
    branches: ["main"]
env:
  PROJECT_NAME: --your-project-name-on-cloudflare-pages--
  OUTPUT_DIRECTORY: build
  BUILD_COMMAND: nix run github:lovedeepsingh-07/oro-jackson -- build --config /path/to/theme.toml --theme /path/to/theme.css --content ./docs --output /path/to/your/content/folder
jobs:
  build-and-public-docs:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      deployments: write
    steps:
      - name: Checkout repository
        uses: actions/checkout@v3
      - name: Install Nix
        uses: cachix/install-nix-action@v31
      - run: ${{ env.BUILD_COMMAND }}
      - name: Install Bun
        uses: oven-sh/setup-bun@v2
      - name: Deploy
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          command: pages deploy ${{env.OUTPUT_DIRECTORY}} --project-name=${{env.PROJECT_NAME}}
          gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```
