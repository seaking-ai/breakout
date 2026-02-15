//! 菜单系统实现
//! 包含菜单初始化、交互处理、清理逻辑
//! Bevy 0.18 版本适配

use bevy::prelude::*;
use super::{constants, MenuButton};
use crate::state::GameState;  // 引用主程序定义的游戏状态

/// 菜单UI根节点标记组件
/// 用于后续清理菜单时识别菜单相关实体
#[derive(Component)]
pub struct MenuRoot;

/// 初始化菜单UI系统
/// 
/// # 参数
/// - `commands`: 命令缓冲区，用于创建UI实体
/// - `asset_server`: 资源服务器，用于加载字体资源
/// 
/// # 说明
/// 创建菜单界面，包含标题和开始游戏按钮
/// 注意：相机在应用启动时已经创建，此处不再重复创建
pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

    // 菜单容器（垂直布局）- Bevy 0.18 使用 Node 组件替代 NodeBundle
    //Node 是 Bevy UI 中最基础的布局容器组件，用于控制元素的尺寸、排列方式等核心布局属性：
    let menu_container = (
        Node {
            width: Val::Percent(100.0),  // 宽度占父容器的 100%（全屏宽度）
            height: Val::Percent(100.0),   // 高度占父容器的 100%（全屏高度）
            align_items: AlignItems::Center,  // 垂直方向（交叉轴）上居中对齐子元素
            justify_content: JustifyContent::Center,  // 水平方向（主轴）上居中对齐子元素
            flex_direction: FlexDirection::Column,  // 子元素按垂直列方向排列（从上到下）
            ..default()
        },
        //BackgroundColor 是 Bevy 提供的 UI 组件，用于设置容器的背景颜色。
        BackgroundColor(constants::MENU_BACKGROUND_COLOR), 
        MenuRoot,  // 标记为菜单根节点，便于清理
    );

    // 标题文本 - Bevy 0.18 使用 Text 和 TextFont 组件
    let title_text = (
        Text::new("打砖块游戏"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"), // 使用项目中已有的字体
            font_size: constants::MENU_TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(constants::MENU_TITLE_COLOR),
        Node {
            //给文本的底部添加 50 像素的外边距，作用是留出间距，避免元素挤在一起。
            //UiRect 还支持 top/left/right/all 等方法
            margin: UiRect::bottom(Val::Px(50.0)),
            ..default()
        },
    );

    // 开始游戏按钮 - Bevy 0.18 使用 Button 组件
    let start_button = (
        //Bevy 内置的，赋予实体 “可点击按钮” 的交互能力。
        Button,
        Node {
            width: Val::Px(constants::MENU_BUTTON_SIZE.x),
            height: Val::Px(constants::MENU_BUTTON_SIZE.y),
            align_items: AlignItems::Center,  // 按钮内文本垂直居中
            justify_content: JustifyContent::Center,  // 按钮内文本水平居中
            ..default()
        },
        BackgroundColor(constants::MENU_BUTTON_COLOR),
        MenuButton,  // 标记为菜单按钮
    );

    // 按钮文本
    let button_text = (
        Text::new("开始游戏"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::MENU_BUTTON_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
    );

    // 生成菜单实体树 - Bevy 0.18 使用 children! 宏
    commands
        .spawn(menu_container)
        .with_children(|parent| {
            // 添加标题
            parent.spawn(title_text);
            // 添加开始按钮
            parent
                .spawn(start_button)
                .with_children(|btn_parent| {
                    btn_parent.spawn(button_text);
                });
        });
}

/// 处理菜单按钮交互（悬停/点击）系统
/// 
/// # 参数
/// - `interaction_query`: 查询按钮的交互状态、背景颜色和子实体
/// - `next_state`: 下一个游戏状态资源，用于切换状态
/// - `text_query`: 查询文本组件，用于修改按钮文本颜色
/// 
/// # 功能
/// - 悬停时改变按钮颜色
/// - 点击时切换到游戏状态
pub fn handle_menu_input(
    //✅ &Interaction：读取按钮的交互状态（悬停 / 点击 / 无交互）；
    //✅ &mut BackgroundColor：可变引用按钮的背景色，用于动态修改；
    //✅ &Children：读取按钮的子实体列表（按钮文本是按钮的子实体）。
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<MenuButton>),
    >,
    //Bevy 的状态管理资源，用于修改游戏全局状态（比如从 Menu 切到 Playing）。
    mut next_state: ResMut<NextState<GameState>>,
    //用于查询并修改文本的颜色组件
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            // 鼠标悬停
            Interaction::Hovered => {
                color.0 = constants::MENU_BUTTON_HOVER_COLOR;
                // 修改按钮文本颜色 
                //text_query.get_mut(children[0])：通过文本实体 ID 获取文本的 TextColor 组件并修改，
                //将文本颜色改为浅灰色（srgb(0.9, 0.9, 0.9)），提升悬停时的视觉反馈。
                if let Ok(mut text_color) = text_query.get_mut(children[0]) {
                    text_color.0 = Color::srgb(0.9, 0.9, 0.9);
                }
            }
            // 鼠标点击
            Interaction::Pressed => {
                // 切换到关卡选择状态
                next_state.set(GameState::LevelSelect);
            }
            // 无交互
            Interaction::None => {
                color.0 = constants::MENU_BUTTON_COLOR;
                if let Ok(mut text_color) = text_query.get_mut(children[0]) {
                    text_color.0 = Color::WHITE;
                }
            }
        }
    }
}

/// 清理菜单系统（切换到游戏前调用）
/// 
/// # 参数
/// - `commands`: 命令缓冲区
/// - `menu_entities`: 查询所有带有 MenuRoot 标记的实体
/// 
/// # 说明
/// 递归销毁所有菜单相关实体，避免残留UI影响游戏画面
/// 
/// # 注意
/// Bevy 0.18 中 `despawn` 方法默认会递归销毁实体及其所有子实体
pub fn cleanup_menu(mut commands: Commands, menu_entities: Query<Entity, With<MenuRoot>>) {
    for entity in &menu_entities {
        commands.entity(entity).despawn();
    }
}