use clap:: {
    Args,
    Parser,
    Subcommand
};

// SHAPES
#[derive(Debug, Args)]
pub struct ShapeCommand{
    #[clap(subcommand)]
    pub command: ShapeSubcommand
}

#[derive(Debug, Subcommand)]
pub enum ShapeSubcommand{
    /// Creates a beautiful square
    Square(SquareShape)
}

#[derive(Debug, Args)]
pub struct SquareShape{

}