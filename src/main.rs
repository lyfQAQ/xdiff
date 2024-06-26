use anyhow::Result;
use clap::Parser;
use xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.action {
        Action::Run(args) => run(args).await?,
        _ => unimplemented!(),
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    // ExtraArgs 实现了 From<Vec<KeyVal>>
    let extra_args = args.extra_params.into();
    profile.diff(extra_args).await?;
    Ok(())
}
