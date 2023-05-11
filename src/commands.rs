use clap:: {
    Args,
    Parser,
    Subcommand
};
use self::text::TextCommand;
pub(crate) mod text;

pub(crate) mod shapes;
use self::shapes::ShapeCommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}


// TYPE OF COMMANDS
#[derive(Debug, Subcommand)]
pub enum EntityType{
    /// Create the 3D shape you want 
    Shape(ShapeCommand),

    /// Text related ascii art commands
    Text(TextCommand),
}

