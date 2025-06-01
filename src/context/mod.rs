use crate::{cli, config, error, plugins};
use bon;
use color_eyre::eyre;
use std::path;
use toml;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub struct BuildArgs {
    pub config: path::PathBuf,
    pub content: path::PathBuf,
    pub output: path::PathBuf,
    pub serve: bool,
}

impl From<cli::Build> for BuildArgs {
    fn from(value: cli::Build) -> Self {
        return Self {
            config: path::PathBuf::from(value.config),
            content: path::PathBuf::from(value.content),
            output: path::PathBuf::from(value.output),
            serve: value.serve,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub config: config::Config,
    pub build_args: BuildArgs,
    pub is_rebuild: bool,
    pub build_path: path::PathBuf,
    pub transformer_plugins: Vec<plugins::Transformer>,
    pub emitter_plugins: Vec<plugins::Emitter>,
}

#[bon::bon]
impl Context {
    #[builder]
    pub fn new(
        config_file_content: &str,
        build_args: BuildArgs,
    ) -> eyre::Result<Self, error::Error> {
        let parsed_app_config: config::Config = toml::from_str(config_file_content)?;

        let mut ctx = Context {
            config: parsed_app_config,
            build_path: build_args.content.clone(),
            is_rebuild: false,
            build_args,
            transformer_plugins: Vec::new(),
            emitter_plugins: Vec::new(),
        };

        // transformers
        if ctx.config.plugins.transformers.frontmatter.enable {
            ctx.transformer_plugins
                .push(plugins::transformers::frontmatter::frontmatter_transformer)
        }
        if ctx.config.plugins.transformers.markdown.enable {
            ctx.transformer_plugins
                .push(plugins::transformers::markdown::markdown_transformer)
        }
        if ctx.config.plugins.transformers.sanitize.enable {
            ctx.transformer_plugins
                .push(plugins::transformers::sanitize::sanitize_transformer)
        }

        // emitters
        if ctx.config.plugins.emitters.static_assets.enable {
            ctx.emitter_plugins
                .push(plugins::emitters::static_assets::static_assets_emitter)
        }
        if ctx.config.plugins.emitters.file_page.enable {
            ctx.emitter_plugins
                .push(plugins::emitters::file_page::file_page_emitter)
        }
        if ctx.config.plugins.emitters.folder_page.enable {
            ctx.emitter_plugins
                .push(plugins::emitters::folder_page::folder_page_emitter)
        }

        return Ok(ctx);
    }
}
