use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct OdooLspExtension;

impl zed::Extension for OdooLspExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match worktree.which("odoo-lsp") {
            Some(path) => Ok(zed::Command {
                command: path,
                args: Vec::new(),
                env: vec![("RUST_LOG".into(), "odoo_lsp=debug".into())],
            }),
            None => Err("Unable to find odoo-lsp from worktree".into()),
        }
    }
}

zed::register_extension!(OdooLspExtension);
