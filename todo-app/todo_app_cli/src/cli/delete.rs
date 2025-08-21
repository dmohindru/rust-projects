use clap::Args;

#[derive(Args, Debug)]
pub struct DeleteCommandArgs {
    #[arg(long)]
    pub id: String,
}
