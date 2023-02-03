pub enum AppColor {
  Weak,
  StrongDark,
  StrongLight,
  PalePrimary,
  PaleSecondary,
  Error,
  Success,
  SuccessDark,
  Warning
}

impl AppColor {
  pub fn as_str(self) -> &'static str {
    match self {
      AppColor::Weak => "#B4B8AB",
      AppColor::StrongDark => "#153243",
      AppColor::StrongLight => "#284b63",
      AppColor::PalePrimary => "#B7C9E2",
      AppColor::PaleSecondary => "#FFA500",
      AppColor::Error => "#5e0808",
      AppColor::Success => "#5ad941",
      AppColor::SuccessDark => "#237513",
      AppColor::Warning => "#7a660c",
    }
  }

  pub fn to_string(self) -> String {
    String::from(self.as_str())
  }
}