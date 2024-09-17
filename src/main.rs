use sysinfo::System;
use users::get_current_username;
mod essentials;
mod pkg_counter;
use essentials::{get_formatting, get_uptime};
use pkg_counter::PackageManager;

struct SystemInfo {
    os: String,
    kernel_version: String,
    hostname: String,
    username: String,
    uptime: String,
    pkg_count: String,
}

impl SystemInfo {
    fn build() -> Self {
        let os = System::name().unwrap_or("Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or("Unknown".to_string());
        let hostname = System::host_name().unwrap_or("Unknown".to_string());
        let user = get_current_username().unwrap();
        let username = user.to_string_lossy().to_string();
        let uptime = get_uptime();
        let pkg_manager = PackageManager::build();
        let pkg_count = pkg_manager.pkgs.to_string();

        Self {
            os,
            kernel_version,
            hostname,
            username,
            uptime,
            pkg_count,
        }
    }

    fn display(&self, color: &str, bold: bool) -> String {
        let reset = get_formatting("reset");
        let color_escape = match (bold, color) {
            (false, c) => get_formatting(c),
            (true, c) => format!("{}{}", get_formatting("bold"), get_formatting(c)),
        };

        format!(
            "{color_escape}{}{reset}{}{color_escape}{}\nOS: {reset}{}\n{color_escape}KERNEL: {reset}{}\n{color_escape}UPTIME: {reset}{}\n{color_escape}PACKAGES: {reset}{}",
            self.username, "@", self.hostname, self.os, self.kernel_version, self.uptime, self.pkg_count
        )
    }
}

fn main() {
    let system_info = SystemInfo::build();
    println!("{}", system_info.display("green", true));
}
