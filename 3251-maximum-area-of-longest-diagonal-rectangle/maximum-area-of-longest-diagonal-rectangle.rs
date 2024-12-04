impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let (mut max_area, mut max_diagonal) = (0.0, 0.0);

        for rectangle in dimensions {
            let (width, height) = (rectangle[0] as f64, rectangle[1] as f64);

            let cur_area = width * height;
            let cur_diagonal = (width.powi(2) + height.powi(2)).sqrt();

            if cur_diagonal > max_diagonal {
                max_area = cur_area;
            } else if cur_diagonal == max_diagonal {
                max_area = max_area.max(cur_area);
            }
            max_diagonal = max_diagonal.max(cur_diagonal);
        }

        max_area as i32
    }
}