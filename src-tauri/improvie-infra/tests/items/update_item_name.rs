use improvie_domain::persistence::db::DbPool;
use improvie_domain::repository::items::ItemsRepository;
use improvie_infra::{persistence::db::DbPoolImpl, repository::items::ItemsRepositoryImpl};
use sea_orm::{ActiveModelTrait, EntityTrait};
use uid::Uid;

#[tokio::test]
async fn main() {
    let db = DbPoolImpl::new_test().await;
    let conn = db.connection();

    let repo = ItemsRepositoryImpl::new();

    let uid = Uid::new();

    improvie_row::items::ActiveModel {
        id: sea_orm::Set(uid),
        title: sea_orm::Set(String::from("Test Item")),
        kind: sea_orm::Set(improvie_logic::constant::items::ItemKind::Folder),
        description: sea_orm::Set(None),
        created_at: sea_orm::Set(chrono::Utc::now()),
    }
    .insert(&conn)
    .await
    .unwrap();

    repo.update_item_name(conn, uid, String::from("Updated Item Name"))
        .await
        .unwrap();

    let updated_model_opt = improvie_row::items::Entity::find_by_id(uid)
        .one(&conn)
        .await
        .unwrap();
    let update_model = updated_model_opt.unwrap();

    assert_eq!(update_model.title, "Updated Item Name");
}
