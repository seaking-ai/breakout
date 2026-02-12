//! 打砖块游戏 - 菜单模块
//! Bevy 0.18 适配

pub use bevy::prelude::*;

// 导出菜单相关常量
pub mod constants {
    use bevy::prelude::*;

    /// 菜单标题字体大小
    pub const MENU_TITLE_FONT_SIZE: f32 = 60.0;
    /// 菜单按钮字体大小
    pub const MENU_BUTTON_FONT_SIZE: f32 = 40.0;
    /// 菜单按钮尺寸
    pub const MENU_BUTTON_SIZE: Vec2 = Vec2::new(300.0, 80.0);
    /// 菜单按钮间距
    pub const MENU_BUTTON_GAP: f32 = 40.0;
    /// 菜单按钮默认颜色
    pub const MENU_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
    /// 菜单按钮悬停颜色
    pub const MENU_BUTTON_HOVER_COLOR: Color = Color::srgb(0.4, 0.4, 0.8);
    /// 菜单标题颜色
    pub const MENU_TITLE_COLOR: Color = Color::srgb(0.2, 0.2, 0.6);
    /// 菜单背景颜色
    pub const MENU_BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
}

/// 菜单按钮组件标记
/// 用于标识菜单中的可交互按钮
#[derive(Component)]
pub struct MenuButton;

/// 菜单根节点组件标记
/// 用于标识菜单UI的根实体，便于清理
#[derive(Component)]
pub struct MenuRoot;

// 导出菜单系统
pub mod systems;
pub use systems::*;