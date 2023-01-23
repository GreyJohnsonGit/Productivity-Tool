pub enum Color {
  Weak,
  StrongDark,
  StrongLight,
  WhiteLight,
  WhiteDark,
}

impl Color {
  pub fn as_str(self) -> &'static str {
    match self {
      Color::Weak => "#B4B8AB",
      Color::StrongDark => "#153243",
      Color::StrongLight => "#284b63",
      Color::WhiteLight => "#f4f9e9",
      Color::WhiteDark => "#eef0eb" 
    }
  }
}