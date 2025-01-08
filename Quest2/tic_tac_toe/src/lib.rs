pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }
    
    for row in table.iter() {
        for &cell in row.iter() {
            if cell == '#' {
                return "tie".to_string();
            }
        }
    }
    
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    } else if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[0][1] == player && table[0][2] == player {
        return true;
    } else if table[1][0] == player && table[1][1] == player && table[1][2] == player {
        return true;
    } else if table[2][0] == player && table[2][1] == player && table[2][2] == player {
        return true;
    }

    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][0] == player && table[2][0] == player {
        return true;
    } 
    else if table[0][1] == player && table[1][1] == player && table[2][1] == player {
        return true;
    } 
    else if table[0][2] == player && table[1][2] == player && table[2][2] == player {
        return true;
    }

    false
}
