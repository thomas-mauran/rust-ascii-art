mod commands;

use commands::{CliArgs, EntityType};
use commands::text::{TextSubcommand};
use commands::shapes::ShapeSubcommand;
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.entity_type {
        EntityType::Shape(shape_command) => match shape_command.command{
            ShapeSubcommand::Square(_) => create_square()
        },
        EntityType::Text(text_command) => match text_command.command{
            TextSubcommand::Hello(text_command) => text_command.say_hello()
        },
    }

}

fn create_square() {
    println!("Creating a square...");
}
