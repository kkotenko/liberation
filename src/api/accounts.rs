use actix_web::{HttpResponse, web};
use log::debug;
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::Claims;
use crate::authentication::scopes::{ARISTOCRAT_ACCOUNTS_MODIFY, ARISTOCRAT_ACCOUNTS_READ};
use crate::models::NewAccount;

pub async fn get_all(app: web::Data<AppState>, authentication: Claims) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::list_accounts(&conn)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn get_one(app: web::Data<AppState>, authentication: Claims, search_id: web::Path<i32>) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn put(app: web::Data<AppState>, authentication: Claims, write_to_id: web::Path<i32>, new_info: web::Json<NewAccount>) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_ACCOUNTS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated_account = actions::update_account(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated_account))
}

pub async fn delete(app: web::Data<AppState>, authentication: Claims, delete_id: web::Path<i32>) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_ACCOUNTS_MODIFY)?;
    let conn = app.open_database_connection()?;

    let account = actions::find_account(&conn, *delete_id)?;
    actions::deactivate_account(&conn, &account)?;
    debug!("deleting account {:?}", &account);
    // TODO: check all books are returned
    // TODO: delete all librarian roles
    actions::delete_all_books_owned_by_account(&conn, &account)?;
    actions::delete_account(&conn, &account)?;
    Ok(HttpResponse::Ok().finish())
}
