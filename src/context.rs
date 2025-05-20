use crate::{cli, config, error, plugins};
use bon;
use color_eyre::eyre;

#[derive(Debug, Clone)]
pub struct Context {
    pub config: config::Config,
    pub build_args: cli::Build,
    pub is_rebuild: bool,
    pub build_path: String,
    pub transformer_plugins: Vec<plugins::Transformer>,
    pub emitter_plugins: Vec<plugins::Emitter>,
}

#[bon::bon]
impl Context {
    #[builder]
    pub fn new(
        app_config: config::Config,
        build_args: cli::Build,
    ) -> eyre::Result<Self, error::Error> {
        let mut ctx = Context {
            config: app_config,
            build_path: build_args.clone().content,
            is_rebuild: false,
            build_args,
            transformer_plugins: Vec::new(),
            emitter_plugins: Vec::new(),
        };

        if ctx.config.plugins.transformers.markdown.enable {
            ctx.transformer_plugins
                .push(plugins::transformers::markdown::markdown_transformer)
        }
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
