use std::collections::HashMap;
use std::io;

use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
enum GridBox {
    X,
    O,
    Blank,
}


fn make_move(grid: [GridBox; 9]) -> [GridBox; 9] {
    for i in 0..9 {
        let mut new_grid = grid.clone();
        if new_grid[i] == GridBox::Blank {
            new_grid[i] = GridBox::X;
            if is_over(new_grid).0 {
                return new_grid;
            }
        }
    }
    for i in 0..9 {
        let mut new_grid = grid.clone();
        if new_grid[i] == GridBox::Blank {
            new_grid[i] = GridBox::O;
            if is_over(new_grid).0 {
                new_grid[i] = GridBox::X;
                return new_grid;
            }
        }
    }
    // 012
    // 345
    // 678


    match grid {
    [GridBox::O, GridBox::Blank,GridBox::Blank,
    GridBox::Blank, GridBox::X, GridBox::Blank,
    GridBox::Blank, GridBox::O, GridBox::Blank
    ] |
    [GridBox::O, GridBox::Blank,GridBox::Blank,
    GridBox::Blank, GridBox::X, GridBox::Blank,
    GridBox::Blank, GridBox::Blank, GridBox::O
    ] => {
        let mut new_grid = grid.clone();
        new_grid[5] = GridBox::X;
        return new_grid
    }
    [GridBox::Blank, GridBox::Blank,GridBox::O,
    GridBox::Blank, GridBox::X, GridBox::Blank,
    GridBox::O, GridBox::Blank, GridBox::Blank
    ] |
    [GridBox::Blank, GridBox::Blank,GridBox::O,
    GridBox::Blank, GridBox::X, GridBox::Blank,
    GridBox::Blank, GridBox::O, GridBox::Blank
    ] => {
        let mut new_grid = grid.clone();
        new_grid[5] = GridBox::X;
        return new_grid
    }

    _ => {}
    }

    for i in [4,0,2,6,8,1,3,5,7] {
        let mut new_grid = grid.clone();

        if new_grid[i] == GridBox::Blank {
            new_grid[i] = GridBox::X;
            return new_grid;
        }
    }
    
    grid
}

fn is_over(grid: [GridBox; 9]) -> (bool, GridBox) {
    for i in 0..3 {
        if grid[i] == grid[i + 3] && grid[i] == grid[i + 6] && grid[i] != GridBox::Blank {
            return (true, grid[i]);
        }
    }
    for i in 0..3 {
        if grid[3 * i] == grid[3 * i + 1]
            && grid[3 * i] == grid[3 * i + 2]
            && grid[3 * i] != GridBox::Blank
        {
            return (true, grid[i*3]);
        }
    }
    if grid[0] == grid[4] && grid[0] == grid[8] && grid[0] != GridBox::Blank {
        return (true, grid[0]);
    }
    if grid[2] == grid[4] && grid[2] == grid[6] && grid[2] != GridBox::Blank {
        return (true, grid[2]);
    }

    

    (false, GridBox::Blank)
}

fn main() {
    let mut free_pg = 1;
    let mut options: HashMap<[GridBox; 9], usize> = HashMap::new();


    let mut game_states: Vec<Vec<[GridBox; 9]>> = vec![vec![[GridBox::Blank; 9],[GridBox::Blank; 9]]];
    game_states[0][1][4] = GridBox::X;

    for first_game in &game_states[0] {
        if !options.contains_key(first_game) {
                options.insert(first_game.clone(), free_pg);
                free_pg += 1;
        }
    }

    loop {
        if game_states.last() == Some(&vec![]) {
            break;
        }

        
        
        game_states.push(vec![]);
        for grid in game_states[game_states.len() - 2].clone() {
            for index in 0..9 {

                let mut clone_grid = grid.clone();

                if is_over(clone_grid).0 {
                        if is_over(clone_grid).1 == GridBox::O {
                            println!("OH NO the bot lost, {:?}", clone_grid)
                        }
                        continue;
                }

                if  clone_grid[index] != GridBox::Blank  {
                    continue;
                }
                // bot move
                clone_grid[index] = GridBox::O;
                


                clone_grid = make_move(clone_grid);

                game_states.last_mut().expect("long_list").push(clone_grid);


                if !options.contains_key(&clone_grid) {
                options.insert(clone_grid, free_pg);
                free_pg += 1;
            }
                }
            }
        }

        let mut book_with_pg_numbers: Vec<(&[GridBox;9], usize)> = options.iter().enumerate().map(|(k, v)|  (v.0, *v.1)).collect::<Vec<(&[GridBox;9], usize)>>();
        book_with_pg_numbers.sort_by(|a, b| a.1.cmp(&b.1));
        let book = book_with_pg_numbers.iter().map(|(k, v)| **k).collect::<Vec<[GridBox;9]>>();


        let mut file = OpenOptions::new()
            .write(true)
            // .append(true)
            .truncate(true)
            .open("./PDF/book.md")
            .unwrap();


        if let Err(e) = write!(file, "") {
            eprintln!("Couldn't write to file: {}", e);
        }

        if let Err(e) = writeln!(file, "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\"><link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin><link href=\"https://fonts.googleapis.com/css2?family=Libre+Caslon+Text&display=swap\" rel=\"stylesheet\">
<style> @import url('https://fonts.googleapis.com/css2?family=Libre+Caslon+Text&display=swap'); </style>

<style>

    document {{

    }}

    .page {{
        height: 100%;
        width: 1000px;
        background_color: #2ecc71;

    }}
    .xo {{
        border:5px solid black;
        width: 150px;
        height: 150px;
        font-size: 100px;
        text-align: center;
        font: 100px \"Libre Caslon Text\"
    }}
    .pg {{
        border:5px solid black;
        width: 150px;
        height: 150px;
        font-size: 100px;
        text-align: center;
        font: 30px \"Libre Caslon Text\"
    }}

    table {{
        position: relative;
        top: 150%;
        left: 0%;
        transform: translate(17%, 50%);
        border:5px solid white;
    }}

    .bottom-text {{
        position: relative;
        width: 100%;
        top: 100%;
        left: -10%;
        transform: translate(0, 400%);
        text-align: center;
        font: 60px \"Libre Caslon Text\"
    }}

    h1 {{
        font-size: 90px;
    }}


</style>
<div
class = \"page\"
>
<br>
<h1 class = \"title\">Tic Tac Toe</h1>
<h2>a step by step guide</h2>
</div>


<div class = \"page\">2</div>
<div class = \"page\">3</div>
        ") {
            eprintln!("Couldn't write to file: {}", e);
        }
        for (i, pg) in book.iter().enumerate() {
            let mut grid_text: Vec<String> = vec![];

            for i in 0..9 {
                grid_text.push(match pg[i] {
                    GridBox::X => String::from("<td class = \"xo\">X</td>"),
                    GridBox::O => String::from("<td class = \"xo\">O</td>"),
                    _ => {
                        let mut sim_game = pg.clone();
                        sim_game[i] = GridBox::O;
                        sim_game = make_move(sim_game);
                        match options.get(&sim_game) {Some(x) => format!("<td class = \"pg\">Pg {}</td>",x+3), None => String::from("<td class = \"pg\"> </td>")}
                    }

                });
            }
            if let Err(e) = writeln!(file, "{}", format!(
"<div class = \"page\">
<table><tr>{}</tr><tr>{}</tr><tr>{}</tr></table> 
<p class = \"bottom-text\">{}</p>
</div>", 
            (grid_text[0].clone() + &grid_text[1] + &grid_text[2]),
            (grid_text[3].clone() + &grid_text[4] + &grid_text[5]),
            (grid_text[6].clone() + &grid_text[7] + &grid_text[8]),
            match is_over(*pg).1 {
                _ if i == 0 => "opponent starts",
                _ if i == 1 => "you start",

                GridBox::X => "x wins",
                GridBox::O => "o wins... somehow?",
                _ if !pg.contains(&GridBox::Blank) => "tie",
                GridBox::Blank => "",
            }, )
        ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }


        let mut game = [GridBox::Blank; 9];
        

    }
