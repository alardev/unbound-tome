use dioxus::prelude::*;
use crate::webapp::model::auth_model::AuthModel;
use crate::webapp::service::AppService;

pub struct AppState {
    // pub api: ApiHandler,
    pub service: AppService,
    pub auth: GlobalSignal<AuthModel>,
    // pub modal: GlobalSignal<ModalModel>,
    // pub users: GlobalSignal<BTreeMap<String, UserDetailsModel>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            // api: ApiHandler::default(),
            service: AppService,
            auth: Signal::global(AuthModel::default),
            // modal: Signal::global(|| ModalModel::None),
            // users: Signal::global(BTreeMap::<String, UserDetailsModel>::new),
        }
    }
}