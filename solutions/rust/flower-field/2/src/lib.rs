pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.len() == 0{
        return vec![];
    }
    if garden[0].len() == 0{
        return vec!["".to_string()];
    }
    let grid = garden.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let mut res = grid.clone();
    let cols = grid[0].len();
    for i in 0..grid.len(){
        for j in 0..cols{
            if grid[i][j] == '*' {
                continue;
            }
            let cell = check_flower(grid.clone(), i as isize, j as isize);
            if cell>0{
                res[i][j] = char::from_digit(cell, 10).unwrap();
            }else {
                res[i][j] = ' ';
            }
        }
    }
    res.iter().map(|row| row.iter().collect()).collect()
}

pub fn check_flower(garden:Vec<Vec<char>>, i:isize, j:isize) -> u32{
    let directions = [(-1, -1), (-1,  0), (-1,  1), ( 0, -1), ( 0,  1), ( 1, -1), ( 1,  0), ( 1,  1)];
    let mut count = 0;
    for (dr,dc) in directions{
        let nr = dr+i;
        let nc = dc+j;
        if nr >= 0 && nr < garden.len() as isize && 
           nc >= 0 && nc < garden[0].len() as isize &&
           garden[nr as usize][nc as usize] == '*'{
                    count += 1;
            } 
    }
    count
}
