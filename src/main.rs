const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
struct Board {
    state: [[usize; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            state: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn is_inside_board(&self, x: usize, y: usize) -> bool {
        if x > BOARD_SIZE - 1 || y > BOARD_SIZE - 1 {
            return false;
        }
        /* else if x < 0 || y < 0 {
            return false;
        } */
        true
    }

    fn is_free_space(&self, x: usize, y: usize) -> Result<bool, &'static str> {
        if !self.is_inside_board(x, y) {
            return Err("Out of range.");
        }

        if self.state[y][x] == 0 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn put(&mut self, x: usize, y: usize, p: usize) -> Result<(), &'static str> {
        if !self.is_inside_board(x, y) {
            return Err("Out of range.");
        }

        if !(p == 1 || p == 2) {
            return Err("p is not player value.");
        }

        if self.is_free_space(x, y)? {
            self.state[y][x] = p;
            self.flip(x, y, p);
        } else {
            return Err("Put point is not None.");
        }
        Ok(())
    }

    fn search(
        &self,
        target_x: usize,
        target_y: usize,
        direction_x: isize,
        direction_y: isize,
        p: usize,
        len: usize,
    ) -> usize {
        // 盤面の範囲外に出る際に検出してReturn
        if target_x == BOARD_SIZE - 1
            || target_y == BOARD_SIZE - 1
            || target_x == 0
            || target_y == 0
        {
            return 0;
        }

        if self.state[(target_y as isize + direction_y) as usize]
            [(target_x as isize + direction_x) as usize]
            == Self::reversi_player_num(p)
        {
            // 指定した色とは反対の色を探す.これで挟まれている色を探索する.
            return self.search(
                (target_x as isize + direction_x) as usize,
                (target_y as isize + direction_y) as usize,
                direction_x,
                direction_y,
                p,
                len + 1,
            );
        } else if len > 0
            && self.state[(target_y as isize + direction_y) as usize]
                [(target_x as isize + direction_x) as usize]
                == p
        {
            // len > 0 挟まれている色が検索によって存在するかつその先に指定された色があれば挟まれたと判定する
            return len;
        } else {
            // 何もなかった場合はReturn
            return 0;
        }
    }

    fn flip(&mut self, x: usize, y: usize, p: usize) {
        for dir in [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
        ] {
            let len = self.search(x, y, dir[0], dir[1], p, 0);
            let mut pos_x = x as isize;
            let mut pos_y = y as isize;
            for _ in 0..len {
                pos_x += dir[0];
                pos_y += dir[1];
                self.state[pos_y as usize][pos_x as usize] = p;
            }
        }
    }

    fn reversi_player_num(p: usize) -> usize {
        match p {
            1 => 2,
            2 => 1,
            _ => 0,
        }
    }

    fn put_first_board(&mut self) {
        self.put(3, 3, 1).unwrap();
        self.put(4, 3, 2).unwrap();
        self.put(3, 4, 2).unwrap();
        self.put(4, 4, 1).unwrap();
    }

    fn print(&self) {
        for i in self.state.into_iter() {
            for j in i.into_iter() {
                match j {
                    0 => print!(" -"),
                    1 => print!(" ○"),
                    2 => print!(" ●"),
                    _ => panic!(),
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.put_first_board();
    board.print();

    let mut p = 2;
    loop {
        // Game Loop

        let mut x = String::new();
        std::io::stdin().read_line(&mut x).unwrap();
        let mut y = String::new();
        std::io::stdin().read_line(&mut y).unwrap();

        let x = x.trim_end().to_owned().parse().unwrap_or(0);
        let y = y.trim_end().to_owned().parse().unwrap_or(0);

        println!("({}, {})", x, y);

        if x > BOARD_SIZE - 1 || y > BOARD_SIZE - 1 {
            println!(
                "Out of range. Please select inside of BOARD_SIZE: {}",
                BOARD_SIZE
            );
            continue;
        }

        if !board.is_free_space(x, y).unwrap() {
            println!("This is not free space. Please select free space.");
            continue;
        }

        board.put(x, y, p).unwrap();
        board.print();

        p = Board::reversi_player_num(p);
    }
}
