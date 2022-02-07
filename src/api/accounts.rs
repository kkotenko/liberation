use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::auth::roles::{ACCOUNTS_DELETE, ACCOUNTS_EDIT, ACCOUNTS_READ};
use crate::error::UserFacingError;
use crate::models::Account;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::list_accounts(&conn)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn get_one(app: web::Data<AppState>, authentication: Authentication, account_id: web::Path<i32>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account(&conn, account_id.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn patch(app: web::Data<AppState>, authentication: Authentication, account: web::Json<Account>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated_account = actions::update_account(&conn, account.into_inner())?;
    Ok(HttpResponse::Ok().json(updated_account))
}

pub async fn delete(app: web::Data<AppState>, authentication: Authentication, account_id: web::Path<i32>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_DELETE)?;
    let conn = app.open_database_connection()?;
    actions::deactivate_account(&conn, account_id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}
