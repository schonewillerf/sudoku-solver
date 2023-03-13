fn possible(y: usize, x: usize, n: i32, grid: &[[i32; 9]; 9]) -> bool {
    for i in 0..9 {
        if grid[y][i] == n || grid[i][x] == n { 
            return false;
        }
    }
    let x0 = (x / 3)*3;
    let y0 = (y / 3)*3;
    for i in 0..3 {
        for j in 0..3 {
            if grid[y0 + i][x0 + j] == n {
                return false;
            }
        }
    }
    return true;
}

fn solve(grid: &mut[[i32; 9]; 9]) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if grid[y][x] == 0 {
                for n in 1..10 {
                    if possible(y, x, n, grid) {
                        grid[y][x] = n;
                        if solve(grid) {
                            return true;
                        }
                        grid[y][x] = 0;
                    }
                }
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let mut grid = [
        [0,0,0,1,0,2,0,0,0],
        [0,6,0,0,0,0,0,7,0],
        [0,0,8,0,0,0,9,0,0],
        [4,0,0,0,0,0,0,0,3],
        [0,5,0,0,0,7,0,0,0],
        [2,0,0,0,8,0,0,0,1],
        [0,0,9,0,0,0,8,0,5],
        [0,7,0,0,0,0,0,6,0],
        [0,0,0,3,0,4,0,0,0],
    ];
    solve(&mut grid);
    println!("the result {:?}", grid);
}
