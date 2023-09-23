use utoipa::OpenApi;

use crate::handlers::health_check::__path_health_check_handler;
use crate::handlers::users::create_user::__path_create_user;
use crate::handlers::users::delete_user::__path_delete_user;
use crate::handlers::users::get_all_users::__path_get_all_users;
use crate::handlers::users::get_single_user::__path_get_single_user;
use crate::handlers::users::update_user::__path_update_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check_handler,
        get_all_users,
        get_single_user,
        update_user,
        create_user,
        delete_user,
    ),
    components(schemas(crate::dtos::User, crate::dtos::CreateUser, crate::dtos::UpdateUser,))
)]
pub struct ApiDoc;
