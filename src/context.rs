use crate::{cli, config, error, helpers, plugins};
use bon;
use color_eyre::eyre;
use toml;
use vfs;

#[derive(Debug, Clone, bon::Builder)]
pub struct BuildArgs {
    pub content: vfs::VfsPath,
    pub output: vfs::VfsPath,
    pub serve: bool,
    pub cli_args: cli::Build,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub config: config::Config,
    pub theme: String,
    pub build_args: BuildArgs,
    pub is_rebuild: bool,
    pub build_path: vfs::VfsPath,
    pub transformer_plugins: Vec<plugins::Transformer>,
    pub emitter_plugins: Vec<plugins::Emitter>,
    pub file_tree: Vec<helpers::file_tree::TreeNode>,
}

#[bon::bon]
impl Context {
    #[builder]
    pub fn new(
        build_args: BuildArgs,
        config_file_content: &str,
        theme_file_content: &str,
    ) -> eyre::Result<Self, error::Error> {
        let parsed_app_config: config::Config = toml::from_str(config_file_content)?;

        let file_tree: Vec<helpers::file_tree::TreeNode> = helpers::file_tree::map_folder()
            .input_path(build_args.content.clone())
            .call()?;

        let mut ctx = Context {
            config: parsed_app_config.clone(),
            theme: theme_file_content.to_string(),
            build_path: build_args.content.clone(),
            is_rebuild: false,
            build_args,
            transformer_plugins: Vec::new(),
            emitter_plugins: Vec::new(),
            file_tree,
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
