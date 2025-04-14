use zix_core::grid;
use colored::Colorize;

fn main()   {
    let output = grid::out::<String>(
        vec![
            "gaga".red().to_string(),
            "gaga".red().to_string(),
            "gaga".red().to_string(),
            " gaga".red().to_string(),
            "gaga".red().to_string(),
            " gaga".red().to_string(),
            "gfdsafasdfasdfasdfaafsdadssssssssssssssssssssssssssssssssssssssssaga".red().to_string(),
            "gagadfsa".red().to_string(),
            " gaga".red().to_string(),
            "gaga".red().to_string(),
            " gaga".red().to_string(),
            "gaga".red().to_string(),
            "gdsafas".red().to_string(),
            "gaga".red().to_string(),
            "gads".red().to_string(),
            "gaga".red().to_string(),
            "gafdsafdsaga".red().to_string(),
            "gaga".red().to_string(),
            "gaga".red().to_string(),
            "gaga".red().to_string(),
            "gfdsafdsaaga".red().to_string(),
            "gogo".blue().to_string(),
            "gugu".blue().to_string(),
            "gafdasssssssssssssssssssfasddddddddddddddddgadfas".red().to_string(),
        ]
    );

    println!("{}", output)

}
