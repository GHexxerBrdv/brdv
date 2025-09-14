use self_update::backends::github::Update;

pub fn update() -> anyhow::Result<()> {
    let status = Update::configure()
        .repo_owner("GHexxarBrdv")
        .repo_name("brdv")
        .bin_name("brdv")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    println!("Update status: {}", status.version());
    Ok(())
}
