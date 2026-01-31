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

// --- Lógica Específica por Sistema ---

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
        // Nota: En Ubuntu modernos .NET ya está en los repos. En viejos podría fallar si no agregan el repo de Microsoft.
        // Usamos 'sudo' explícitamente porque install_script lo hereda.
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
        // En Arch se llama simplemente 'dotnet-sdk'
        return shell::install_script("sudo", &["pacman", "-S", "--noconfirm", "dotnet-sdk"]);
    }

    // 4. Alpine Linux (Muy usado en Docker)
    if shell::is_installed("apk") {
        println!("   Detected 'apk' (Alpine).");
        // Alpine necesita habilitar repos comunitarios a veces, pero intentamos el install directo
        return shell::install_script("sudo", &["apk", "add", "dotnet8-sdk"]);
    }

    // 5. Fallback (Gentoo, Slackware, NixOS, etc.)
    println!("   {} Warning: No supported package manager found (apt, dnf, pacman, apk).".yellow());
    println!("   Please install .NET SDK 8 manualy from: https://dotnet.microsoft.com/download");
    false
}