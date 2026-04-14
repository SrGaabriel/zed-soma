use zed::lsp::CompletionKind;
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, Result};

struct SomaExtension;

impl SomaExtension {
    pub fn find_souls(&self, worktree: &zed::Worktree) -> Result<String> {
        if let Some(path) = worktree.which("souls") {
            return Ok(path);
        }
        Err("Could not find 'souls' in the worktree.".into())
    }
}

impl zed::Extension for SomaExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let name = &completion.label;
        let ty = completion.detail?;

        let (code, name_range, sep_range, ty_range) = match completion.kind? {
            CompletionKind::Constructor => {
                let prefix = "def a : ";
                let sep = " : ";
                let assign = " = ";
                let code = format!("{prefix}{ty}{assign}{name}");
                let ty_start = prefix.len();
                let ty_end = ty_start + ty.len();
                let name_start = ty_end + assign.len();
                let sep_start = "def a".len();
                (
                    code,
                    name_start..name_start + name.len(),
                    sep_start..sep_start + sep.len(),
                    ty_start..ty_end,
                )
            }
            _ => {
                let prefix = "def ";
                let sep = " : ";
                let code = format!("{prefix}{name}{sep}{ty} = x");
                let name_start = prefix.len();
                let name_end = name_start + name.len();
                let sep_start = name_end;
                let ty_start = sep_start + sep.len();
                (
                    code,
                    name_start..name_end,
                    sep_start..sep_start + sep.len(),
                    ty_start..ty_start + ty.len(),
                )
            }
        };

        Some(CodeLabel {
            spans: vec![
                CodeLabelSpan::code_range(name_range),
                CodeLabelSpan::code_range(sep_range),
                CodeLabelSpan::code_range(ty_range),
            ],
            filter_range: (0..name.len()).into(),
            code,
        })
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        let command = self.find_souls(worktree)?;
        Ok(zed::Command {
            command: command,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(SomaExtension);
