//https://leetcode.com/problems/rotate-image/

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for r in matrix {
        println!("{r:?}");
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let len: usize = matrix.len();
    let mut ans = [[0; 3]; 3];
    for i in 0..len {
        for j in 0..len {
            ans[j][i] = matrix[i][j]
        }
    }
    ans
}

fn reflect(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut ans =  [[0; 3]; 3];
    matrix.clone_into(&mut ans);
    let len :usize = matrix.len();
    // move columns like a mirror
    for i in 0..len {
        for j in 0..len/2 {
            let tmp = ans[i][j];
            ans[i][j] = ans[i][len-j-1];
            ans[i][len-j-1] = tmp;
        }
    }

    ans
}
fn rotate(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    reflect(transpose(matrix))
}


#[test]
fn test_transpose() {
    let mat = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let desired = [[101, 201, 301], [102, 202, 302], [103, 203, 303]];
    let trans = transpose(mat);
    assert_eq!(trans, desired)
}
#[test]
fn test_transpose2() {
    let mat = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9]
    ];
    let desired = [
        [1, 4, 7], //
        [2, 5, 8],
        [3, 6, 9]
    ];
    let trans = transpose(mat);
    assert_eq!(trans, desired)
}

#[test]
fn test_reflect() {
    let mat = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let desired = [[103, 102, 101], [203, 202, 201], [303, 302, 301]];
    let trans = reflect(mat);
    assert_eq!(trans, desired)
}

#[test]
fn test_rotate() {
    let mat = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let desired = [[301, 201, 101], [302, 202, 102], [303, 203, 103]];
    let trans = rotate(mat);
    assert_eq!(trans, desired)
}

#[test]
fn test_rotate2() {
    let mat = [
        [1,2,3], // \n
        [4,5,6],
        [7,8,9]
    ];
    let desired = [
        [7,4,1], // \n
        [8,5,2],
        [9,6,3]
    ];
    let trans = rotate(mat);
    assert_eq!(trans, desired)
}

fn main() {
    let matrix = [
        [101, 102, 103], // \n
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);

    let reflected = reflect(matrix);
    println!("reflect:");
    pretty_print(&reflected);

    let rotated = rotate(matrix);
    println!("rotated:");
    pretty_print(&rotated);

}
