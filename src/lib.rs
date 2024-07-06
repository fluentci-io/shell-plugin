use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn bash(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bash", "-c", &format!("'{}'", args)])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn fish(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "fish", "-c", &format!("'{}'", args)])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn zsh(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "zsh", "-c", &format!("'{}'", args)])?
        .stdout()?;
    Ok(stdout)
}
