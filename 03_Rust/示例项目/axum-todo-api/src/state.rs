use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::Todo;

/// 全局共享状态：内存中的 todo 存储。
/// 生产环境可替换为 sqlx 连接池等。
pub type AppState = Arc<RwLock<HashMap<Uuid, Todo>>>;
