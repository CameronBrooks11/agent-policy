use crate::error::Result;
use camino::Utf8PathBuf;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Install git hooks for agent-policy.
///
/// # Errors
/// Returns an error if the `.git/hooks` directory cannot be accessed or if writing the hook fails.
pub fn run(pre_push: bool) -> Result<()> {
    let git_dir = Utf8PathBuf::from(".git");
    if !git_dir.exists() {
        return Err(crate::error::Error::Io {
            path: git_dir.into_std_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not a git repository (no .git directory found)",
            ),
        });
    }

    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir).map_err(|e| crate::error::Error::Io {
            path: hooks_dir.clone().into_std_path_buf(),
            source: e,
        })?;
    }

    let hook_name = if pre_push { "pre-push" } else { "pre-commit" };
    let hook_path = hooks_dir.join(hook_name);

    let script = r#"#!/bin/sh
# agent-policy hook
# Fails the commit/push if generated files drift from agent-policy.yaml

echo "Running agent-policy check..."
if ! command -v agent-policy >/dev/null 2>&1; then
    echo "Warning: agent-policy not found in PATH. Skipping check."
    exit 0
fi

agent-policy check
if [ $? -ne 0 ]; then
    echo ""
    echo "agent-policy check failed! Please run 'agent-policy generate' and commit the changes."
    exit 1
fi
"#;

    fs::write(&hook_path, script).map_err(|e| crate::error::Error::Io {
        path: hook_path.clone().into_std_path_buf(),
        source: e,
    })?;

    #[cfg(unix)]
    {
        if let Ok(mut perms) = fs::metadata(&hook_path).map(|m| m.permissions()) {
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&hook_path, perms);
        }
    }

    println!("✅ Successfully installed {hook_name} hook at {hook_path}");
    Ok(())
}
