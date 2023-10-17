use bat::PrettyPrinter;

#[derive(Default, Builder, Serialize, Debug, Clone)]
#[builder(build_fn(private, name = "build_internal"))]
pub(crate) struct CloudData {
    #[builder(default)]
    chpasswd: ChPasswd,
    #[builder(default = "false")]
    ssh_pwauth: bool,
    #[builder(default = "true")]
    disable_root: bool,
    #[builder(default = "true")]
    package_update: bool,
    #[builder(default = "true")]
    package_upgrade: bool,
    #[builder(default = "true")]
    package_reboot_if_required: bool,
    #[builder(default = "vec![]")]
    packages: Vec<String>,
    #[builder(default = "vec![]")]
    write_files: Vec<WriteFile>,
    #[builder(default = "vec![]")]
    users: Vec<User>,
    #[builder(default = "vec![]")]
    runcmd: Vec<String>,
}

impl CloudData {
    pub(crate) fn print(&self, is_pretty: bool) {
        let yaml_output = serde_yaml::to_string(self).unwrap();
        let file_content = format!("#cloud-config\n{yaml_output}");

        let mut printer = PrettyPrinter::new();
        printer.input_from_bytes(file_content.as_bytes());
        printer.colored_output(false);

        if is_pretty {
            printer.grid(true);
            printer.language("yaml");
            printer.line_numbers(true);
            printer.colored_output(true);
        }
        printer.print().unwrap();
    }
}

impl CloudDataBuilder {
    pub(crate) fn build(mut self) -> Result<CloudData, CloudDataBuilderError> {
        if let Some(runcmds) = &self.runcmd {
            if !runcmds.is_empty() {
                self.add_runcmd(String::from("reboot"));
            }
        }
        let cloud_data = self.build_internal()?;
        Ok(cloud_data)
    }

    pub(crate) fn add_runcmd(&mut self, runcmd: String) -> &mut Self {
        self.runcmd
            .get_or_insert_with(std::vec::Vec::new)
            .push(runcmd);
        self
    }

    pub(crate) fn add_package(&mut self, package: String) -> &mut Self {
        self.packages
            .get_or_insert_with(std::vec::Vec::new)
            .push(package);
        self
    }

    pub(crate) fn add_write_file(&mut self, write_file: WriteFile) -> &mut Self {
        self.write_files
            .get_or_insert_with(std::vec::Vec::new)
            .push(write_file);
        self
    }

    pub(crate) fn disallow_ssh_agent_forward(&mut self) -> &mut Self {
        let disable_agent_forward = String::from(
            "sed -i -e '/^\\(#\\|\\)AllowAgentForwarding/s/^.*$/AllowAgentForwarding no/' /etc/ssh/sshd_config",
        );
        self.add_runcmd(disable_agent_forward);
        self
    }

    pub(crate) fn disallow_ssh_tcp_forward(&mut self) -> &mut Self {
        let disable_tcp_forward = String::from(
            "sed -i -e '/^\\(#\\|\\)AllowTcpForwarding/s/^.*$/AllowTcpForwarding no/' /etc/ssh/sshd_config",
        );
        self.add_runcmd(disable_tcp_forward);
        self
    }

    pub(crate) fn disallow_ssh_x11_forward(&mut self) -> &mut Self {
        let disable_x11_forward = String::from(
            "sed -i -e '/^\\(#\\|\\)X11Forwarding/s/^.*$/X11Forwarding no/' /etc/ssh/sshd_config",
        );
        self.add_runcmd(disable_x11_forward);
        self
    }

    pub(crate) fn with_fail2ban(&mut self) -> &mut Self {
        // Install package
        self.add_package(String::from("fail2ban"));

        // Configure ssh jail
        let ssh_jail = String::from(
            "[sshd]\nenabled = true\nport = ssh\nfilter = sshd\nlogpath = /var/log/auth.log\nmaxretry = 3\nbantime = 600",
        );
        let write_file = WriteFile {
            path: String::from("/etc/fail2ban/jail.d/ssh.conf"),
            content: ssh_jail,
        };
        self.add_write_file(write_file);

        // Enable and start service
        for enable_cmds in [String::from("systemctl enable fail2ban"),
            String::from("systemctl start fail2ban")] {
            self.add_runcmd(enable_cmds);
        }
        self
    }
}

#[derive(Default, Builder, Serialize, Debug, Clone)]
pub(crate) struct ChPasswd {
    #[builder(default = "false")]
    expire: bool,
}

#[derive(Builder, Serialize, Debug, Clone, Default)]
pub(crate) struct User {
    name: String,
    #[builder(default = "String::from(\"ALL=(ALL) NOPASSWD:ALL\")")]
    sudo: String,
    #[builder(default = "vec![String::from(\"adm\"), String::from(\"sudo\")]")]
    groups: Vec<String>,
    #[builder(default = "String::from(\"/bin/bash\")")]
    shell: String,
    ssh_import_id: Vec<String>,
}

impl UserBuilder {
    pub(crate) fn add_ssh_import_id(&mut self, ssh_import_id: String) {
        self.ssh_import_id
            .get_or_insert_with(std::vec::Vec::new)
            .push(ssh_import_id);
    }
}

#[derive(Builder, Serialize, Debug, Clone)]
pub(crate) struct WriteFile {
    path: String,
    content: String,
}
