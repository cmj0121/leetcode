pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let x = mat.len() as i32;
    let y = mat.first().unwrap().len() as i32;
    let mut ret: Vec<Vec<i32>> = vec![];

    if x * y != r * c {
        return mat;
    }

    for i in 0..r {
        let mut tmp: Vec<i32> = vec![];
        for j in 0..c {
            let pos = (i * c + j) as usize;
            tmp.push(mat[pos / y as usize][pos % y as usize]);
        }

        ret.push(tmp);
    }

    ret
}
fn main() {}
