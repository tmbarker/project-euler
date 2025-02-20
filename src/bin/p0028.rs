fn solve() -> String {
    DiagSpiralIter::new(1, 1001).sum::<usize>().to_string()
}

struct DiagSpiralIter {
    n: usize,
    delta: usize,
    rem_corners: usize,
    cur_spiral: usize,
    max_spiral: usize,
}

impl DiagSpiralIter {
    const CORNERS: usize = 4;
    pub fn new(center: usize, width: usize) -> Self {
        assert!(width > 0, "Width must be greater than 0");
        assert_eq!(width % 2, 1, "Spiral width must be odd");
        Self {
            n: center,
            delta: 2,
            rem_corners: 1,
            cur_spiral: 1,
            max_spiral: width / 2 + 1,
        }
    }
}

impl Iterator for DiagSpiralIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem_corners == 0 {
            self.rem_corners = Self::CORNERS;
            self.cur_spiral += 1;
            self.delta += 2;

            if self.cur_spiral > self.max_spiral {
                return None;
            }
        }

        //  NOTE: The center "spiral" is a special case, in that it only has one corner.
        //
        if self.cur_spiral == 1 {
            self.cur_spiral += 1;
            self.rem_corners = Self::CORNERS;
            return Some(self.n);
        }

        self.n += self.delta;
        self.rem_corners -= 1;
        Some(self.n)
    }
}

euler::register_problem!("Number Spiral Diagonals", solve, "669171001");
