impl Solution {
    fn can_be_planted_in(index: usize, flowerbed: &[i32]) -> bool {
        flowerbed[index - 1] == 0 && flowerbed[index] == 0 && flowerbed[index + 1] == 0
    }

    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let (mut flowerbed, mut n) = (flowerbed, n);

        flowerbed.insert(0, 0);
        flowerbed.push(0);

        for index in 1..flowerbed.len() - 1 {
            if n == 0 {
                break;
            }

            if Self::can_be_planted_in(index, &flowerbed) {
                flowerbed[index] = 1;
                n -= 1;
            }
        }

        n == 0
    }
}