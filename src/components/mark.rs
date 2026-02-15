use bevy::prelude::*;

/// 记分板UI组件标记
#[derive(Component)]
pub struct ScoreboardUi;

/// 挡板组件标记
#[derive(Component)]
pub struct Paddle;

/// 球组件标记
#[derive(Component)]
pub struct Ball;

/// 游戏提示UI组件标记
#[derive(Component)]
pub struct GameHintUi;

/// 小球是否附着在挡板上
#[derive(Component)]
pub struct BallAttached;

/// 砖块组件标记
#[derive(Component)]
pub struct Brick;

/// 生命数显示UI组件标记
/// 用于在屏幕右上角显示剩余小球数量
#[derive(Component)]
pub struct LivesUi;

/// 单个生命指示器组件标记
/// 用于标识代表一次生命的小球图标
#[derive(Component)]
pub struct LifeIndicator;

/// 游戏结束UI根节点标记
/// 用于标识GameOver界面的根实体，便于清理
#[derive(Component)]
pub struct GameOverRoot;

/// 返回菜单按钮组件标记
#[derive(Component)]
pub struct BackToMenuButton;

/// 游戏实体根节点标记
/// 用于标识游戏场景中的实体（挡板、小球、墙壁、砖块等），便于在退出游戏时清理
#[derive(Component)]
pub struct GamePlayRoot;

/// 游戏胜利UI根节点标记
/// 用于标识Victory界面的根实体，便于清理
#[derive(Component)]
pub struct VictoryRoot;

/// 小球动画组件
/// 用于控制精灵图集的帧动画
#[derive(Component)]
pub struct BallAnimation {
    /// 当前帧索引 (0-3)
    pub current_frame: usize,
    /// 总帧数
    pub total_frames: usize,
    /// 每帧持续时间（秒）
    pub frame_duration: f32,
    /// 当前帧已过去的时间
    pub timer: f32,
}

impl BallAnimation {
    /// 创建新的小球动画组件
    ///
    /// # 参数
    /// - `total_frames`: 总帧数
    /// - `frame_duration`: 每帧持续时间（秒）
    ///
    /// # 返回值
    /// 新创建的BallAnimation实例
    pub fn new(total_frames: usize, frame_duration: f32) -> Self {
        Self {
            current_frame: 0,
            total_frames,
            frame_duration,
            timer: 0.0,
        }
    }
}