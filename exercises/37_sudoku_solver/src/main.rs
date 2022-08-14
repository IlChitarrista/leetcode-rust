fn main() {
    let mut board = vec![vec!['5','3','.','.','7','.','.','.','.'], vec!['6','.','.','1','9','5','.','.','.'], vec!['.','9','8','.','.','.','.','6','.'], vec!['8','.','.','.','6','.','.','.','3'], vec!['4','.','.','8','.','3','.','.','1'], vec!['7','.','.','.','2','.','.','.','6'], vec!['.','6','.','.','.','.','2','8','.'], vec!['.','.','.','4','1','9','.','.','5'], vec!['.','.','.','.','8','.','.','7','9']];
    //println!("Initial: {:?}", board);
    solution(&mut board);
    println!("Resolved: {:?}", board);
}

fn solution(board: &mut Vec<Vec<char>>) {
    use std::collections::HashMap;
    let (mut h_idx, mut v_idx, mut curr_idx) = (0, 0, [0,0]);
    while curr_idx[0] < 9 || curr_idx [1] < 9 {
        let mut seen_stuff: HashMap<char, i32> = HashMap::new();
        while h_idx < 9 {
            let curr = seen_stuff.get(&(board[curr_idx[1]][h_idx]));
            if curr == None {
                if board[curr_idx[1]][h_idx] != '.' {
                    seen_stuff.insert(board[curr_idx[1]][h_idx], 0);
                    println!("{}", board[curr_idx[1]][h_idx]);
                }
            }
            h_idx += 1;
        }
        h_idx = 0;
        while v_idx < 9 {
            let curr = seen_stuff.get(&(board[v_idx][curr_idx[0]]));
            if curr == None {
                if board[v_idx][curr_idx[0]] != '.' {
                    seen_stuff.insert(board[v_idx][curr_idx[0]], 0);
                    println!("{}", board[v_idx][curr_idx[0]]);
                }
            }
            v_idx += 1;
        }


        println!("AIHAIHAIH");
        break;
    }

    unimplemented!()
}

