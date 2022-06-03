use std::borrow::BorrowMut;

// m == image.length
// n == image[i].length
// 1 <= m, n <= 50
// 0 <= image[i][j], newColor < 216
// 0 <= sr < m
// 0 <= sc < n
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let origin_color = image[sr as usize][sc as usize];
    if origin_color == new_color {
        return image;
    }
    let mut image_mute = image.clone();
    change_fill_color(image_mute.borrow_mut(), sr, sc, origin_color, new_color);
    image_mute
}

fn change_fill_color(image_ref: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, new_color: i32) {
    let m = image_ref.len();
    let n = image_ref[0].len();
    if sr < 0 || sr > (m as i32) - 1 {
        return;
    }
    if sc < 0 || sc > (n as i32) - 1 {
        return;
    }
    if image_ref[sr as usize][sc as usize] != color {
        return;
    }
    image_ref[sr as usize][sc as usize] = new_color;
    change_fill_color(image_ref, sr - 1, sc, color, new_color);
    change_fill_color(image_ref, sr + 1, sc, color, new_color);
    change_fill_color(image_ref, sr, sc - 1, color, new_color);
    change_fill_color(image_ref, sr, sc + 1, color, new_color);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flood_fill() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        )
    }

    #[test]
    fn test_stackover_flow() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
            vec![vec![0, 0, 0], vec![0, 1, 1]]
        )
    }
}
