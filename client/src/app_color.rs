pub enum AppColor {
  Weak,
  StrongDark,
  StrongLight,
  WhiteLight,
  WhiteDark,
}

impl AppColor {
  pub fn as_str(self) -> &'static str {
    match self {
      AppColor::Weak => "#B4B8AB",
      AppColor::StrongDark => "#153243",
      AppColor::StrongLight => "#284b63",
      AppColor::WhiteLight => "#FFA500",
      AppColor::WhiteDark => "#B7C9E2" 
    }
  }

  pub fn to_string(self) -> String {
    String::from(self.as_str())
  }
}