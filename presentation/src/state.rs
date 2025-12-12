use axum::extract::FromRef;

/// Main application state that holds all services
#[derive(Clone)]
pub struct AppState<US>
where
    US: Clone,
{
    pub user_service: US,
}

#[derive(Clone)]
pub struct UserServiceState<T>(pub T);

impl<US> FromRef<AppState<US>> for UserServiceState<US>
where
    US: Clone,
{
    fn from_ref(state: &AppState<US>) -> Self {
        Self(state.user_service.clone())
    }
}
