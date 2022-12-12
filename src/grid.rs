pub struct Grid<'a> {
    input: &'a [u8],
    width: usize,
}

impl<'a> Grid<'a> {
    pub fn parse(input: &'a str) -> Result<Self, &'static str> {
        let width = input
            .lines()
            .next()
            .ok_or("Expected non-empty input")?
            .len();
        Ok(Self {
            input: input.as_bytes(),
            width,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.input.len() / (self.width + 1)
    }

    pub fn at(&self, x: usize, y: usize) -> Option<u8> {
        self.input
            .get(y.checked_mul(self.width + 1)?..)?
            .get(..self.width)?
            .get(x)
            .copied()
    }

    pub fn find(&self, c: u8) -> Option<(usize, usize)> {
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.at(x, y).unwrap() == c {
                    return Some((x, y));
                }
            }
        }
        None
    }
}
