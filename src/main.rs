use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::str::FromStr;

-#[derive(StructOpt)]
struct Cli {
    /// Columns pattern. e.g  1:id,10-15:amt,... 
    pattern: String ,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


struct Field{
    start:u8,
    end:u8,
    name:String,
}

struct FieldParseError{}

impl Error for FieldParseError{}

impl FromStr for Field{
    type Err = FieldParseError;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        x
    }

}
fn main() -> Result<()> {
    let args = Cli::from_args();
    let fields:Vec<&str> = args.pattern.split(",").collect();
    
    
    let file = File::open(args.path)?;
    for line in BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(())
}
