#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;

use clap::Parser;

mod cloud_init;

use cloud_init::{CloudDataBuilder, UserBuilder};

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    /// Name of the user to create on the system.
    #[arg(short = 'u', long)]
    username: Option<String>,
    /// GitHub Handle to use for the SSH Key import. Can be used multiple times.
    #[arg(short = 'g', long = "github")]
    github_handle: Option<Vec<String>>,
    /// Pretty Print the output. Default is machine readable.
    #[arg(long = "pretty", default_value = "false")]
    pretty_print: bool,
    /// Package to install on the system. Can be used multiple times. By default, no packages are installed.
    #[arg(short = 'p', long = "package")]
    packages: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    let mut cloud_init_builder = CloudDataBuilder::default();

    // Add User if provided
    if let Some(username) = args.username {
        let mut user_builder = UserBuilder::default();
        user_builder.name(username);
        if let Some(github_handle) = args.github_handle {
            let gh_ssh_import_ids: Vec<String> = github_handle
                .into_iter()
                .map(|handle| format!("gh:{handle}"))
                .collect();
            for gh_import_id in gh_ssh_import_ids {
                user_builder.add_ssh_import_id(gh_import_id);
            }
        }
        let user = user_builder.build().unwrap();
        cloud_init_builder.users(vec![user]);
    };

    // Add Packages if provided

    if let Some(packages) = args.packages {
        for package in packages.into_iter() {
            cloud_init_builder.add_package(package);
        }
    };

    let cloud_init = cloud_init_builder.build().unwrap();

    cloud_init.print(args.pretty_print);
}
