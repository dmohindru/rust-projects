use clap::Args;

#[derive(Args, Debug)]
pub struct CompleteCommandArgs {
    #[arg(long)]
    pub id: String,
}
