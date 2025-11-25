use zed_extension_api as zed;

struct SomaExtension;

impl zed::Extension for SomaExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed::Command {
            command: "souls".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(SomaExtension);
