use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum SetupCommand {
    Server,
    Client {
        #[arg(long)]
        server: String,
    },
}
