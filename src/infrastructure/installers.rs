use crate::infrastructure::shell;
use colored::*;

pub fn install_rust() -> bool {
    println!("   Requesting Rust installation script...");

    if cfg!(target_os = "windows") {
        println!("   On Windows, opening rustup.rs...");
        shell::execute("explorer", &["https://rustup.rs"], ".").is_ok()
    } else {
        // Universal para Mac y Linux
        let command = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh";
        shell::install_script("sh", &["-c", command])
    }
}

pub fn install_dotnet() -> bool {
    println!("   Requesting .NET SDK installation...");

    if cfg!(target_os = "windows") {
        install_dotnet_windows()
    } else if cfg!(target_os = "macos") {
        install_dotnet_mac()
    } else {
        install_dotnet_linux()
    }
}

fn install_dotnet_windows() -> bool {
    // Windows: Winget es el estándar hoy en día
    println!("   Detected Windows. Using Winget...");
    shell::install_script("winget", &["install", "Microsoft.DotNet.SDK.8"])
}

fn install_dotnet_mac() -> bool {
    // Mac: Homebrew es el rey
    if shell::is_installed("brew") {
        println!("   Detected macOS with Homebrew...");
        shell::install_script("brew", &["install", "--cask", "dotnet-sdk"])
    } else {
        println!("   {} Error: Homebrew not found. Install it first or download .NET manually.".red());
        false
    }
}

fn install_dotnet_linux() -> bool {
    println!("   {} Detecting Linux Package Manager...".cyan());

    // 1. Debian / Ubuntu / Mint / Kali
    if shell::is_installed("apt") {
        println!("   Detected 'apt' (Debian/Ubuntu family).");
        return shell::install_script("sudo", &["apt-get", "install", "-y", "dotnet-sdk-8.0"]);
    } 
    
    // 2. Fedora / RHEL / CentOS
    if shell::is_installed("dnf") {
        println!("   Detected 'dnf' (Fedora family).");
        return shell::install_script("sudo", &["dnf", "install", "-y", "dotnet-sdk-8.0"]);
    }

    // 3. Arch Linux / Manjaro
    if shell::is_installed("pacman") {
        println!("   Detected 'pacman' (Arch family).");
        
        return shell::install_script("sudo", &["pacman", "-S", "--noconfirm", "dotnet-sdk"]);
    }

    // 4. Alpine Linux (Muy usado en Docker)
    if shell::is_installed("apk") {
        println!("   Detected 'apk' (Alpine).");
        
        return shell::install_script("sudo", &["apk", "add", "dotnet8-sdk"]);
    }

   
    println!("   {} Warning: No supported package manager found (apt, dnf, pacman, apk).".yellow());
    println!("   Please install .NET SDK 8 manualy from: https://dotnet.microsoft.com/download");
    false
}

pub fn install_avalonia_templates() -> bool {
    println!("   Checking Avalonia templates...");
    
    let check = shell::execute_and_capture("dotnet", &["new", "list"], ".");
    
    if let Ok(output) = check {
        if output.contains("avalonia") {
            println!("   Avalonia templates are already installed.");
            return true;
        }
    }

    println!("   Installing Avalonia UI Templates...");

    shell::execute("dotnet", &["new", "install", "Avalonia.Templates"], ".").is_ok()
}

pub fn install_wpf_distro() -> bool {
    if !cfg!(target_os = "windows") {
        println!("   {} WPF is not supported on this operating system.".red());
        return false;
    }

    println!("   {} WPF missing. Initializing repair protocol...".yellow());
    println!("   Downloading official .NET Desktop components via Winget...");

   
    let runtime_ok = shell::install_script("winget", &[
        "install", "Microsoft.DotNet.DesktopRuntime.8", "--accept-source-agreements", "--accept-package-agreements"
    ]);

    let sdk_ok = shell::install_script("winget", &[
        "install", "Microsoft.DotNet.SDK.8", "--force", "--accept-source-agreements", "--accept-package-agreements"
    ]);

    if runtime_ok && sdk_ok {
        println!("   {} WPF Support installed successfully!".green());
        true
    } else {
        println!("   {} Failed to install WPF components. Try running Orbit as Administrator.".red());
        false
    }
}

pub fn install_docker() -> bool {
    println!("   {} Initializing Docker installation sequence...", "🐳".blue());
    
    if cfg!(target_os = "windows") {
        println!("   Downloading Docker Desktop via Winget (this may take a while)...");
        // Docker.DockerDesktop es el ID oficial en la tienda de Microsoft
        return shell::install_script("winget", &[
            "install", "Docker.DockerDesktop", 
            "--accept-source-agreements", 
            "--accept-package-agreements"
        ]);

    } else if cfg!(target_os = "macos") {
        if shell::is_installed("brew") {
            println!("   Installing Docker via Homebrew...");
            return shell::install_script("brew", &["install", "--cask", "docker"]);
        } else {
            println!("   {} Error: Homebrew is required to install Docker on Mac.".red());
            return false;
        }

    } else {
        // LINUX: Usamos el script oficial de conveniencia de get.docker.com
        // Es el estándar industrial para instalaciones rápidas en servidores
        println!("   Installing Docker Engine via official script...");
        let cmd = "curl -fsSL https://get.docker.com | sh";
        return shell::install_script("sh", &["-c", cmd]);
    }
}