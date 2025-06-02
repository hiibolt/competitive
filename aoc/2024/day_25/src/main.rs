fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap();

    let locks = data
        .split("\n\n")
        .filter(|obj| {
            obj.lines().next().unwrap().chars().next().unwrap() == '#'
        })
        .map(|st| {
            st.lines().map(|ln| ln.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();
    let keys = data
        .split("\n\n")
        .filter(|obj| {
            obj.lines().next().unwrap().chars().next().unwrap() != '#'
        })
        .map(|st| {
            st.lines().map(|ln| ln.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    let mut lock_heights = Vec::new();
    for lock in locks {
        let mut lock_height_set = Vec::new();
        for g in 0..lock[0].len() {
            for i in 0..lock.len() {
                if lock[i][g] == '.' {
                    lock_height_set.push(i - 1);
                    break;
                }
            }
        }
        lock_heights.push(lock_height_set);
    }
    let mut key_heights = Vec::new();
    for key in keys {
        let mut key_height_set = Vec::new();
        for g in 0..key[0].len() {
            for i in 0..key.len() {
                if key[key.len() - i - 1][g] == '.' {
                    key_height_set.push(i - 1);
                    break;
                }
            }
        }
        key_heights.push(key_height_set);
    }

    println!("Lock Heights: {lock_heights:#?}");
    println!("Keys: {key_heights:#?}");

    let mut total = 0usize;
    for lock_height in lock_heights {
        'key: for key_height in &key_heights {
            println!("{:?} {:?}", lock_height, key_height);
            for i in 0..lock_height.len() {
                if lock_height[i] + key_height[i] > 5 {
                    continue 'key;
                }
            }

            total += 1;
        }
    }

    println!("Total combinations: {total}");
}
