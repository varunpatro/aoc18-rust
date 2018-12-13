const GRID_SERIAL_NUMBER: usize = 5177;
const GRID_SIZE: usize = 300;

fn power_level(x: usize, y: usize) -> i32 {
    if x == 0 || y == 0 {
        return 0;
    }

    let mut p = x;
    p += 10;
    p *= y;
    p += GRID_SERIAL_NUMBER;
    p *= x + 10;
    p /= 100;
    p %= 10;
    p as i32 - 5
}

fn get_prefix(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut prefix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..v.len() {
        let mut row = Vec::new();
        for _ in 0..v.len() {
            row.push(0);
        }
        prefix.push(row);
    }

    for i in 1..v.len() {
        for j in 1..v.len() {
            let a = prefix[i - 1][j - 1];
            let b = prefix[i - 1][j];
            let c = prefix[i][j - 1];
            prefix[i][j]= b + c - a + v[i][j];
        }
    }

    prefix
}

fn get_max(v: &Vec<Vec<i32>>, p: &Vec<Vec<i32>>, s: usize) -> (i32, usize, usize) {
    let mut max = 0;
    let (mut ix, mut iy) = (0, 0);

    // println!("{:?}", v);
    // println!("{:?}", p);

    for x in 1..v.len() - s + 1 {
        for y in 1..v.len() - s + 1 {

            let a = p[x - 1][y - 1];
            let b = p[x - 1][y + s - 1];
            let c = p[x + s - 1][y - 1];
            let d = p[x + s - 1][y + s - 1];
            let sum = d + a - b - c;

            if sum > max {
                max = sum;
                ix = x;
                iy = y;
            }
        }
    }

    (max, ix, iy)
}

fn main() {
    let mut xs = Vec::new();

    for x in 0..GRID_SIZE+1 {
        let mut v = Vec::new();
        for y in 0..GRID_SIZE+1 {
            v.push(power_level(x, y));
        }
        xs.push(v);
    }

    let prefix = get_prefix(&xs);
    let mut mx = 0;
    let mut ix = 0;
    let mut iy = 0;
    let mut size = 0;
    for s in 1..xs.len() {
        let (m, x, y) = get_max(&xs, &prefix, s);
        if m > mx {
            mx = m;
            ix = x;
            iy = y;
            size = s;
        }
    }

    println!("max: {} @ {}, {} for {}x{}", mx, ix, iy, size, size);
}