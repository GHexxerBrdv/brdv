use self_update::backends::github::Update;
use std::error::Error;

pub fn update() -> Result<(), Box<dyn Error>> {
    let status = Update::configure()
        .repo_owner("GHexxarBrdv")
        .repo_name("brdv")
        .bin_name("brdv")
        .show_download_progress(true)
        .current_version("0.1.0")
        .build()?
        .update()?;

    println!("Update status: {:?}", status);
    Ok(())
}
