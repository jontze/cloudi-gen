use bat::PrettyPrinter;

#[derive(Default, Builder, Serialize, Debug, Clone)]
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
    pub(crate) fn add_package(&mut self, package: String) {
        self.packages
            .get_or_insert_with(std::vec::Vec::new)
            .push(package);
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
