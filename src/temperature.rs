/*

華氏溫度（F）和攝氏溫度（C）之間的轉換公式如下：

1. 將華氏溫度轉換為攝氏溫度的公式：
  C = 5/9 * (F - 32)

2. 將攝氏溫度轉換為華氏溫度的公式：
  F = 9/5 * C + 32

這些公式可以用於在兩種溫度單位之間進行轉換。例如，將華氏溫度 32 度轉換為攝氏溫度，或將攝氏溫度 0 度轉換為華氏溫度。

*/

pub enum TemperatureType {
  // 攝氏
  Celsius,

  // 華氏
  Fahrenheit
}

fn fa_to_cel(value: f64) -> f64 {
  (value - 32.0) * 5.0 / 9.0
}

fn cel_to_fa(value: f64) -> f64 {
  ((9.0 / 5.0) * value) + 32.0
}

pub fn transfer (value: f64, to_type: TemperatureType) -> f64 {
 match to_type {
  TemperatureType::Celsius => fa_to_cel(value),
  TemperatureType::Fahrenheit => cel_to_fa(value),
 }
}

