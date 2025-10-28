use clap::{ArgGroup, Parser};
use math_ops::{div, mul, sub, sum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("operation")
        .required(true)
        .args(["sum", "sub", "mul", "div"]),
))]
pub struct SimpleOpsCli {
    /// First operand
    #[arg(long)]
    pub op1: u64,
    /// Second operand
    #[arg(long)]
    pub op2: u64,
    /// Switch for add operation
    #[arg(long)]
    pub sum: bool,
    /// Switch for subtract operation
    #[arg(long)]
    pub sub: bool,
    /// Switch for multiplication operation
    #[arg(long)]
    pub mul: bool,
    /// Switch for division operation
    #[arg(long)]
    pub div: bool,
}
fn main() {
    // Parse command line arguments
    let cli = SimpleOpsCli::parse();

    // Perform the requested operation
    let result = if cli.sum {
        sum(cli.op1, cli.op2)
    } else if cli.sub {
        sub(cli.op1, cli.op2)
    } else if cli.mul {
        mul(cli.op1, cli.op2)
    } else if cli.div {
        div(cli.op1, cli.op2)
    } else {
        unreachable!("clap ensures one operation is selected")
    };
    // Print the result
    println!("Result: {}", result);
}
