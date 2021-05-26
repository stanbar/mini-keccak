#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Matrix(pub [[u16; 5]; 5]);

impl Matrix {
    pub fn zeros() -> Self {
        Matrix([[0u16; 5]; 5])
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| {
                    x.iter()
                        .map(|y| format!("{:x}", y))
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
