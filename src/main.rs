use rand::Rng;

fn main() {
    println!("Enter island size, like \"10 10\".");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    trim_newline(&mut line);

    let size = line.split(" ").collect::<Vec<&str>>();

    let size_x: usize = size[0].parse::<usize>().unwrap();
    let size_y: usize = size[1].parse::<usize>().unwrap();

    let map = generate_island_map(size_x, size_y);
    println!("Generated island map with size ({}|{}):", size_x, size_y);
    // TODO: Beautify map conole render
    println!("{:?}", map);

    let islands_col = find_islands(map);
    println!("Found {} islands", islands_col)
}

fn generate_island_map(size_x: usize, size_y: usize) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let out: Vec<Vec<bool>> = (0..size_x).map(|_| (0..size_y).map(|_| rng.gen()).collect()).collect();
    return out;
}

fn find_islands(map: Vec<Vec<bool>>) -> usize {
    let size_x = map.len();
    let size_y = map[0].len();

    let mut out: usize = 0;
    let mut tested: Vec<(usize, usize)> = vec![];
    let mut testing_is: Vec<(usize, usize)> = vec![];

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if tested.contains(&(x, y)) { continue }

            if map[x][y] {
                out += 1;
                testing_is.push((x, y));

                loop {
                    if testing_is.len() == 0 { break; }

                    let x: usize = testing_is[0].0;
                    let y: usize = testing_is[0].1;

                    if x != 0 {
                        if !tested.contains(&(x - 1, y)) && map[x - 1][y] {
                            testing_is.push((x - 1, y));
                        }
                    }

                    if y != 0 {
                        if !tested.contains(&(x, y - 1)) && map[x][y - 1] {
                            testing_is.push((x, y - 1));
                        }
                    }

                    if x + 1 < size_x {
                        if !tested.contains(&(x + 1, y)) && map[x + 1][y] {
                            testing_is.push((x + 1, y));
                        }
                    }

                    if y + 1 < size_y {
                        if !tested.contains(&(x, y + 1)) && map[x][y + 1] {
                            testing_is.push((x, y + 1));
                        }
                    }

                    testing_is.remove(0);
                    tested.push((x, y));
                }
            }
        }
    }

    return out
}

fn trim_newline(line: &mut String) {
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
}
