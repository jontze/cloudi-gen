# Cloudi-Gen

`cloudi-gen` is a CLI tool that aids in the generation of cloud-init configurations, making it easier for you to set up cloud instances with your desired configurations.

## Features

1. **User Creation**: Easily create a user on your cloud instance with a specified username.
2. **SSH Key Import from GitHub**: Import SSH keys directly from a GitHub handle, streamlining the setup process.
3. **Package Installation**: Specify packages you want installed on your cloud instance right from the start.
4. **Fail2Ban Configuration**: Enhance security with `fail2ban`. By default, it's installed, but you have the option to disable it.
5. **SSH Configurations**: Customize SSH settings, including:
   - Allowing/Disallowing SSH TCP tunneling.
   - Allowing/Disallowing SSH X11 forwarding.
   - Allowing/Disallowing SSH agent forwarding.
6. **Pretty Print**: Get a neatly formatted output. By default, the output is machine-readable, but you can opt for a more human-friendly version.

## Usage Example

```bash
cloudi-gen -u jontze -g jontze -p curl -p jq > cloud-init.yml
```

With this command:

- A user named `jontze` will be created on the cloud instance
- The SSH keys of the GitHub user `jontze` will be imported
- The packages `curl` and `jq` will be installed
- The cloud-init configuration will be saved to `cloud-init.yml`

## Installation

### From GitHub Releases

1. Navigate to the [Releases](https://github.com/jontze/cloudi-gen/releases) section of the `cloudi-gen` GitHub repository.
2. Download the appropriate executable for your operating system from the latest release.
3. Save the downloaded executable to a directory that's in your system's PATH.
4. Make the executable file runnable (for Linux/macOS users, you can use the command `chmod +x /path/to/cloudi-gen`).
5. You can now run `cloudi-gen` from your terminal!

### Compile from Source

If you have `cargo` installed, you can compile `cloudi-gen` from source:

   ```bash
   cargo install --git https://github.com/jontze/cloudi-gen
   ```

You can now run `cloudi-gen` from your terminal!
