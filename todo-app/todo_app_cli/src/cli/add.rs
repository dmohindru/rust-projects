use clap::Args;

#[derive(Args, Debug)]
pub struct AddCommandArgs {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub description: String,
}
