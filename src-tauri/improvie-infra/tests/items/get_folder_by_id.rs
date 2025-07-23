use improvie_domain::persistence::db::DbPool;
use improvie_domain::repository::items::ItemsRepository;
use improvie_infra::{persistence::db::DbPoolImpl, repository::items::ItemsRepositoryImpl};
use sea_orm::ActiveModelTrait;
use uid::Uid;

#[tokio::test]
async fn main() {
    let db = DbPoolImpl::new_test().await;
    let conn = db.connection();

    let repo = ItemsRepositoryImpl::new();

    let folder_uid = Uid::new();

    improvie_row::items::ActiveModel {
        id: sea_orm::Set(folder_uid),
        title: sea_orm::Set(String::from("Test Folder")),
        kind: sea_orm::Set(improvie_logic::constant::items::ItemKind::Folder),
        description: sea_orm::Set(Some(String::from("Test Folder Description"))),
        created_at: sea_orm::Set(chrono::Utc::now()),
    }
    .insert(&conn)
    .await
    .unwrap();

    improvie_row::folders::ActiveModel {
        item_id: sea_orm::Set(folder_uid),
    }
    .insert(&conn)
    .await
    .unwrap();

    let result = repo.get_folder_by_id(conn, folder_uid).await.unwrap();

    assert!(result.is_some());
    let folder = result.unwrap();
    assert_eq!(folder.item.id, folder_uid);
    assert_eq!(folder.item.title, "Test Folder");
    assert_eq!(
        folder.item.description,
        Some(String::from("Test Folder Description"))
    );

    let non_existent_result = repo.get_folder_by_id(conn, Uid::new()).await.unwrap();
    assert!(non_existent_result.is_none());
}
