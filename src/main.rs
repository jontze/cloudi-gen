#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate miette;

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
    /// Disable the fail2ban installation. By default, fail2ban is installed.
    #[arg(long = "no-fail2ban", default_value = "false")]
    no_fail2ban: bool,
    /// Allow SSH tcp tunneling. By default, SSH tunneling is disabled.
    /// This is a security risk and should only be enabled if you know what you are doing.
    #[arg(long = "allow-ssh-tcp-forward", default_value = "false")]
    allow_ssh_tcp_forward: bool,
    /// Allow SSH X11 forwarding. By default, SSH X11 forwarding is disabled.
    /// This is a security risk and should only be enabled if you know what you are doing.
    #[arg(long = "allow-ssh-x11-forward", default_value = "false")]
    allow_ssh_x11_forward: bool,
    /// Allow SSH agent forwarding. By default, SSH agent forwarding is disabled.
    /// This is a security risk and should only be enabled if you know what you are doing.
    #[arg(long = "allow-ssh-agent-forward", default_value = "false")]
    allow_ssh_agent_forward: bool,
}

fn main() -> miette::Result<()> {
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
        let user = user_builder.build()?;
        cloud_init_builder.users(vec![user]);
    };

    // Add Packages if provided
    if let Some(packages) = args.packages {
        for package in packages.into_iter() {
            cloud_init_builder.add_package(package);
        }
    };

    // Enable X11 forwarding only if requested
    if !args.allow_ssh_x11_forward {
        cloud_init_builder.disallow_ssh_x11_forward();
    }

    // Enable SSH agent forwarding only if requested
    if !args.allow_ssh_agent_forward {
        cloud_init_builder.disallow_ssh_agent_forward();
    }

    // Enable SSH tcp forwarding only if requested
    if !args.allow_ssh_tcp_forward {
        cloud_init_builder.disallow_ssh_tcp_forward();
    }

    // Add fail2ban if not disabled
    if !args.no_fail2ban {
        cloud_init_builder.with_fail2ban();
    }

    let cloud_init = cloud_init_builder.build()?;

    cloud_init.print(args.pretty_print);

    Ok(())
}
