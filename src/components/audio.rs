use bevy::prelude::*;

/// 游戏音效资源集合
/// 包含游戏中所有使用的音效，便于统一管理和访问
/// 实现了 Asset + FromWorld trait，支持通过 assets_tracking 模块异步加载
#[derive(Resource, Asset, Clone, Reflect)]
pub struct GameSounds {
    /// 球碰撞音效（挡板、墙壁、砖块碰撞时播放）
    pub collision: Handle<AudioSource>,
    /// 游戏开始音效
    pub game_start: Handle<AudioSource>,
    /// 游戏结束音效
    pub game_over: Handle<AudioSource>,
    /// 得分音效（摧毁砖块时播放）
    pub score: Handle<AudioSource>,
    /// 胜利音效（清除所有砖块时播放）
    pub victory: Handle<AudioSource>,
}

impl GameSounds {
    /// 创建一个新的 GameSounds 实例，加载所有音效资源
    ///
    /// # 参数
    /// - `asset_server`: 资源服务器，用于加载音效文件
    ///
    /// # 返回值
    /// 返回包含所有音效句柄的 GameSounds 实例
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            collision: asset_server.load("sounds/breakout_collision.ogg"),
            game_start: asset_server.load("sounds/breakout_collision.ogg"),
            game_over: asset_server.load("sounds/breakout_collision.ogg"),
            score: asset_server.load("sounds/breakout_collision.ogg"),
            victory: asset_server.load("sounds/breakout_collision.ogg"),
        }
    }
}

impl FromWorld for GameSounds {
    /// 从 World 中创建 GameSounds 实例
    /// 这是 assets_tracking 模块要求的接口，用于异步加载资源
    ///
    /// # 参数
    /// - `world`: Bevy 世界，包含所有资源和实体
    ///
    /// # 返回值
    /// 返回包含所有音效句柄的 GameSounds 实例
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self::new(asset_server)
    }
}