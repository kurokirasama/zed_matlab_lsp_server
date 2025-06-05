use zed_extension_api as zed;
use zed::{Extension, LanguageServerId, Result, Command, Worktree};

struct MatlabExtension;

impl Extension for MatlabExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        Ok(Command {
            command: "/usr/bin/node".to_string(), // The Node.js executable
            args: vec![
                "/home/kira/software/MATLAB-language-server/out/index.js".to_string(), // The main server script
                "--stdio".to_string(),
                "--matlabInstallPath".to_string(),
                "/usr/local/MATLAB/R2024b".to_string(), // Your MATLAB installation path
                // Adding the settings as command-line arguments - Assuming --settingName value format
                "--MATLAB.indexWorkspace".to_string(), "true".to_string(),
                "--MATLAB.matlabConnectionTiming".to_string(), "onStart".to_string(),
                "--MATLAB.telemetry".to_string(), "false".to_string(),
            ],
            env: Default::default(), // No specific environment variables required from the config
        })
    }
}

zed::register_extension!(MatlabExtension);
