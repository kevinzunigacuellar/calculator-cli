use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
   #[clap(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add two numbers
   Add { number_one: i32, number_two: i32 },
    /// subtract two numbers
   Substract { number_one: i32, number_two: i32 },
    /// multiply two numbers
   Multiply { number_one: i32, number_two: i32 },
    /// divide two numbers
   Divide { number_one: i32, number_two: i32 },
}

fn main() {
   let value = Value::parse();

   // You can check for the existence of subcommands, and if found use their
   // matches just as you would the top level app
   match &value.command {
       Commands::Add { number_one, number_two } => {
           println!("The answer is: {:?}", number_one + number_two)
       },
       Commands::Substract { number_one, number_two } => {
           println!("The answer is: {:?}", number_one - number_two)
       },
       Commands::Multiply { number_one, number_two } => {
           println!("The answer is: {:?}", number_one * number_two)
       },
       Commands::Divide { number_one, number_two } => {
           println!("The answer is: {:?}", number_one / number_two)
       },
   }
}