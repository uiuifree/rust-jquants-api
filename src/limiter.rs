//! クライアント側レート制限。
//! J-Quants API の契約プランごとの API コール制限（1 分あたり件数）に合わせて
//! リクエストの送信間隔を自動調整する。

use std::collections::VecDeque;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// J-Quants の契約プラン。
/// [`JQuantsClient::with_plan`](crate::JQuantsClient::with_plan) に渡すと
/// プランの API コール制限に合わせたレート制限が有効になる。
///
/// 各プランの制限値は 2026-08 時点の公式料金ページに基づく。
/// 変更される可能性があるため、正確な値は
/// <https://jpx-jquants.com/> の料金プラン比較を確認すること
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plan {
    /// Free プラン（5 件/分）
    Free,
    /// Light プラン（60 件/分）
    Light,
    /// Standard プラン（120 件/分）
    Standard,
    /// Premium プラン（500 件/分）
    Premium,
}

impl Plan {
    /// このプランの 1 分あたりの最大 API コール数
    pub fn requests_per_minute(self) -> u32 {
        match self {
            Plan::Free => 5,
            Plan::Light => 60,
            Plan::Standard => 120,
            Plan::Premium => 500,
        }
    }
}

/// 直近 1 分間のリクエスト回数を数え、上限に達していたら空きが出るまで待つ
/// （スライディングウィンドウ方式）
#[derive(Debug)]
pub(crate) struct RateLimiter {
    max_per_window: u32,
    window: Duration,
    history: Mutex<VecDeque<Instant>>,
}

impl RateLimiter {
    pub(crate) fn per_minute(max: u32) -> Self {
        Self {
            max_per_window: max,
            window: Duration::from_secs(60),
            history: Mutex::new(VecDeque::new()),
        }
    }

    /// リクエスト 1 回分の枠を確保する。上限に達している間は待機する
    pub(crate) async fn acquire(&self) {
        loop {
            let wait = {
                let mut history = self.history.lock().await;
                let now = Instant::now();
                while let Some(&oldest) = history.front() {
                    if now.duration_since(oldest) >= self.window {
                        history.pop_front();
                    } else {
                        break;
                    }
                }
                if (history.len() as u32) < self.max_per_window {
                    history.push_back(now);
                    None
                } else {
                    // 一番古いリクエストがウィンドウから外れるまで待つ
                    let oldest = *history.front().expect("history is non-empty here");
                    Some((oldest + self.window).saturating_duration_since(now))
                }
            };
            match wait {
                None => return,
                Some(duration) => tokio::time::sleep(duration).await,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_rates_match_official_pricing() {
        assert_eq!(Plan::Free.requests_per_minute(), 5);
        assert_eq!(Plan::Light.requests_per_minute(), 60);
        assert_eq!(Plan::Standard.requests_per_minute(), 120);
        assert_eq!(Plan::Premium.requests_per_minute(), 500);
    }

    #[tokio::test(start_paused = true)]
    async fn acquire_is_instant_under_the_limit() {
        let limiter = RateLimiter::per_minute(2);
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_secs(1));
    }

    #[tokio::test(start_paused = true)]
    async fn acquire_waits_when_window_is_full() {
        let limiter = RateLimiter::per_minute(2);
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        // 3 件目はウィンドウが空くまで（約 60 秒）待たされる
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_secs(60));
    }

    #[tokio::test(start_paused = true)]
    async fn window_slides_after_time_passes() {
        let limiter = RateLimiter::per_minute(2);
        limiter.acquire().await;
        limiter.acquire().await;
        tokio::time::sleep(Duration::from_secs(61)).await;
        // 1 分経過後は待ちなしで取得できる
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_secs(1));
    }
}
