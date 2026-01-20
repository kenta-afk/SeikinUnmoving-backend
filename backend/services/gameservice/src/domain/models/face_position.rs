use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 顔の座標を表す構造体
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FacePosition {
    /// X座標（画面の左端からの相対位置）
    pub x: f64,
    /// Y座標（画面の上端からの相対位置）
    pub y: f64,
    /// 幅
    pub width: f64,
    /// 高さ
    pub height: f64,
}

impl FacePosition {
    /// 新しいFacePositionを作成
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 顔の中心座標を取得
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// 2つの顔位置間の距離を計算（ユークリッド距離）
    pub fn distance_to(&self, other: &FacePosition) -> f64 {
        let (x1, y1) = self.center();
        let (x2, y2) = other.center();
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    /// 動きがあったかどうかを判定（閾値を超えた場合）
    pub fn has_moved(&self, other: &FacePosition, threshold: f64) -> bool {
        self.distance_to(other) > threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let pos = FacePosition::new(10.0, 20.0, 100.0, 100.0);
        let (x, y) = pos.center();
        assert_eq!(x, 60.0);
        assert_eq!(y, 70.0);
    }

    #[test]
    fn test_distance() {
        let pos1 = FacePosition::new(0.0, 0.0, 100.0, 100.0);
        let pos2 = FacePosition::new(300.0, 400.0, 100.0, 100.0);
        let distance = pos1.distance_to(&pos2);
        // (350-50)^2 + (450-50)^2 = 300^2 + 400^2 = 90000 + 160000 = 250000
        // sqrt(250000) = 500
        assert_eq!(distance, 500.0);
    }

    #[test]
    fn test_has_moved() {
        let pos1 = FacePosition::new(0.0, 0.0, 100.0, 100.0);
        let pos2 = FacePosition::new(10.0, 10.0, 100.0, 100.0);
        let pos3 = FacePosition::new(100.0, 100.0, 100.0, 100.0);

        assert!(!pos1.has_moved(&pos2, 20.0)); // 距離が閾値以下
        assert!(pos1.has_moved(&pos3, 20.0)); // 距離が閾値を超える
    }
}
