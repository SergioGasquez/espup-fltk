use anyhow::Result;
use espup::{
    install,
    targets::{parse_targets, Target},
    toolchain::rust::XtensaRust,
    InstallOpts,
};
use fltk::{app, prelude::*};
use std::{collections::HashSet, path::PathBuf};
mod ui;

#[cfg(target_os = "windows")]
const DEFAULT_HOST: &str = "x86_64-pc-windows-msvc";
#[cfg(target_os = "linux")]
const DEFAULT_HOST: &str = "x86_64-unknown-linux-gnu";
#[cfg(target_os = "macos")]
const DEFAULT_HOST: &str = "aarch64-apple-darwin";

// TODO: Make it public in espup
#[cfg(windows)]
const DEFAULT_EXPORT_FILE: &str = "export-esp.ps1";
#[cfg(not(windows))]
const DEFAULT_EXPORT_FILE: &str = "export-esp.sh";

fn main() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut ui = ui::UserInterface::make_window();

    ui.default_host_text_input.set_value(DEFAULT_HOST);
    ui.export_file_text_input.set_value(DEFAULT_EXPORT_FILE);
    ui.llvm_version.set_value("15");
    ui.llvm_version.add("15");
    ui.nightly_version_text_input.set_value("nightly");
    ui.toolchain_version_text_input
        .set_value(XtensaRust::get_latest_version()?.as_str());
    ui.esp_idf.set_value("release/v4.4");
    ui.esp_idf.add("release/v4.4");
    ui.esp_idf.add("release/v5.0");
    ui.esp_idf.add("master");
    ui.esp_idf.add("v4.4.1");
    ui.esp_idf.add("v4.4.2");
    ui.esp_idf.add("v4.4.3");
    ui.log_level.set_value("info");
    ui.log_level.add("debug");
    ui.log_level.add("info");
    ui.log_level.add("warn");
    ui.log_level.add("error");

    let targets = parse_targets("all").unwrap();
    ui.install_button.set_callback(move |_| {
        let opts = InstallOpts {
            default_host: Some(ui.default_host_text_input.value()),
            esp_idf_version: ui.esp_idf.value(),
            export_file: Some(PathBuf::from(ui.export_file_text_input.value())),
            extra_crates: None,
            llvm_version: ui.llvm_version.value().unwrap(),
            log_level: ui.log_level.value().unwrap(),
            nightly_version: ui.nightly_version_text_input.value(),
            profile_minimal: true,
            targets: targets.clone(),
            toolchain_version: None,
        };
        println!("{:?}", opts);
        install(opts);
        // ui.install_button.set_label("Installation done");
        println!("Installation done");
    });

    app.run().unwrap();
    Ok(())
}
