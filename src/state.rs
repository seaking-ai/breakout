pub use bevy::prelude::*;

/// 游戏状态枚举
/// 定义游戏的不同状态，用于状态管理
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// 菜单状态 - 显示开始菜单
    #[default]
    Menu,
    /// 关卡选择状态 - 显示关卡选择界面
    LevelSelect,
    /// 游戏进行中状态
    Playing,
    /// 游戏结束状态（失败）
    GameOver,
    /// 游戏胜利状态（所有砖块被消灭）
    Victory,
}

/// 当前关卡资源
/// 存储玩家选择的关卡编号
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    /// 第一关 - 简单难度，只有3行砖块
    #[default]
    Level1,
    /// 第二关 - 完整难度，填满砖块
    Level2,
}

/// 游戏进行中状态枚举
/// 定义游戏进行中的不同状态，用于状态管理
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayingState {
    /// 初始小球附着在挡板上
    #[default]
    ball_attached,
    /// 发射后游戏进行中状态
    ball_launched,
}