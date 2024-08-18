use std::cmp::max;

pub fn longest_subsequence_len(left: &str, right: &str) -> u64 {
    let matrix = subsequence_matrix(left, right);

    *matrix.last().unwrap().last().unwrap()
}

pub fn longest_subsequence_make(left: &str, right: &str) -> String {
    let left: &[u8] = left.as_ref();
    let right: &[u8] = right.as_ref();

    let matrix = subsequence_matrix(left, right);

    let mut current_value = *matrix.last().unwrap().last().unwrap();
    let output_size = current_value as usize;

    let mut output: Vec<u8> = vec![u8::default(); output_size];
    let mut c = Some(output.len() - 1);

    let mut i = matrix.len() - 1;
    let mut j = matrix[0].len() - 1;

    while i > 0 && j > 0 {
        if matrix[i][j - 1] == current_value {
            j -= 1;
        } else if matrix[i - 1][j] == current_value {
            i -= 1;
        } else {
            *output.get_mut(c.unwrap()).unwrap() = *left.get(i - 1).unwrap();
            c = c.and_then(|x| x.checked_sub(1));
            i -= 1;
            j -= 1;
            current_value = matrix[i][j];
        }
    }
    std::str::from_utf8(output.as_ref()).unwrap().to_owned()
}

fn subsequence_matrix<T: AsRef<[u8]>, U: AsRef<[u8]>>(char_row: T, char_col: U) -> Vec<Vec<u64>> {
    let char_row: &[u8] = char_row.as_ref();
    let char_col: &[u8] = char_col.as_ref();

    let row_len = char_row.len() + 1;
    let col_len = char_col.len() + 1;

    let mut matrix: Vec<Vec<u64>> = vec![vec![0u64; col_len]; row_len];

    for i in 1..row_len {
        for j in 1..col_len {
            let row_char = char_row.get(i - 1);
            let col_char = char_col.get(j - 1);

            let new_value = {
                let choose_up_left_rather_than_max_left_above =
                    row_char == col_char && row_char.is_some();
                match choose_up_left_rather_than_max_left_above {
                    true => matrix[i - 1][j - 1] + 1,
                    false => max(matrix[i][j - 1], matrix[i - 1][j]),
                }
            };

            let changeable_value = matrix.get_mut(i).unwrap().get_mut(j).unwrap();
            *changeable_value = new_value;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_u8() {
        let result = longest_subsequence_len("0124", "01234");
        assert_eq!(result, 4u64);
    }
    #[test]
    fn strs() {
        let result = longest_subsequence_len("test", "tempt");
        assert_eq!(result, 3u64);
    }

    #[test]
    fn strs_common() {
        let result = longest_subsequence_make("test", "tempt");
        assert_eq!(result, "tet");
    }

    #[test]
    fn more_strs() {
        let a = "test";
        let b = "tempt";
        let r1 = longest_subsequence_len(a, b);
        let r2 = longest_subsequence_make(a, b);
        assert_eq!(r1, r2.len() as u64);
    }
}
