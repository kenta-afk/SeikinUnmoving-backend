use worker::*;
use videoservice::{VideoService, VideoServiceImpl};
use crate::VideoRepositoryD1;

/// D1データベースからビデオサービスを構築
fn build_service_d1(ctx: &RouteContext<()>) -> Result<VideoServiceImpl<VideoRepositoryD1, videoservice::infrastructure::adapters::uuid_service::UuidServiceImpl>> {
    let db = ctx.env.d1("DB")?;
    let video_repo = VideoRepositoryD1::new(db);
    let uuid_service = videoservice::infrastructure::adapters::uuid_service::UuidServiceImpl;
    
    Ok(VideoServiceImpl::new(video_repo, uuid_service))
}

/// 動画一覧取得
async fn get_videos(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // D1データベースから動画サービスを構築
    let video_service = match build_service_d1(&ctx) {
        Ok(service) => service,
        Err(e) => {
            console_log!("Failed to create video service: {}", e);
            return Response::error(format!("Failed to create video service: {}", e), 500);
        }
    };

    // 動画一覧を取得
    let dto = match video_service.get_videos(videoservice::GetVideosCommand {}).await {
        Ok(dto) => dto,
        Err(e) => {
            console_log!("Failed to get videos: {}", e);
            return Response::error(format!("Failed to get videos: {}", e), 500);
        }
    };

    Response::from_json(&dto)
}

/// ランダム動画取得
async fn get_random_video(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // D1データベースから動画サービスを構築
    let video_service = match build_service_d1(&ctx) {
        Ok(service) => service,
        Err(e) => {
            console_log!("Failed to create video service: {}", e);
            return Response::error(format!("Failed to create video service: {}", e), 500);
        }
    };

    // ランダム動画を取得
    let dto = match video_service.get_random_video(videoservice::GetRandomVideoCommand {}).await {
        Ok(dto) => dto,
        Err(e) => {
            console_log!("Failed to get random video: {}", e);
            return Response::error(format!("Failed to get random video: {}", e), 500);
        }
    };

    Response::from_json(&dto)
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router
        .get_async("/api/videos", get_videos)
        .get_async("/api/videos/random", get_random_video)
}
