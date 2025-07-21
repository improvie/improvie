use improvie_domain::persistence::db::DbPool;
use improvie_domain::repository::items::ItemsRepository;
use improvie_infra::{persistence::db::DbPoolImpl, repository::items::ItemsRepositoryImpl};
use improvie_logic::constant::items::ContentKind;
use sea_orm::ActiveModelTrait;
use uid::Uid;

#[tokio::test]
async fn main() {
    let db = DbPoolImpl::new_test().await;
    let conn = db.connection();

    let repo = ItemsRepositoryImpl::new(db.clone());

    let content_uid = Uid::new();

    improvie_row::items::ActiveModel {
        id: sea_orm::Set(content_uid),
        title: sea_orm::Set(String::from("Test Content")),
        kind: sea_orm::Set(improvie_logic::constant::items::ItemKind::Content),
        description: sea_orm::Set(Some(String::from("Test Description"))),
        created_at: sea_orm::Set(chrono::Utc::now()),
    }
    .insert(&conn)
    .await
    .unwrap();

    improvie_row::contents::ActiveModel {
        item_id: sea_orm::Set(content_uid),
        kind: sea_orm::Set(ContentKind::Video),
        content_path: sea_orm::Set(String::from("/test/path/video.mp4")),
        thumbnail_path: sea_orm::Set(Some(String::from("/test/path/thumbnail.jpg"))),
        seconds: sea_orm::Set(120),
    }
    .insert(&conn)
    .await
    .unwrap();

    let result = repo.get_content_by_id(content_uid).await.unwrap();

    assert!(result.is_some());
    let content = result.unwrap();
    assert_eq!(content.item.id, content_uid);
    assert_eq!(content.item.title, "Test Content");
    assert_eq!(
        content.item.description,
        Some(String::from("Test Description"))
    );
    assert_eq!(content.kind, ContentKind::Video);
    assert_eq!(content.content_path, "/test/path/video.mp4");
    assert_eq!(
        content.thumbnail_path,
        Some(String::from("/test/path/thumbnail.jpg"))
    );

    let non_existent_result = repo.get_content_by_id(Uid::new()).await.unwrap();
    assert!(non_existent_result.is_none());
}
