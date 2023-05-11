use clap:: {
    Args,
    Parser,
    Subcommand
};

// TEXT
#[derive(Debug, Args)]
pub struct TextCommand{
    #[clap(subcommand)]
    pub command: TextSubcommand
}

#[derive(Debug, Subcommand)]
pub enum TextSubcommand{
    /// Say hello to the name you want
    Hello(HelloCommand)
}

#[derive(Debug, Args)]
pub struct HelloCommand{
    pub name: String
}

// implement the helloCommand
impl HelloCommand{
    pub fn say_hello(&self){
        println!("Hello, {}!", self.name)
    }
}