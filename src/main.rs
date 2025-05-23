mod lexer;
use sprunk::MultiPeekCheckExt;
use std::{error::Error, io, process, env};

#[derive(Debug, serde::Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn example(file_path: &str) -> Result<(), Box<dyn Error>> {
    //let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    //let query = &args[1];

    if let Err(err) = example(file_path) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    let mut query: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut query) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

    println!("You entered: {}", query);
    let tokens: Vec<lexer::TokenType> = lexer::lexer(&query);

    for token in &tokens {
        println!("{}", token);
    }

    println!("\rFiltering Tokens!\r");
    let filtered_tokens: Vec<lexer::TokenType> = tokens.into_iter().filter(|x| match x {
                                               lexer::TokenType::Whitespace(_) => false,
                                               _ => true,
                                               })
                                            .collect();
    
    filtered_tokens.iter().for_each(|x| println!("{}", x));



    let mut iter = filtered_tokens.iter(); // shared, non-consuming

    if iter.multi_peek_check(&[lexer::TokenType::Pipe,
                                        lexer::TokenType::StatsKeyword,
                                        lexer::TokenType::CountKeyword]) {
        println!("Pattern matched!");
    } else {
        println!("No match.");
    }

    iter.next();
    iter.next();
    iter.next();

        if iter.multi_peek_check(&[lexer::TokenType::Pipe,
                                        lexer::TokenType::StatsKeyword,
                                        lexer::TokenType::CountKeyword]) {
        println!("Pattern matched!");
    } else {
        println!("No match.");
    }

    match iter {
        x if x.multi_peek_check(&[lexer::TokenType::Pipe,
                                        lexer::TokenType::StatsKeyword,
                                        lexer::TokenType::CountKeyword]) => println!("Pipe Stats Count"),
        x if x.multi_peek_check(&[lexer::TokenType::Pipe,
                                        lexer::TokenType::ByKeyword]) => println!("Pipe By"),
        _ => println!("No match")            
    }

    // Still usable
    println!("Next item: {:?}", iter.next()); // Still 10
    

}

