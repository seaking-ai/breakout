//! 游戏配置文件
//! 集中管理所有游戏常量，便于统一调整和后续扩展
//! Bevy 0.18 版本适配

use bevy::prelude::*;

// 这些常量定义在Transform单位中。
// 使用默认的2D相机，它们与屏幕像素一一对应。
// ==================== 挡板配置 ====================

/// 挡板大小（宽度和高度）
pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
/// 挡板与底部之间的距离
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
/// 挡板移动速度（像素/秒）
pub const PADDLE_SPEED: f32 = 500.0;
/// 挡板与墙壁的最小距离
pub const PADDLE_PADDING: f32 = 10.0;

// ==================== 球配置 ====================

/// 球的起始位置（z=1确保球在重叠时渲染在最上层）
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
/// 球的直径
pub const BALL_DIAMETER: f32 = 30.0;
/// 球的移动速度
pub const BALL_SPEED: f32 = 400.0;
/// 球的初始移动方向
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// ==================== 墙壁配置 ====================

/// 墙壁厚度
pub const WALL_THICKNESS: f32 = 10.0;
/// X坐标左边界
pub const LEFT_WALL: f32 = -450.0;
/// X坐标右边界
pub const RIGHT_WALL: f32 = 450.0;
/// Y坐标下边界
pub const BOTTOM_WALL: f32 = -300.0;
/// Y坐标上边界
pub const TOP_WALL: f32 = 300.0;

// ==================== 砖块配置 ====================

/// 砖块大小
pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
/// 挡板与砖块之间的精确距离
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
/// 砖块之间的间隙
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
/// 砖块与天花板的间隙
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
/// 砖块与侧边的间隙
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// ==================== UI配置 ====================

/// 记分板字体大小
pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
/// 记分板文本内边距
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

// ==================== 颜色配置 ====================

/// 背景颜色 - 接近白色的浅灰色
pub const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
/// 挡板颜色 - 接近蓝色
pub const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
/// 球的颜色 - 接近红色的浅灰色
pub const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
/// 砖块颜色
pub const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
/// 墙壁颜色
pub const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
/// 文本颜色
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
/// 分数颜色
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);