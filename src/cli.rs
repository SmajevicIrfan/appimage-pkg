use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "appimage-pkg")]
#[command(bin_name = "appimage-pkg")]
#[command(version, about = "A simple AppImage package manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install an AppImage from URL or file path
    #[command(arg_required_else_help = true)]
    Install {
        /// URL from which to install AppImage
        #[arg(long, short = 'u', conflicts_with = "file")]
        url: Option<String>,

        /// File from which to install AppImage
        #[arg(long, short = 'f', conflicts_with = "url")]
        file: Option<String>,
    },
    /// List installed AppImages
    List {
        /// Query installed AppImages
        #[arg(long, short = 'q')]
        query: Option<String>,
    },
    /// Remove an installed AppImage
    Remove {
        /// Name of the AppImage to remove
        name: String,
    },
}
