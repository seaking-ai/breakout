//! 关卡选择模块
//! 提供关卡选择界面，允许玩家选择不同的关卡进行游戏

use bevy::prelude::*;
use crate::state::{GameState, Level};
use crate::menu::constants;

/// 关卡选择UI根节点标记组件
#[derive(Component)]
pub struct LevelSelectRoot;

/// 关卡选择按钮类型
#[derive(Component)]
pub enum LevelButton {
    /// 第一关按钮
    Level1,
    /// 第二关按钮
    Level2,
}

/// 初始化关卡选择界面系统
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建UI实体
/// - `asset_server`: 资源服务器，用于加载字体资源
///
/// # 说明
/// 创建关卡选择界面，包含标题和两个关卡选择按钮
pub fn setup_level_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 关卡选择容器（垂直布局）
    let container = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(constants::MENU_BACKGROUND_COLOR),
        LevelSelectRoot,
    );

    // 标题文本
    let title_text = (
        Text::new("选择关卡"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::MENU_TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(constants::MENU_TITLE_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(30.0)),
            ..default()
        },
    );

    // 第一关按钮文本
    let level1_text = (
        Text::new("第一关 - 简单"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::MENU_BUTTON_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
    );

    // 第一关按钮
    let level1_button = (
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(60.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(constants::MENU_BUTTON_COLOR),
        LevelButton::Level1,
    );

    // 第二关按钮文本
    let level2_text = (
        Text::new("第二关 - 困难"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::MENU_BUTTON_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
    );

    // 第二关按钮
    let level2_button = (
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(60.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(constants::MENU_BUTTON_COLOR),
        LevelButton::Level2,
    );

    // 生成关卡选择界面实体树
    commands
        .spawn(container)
        .with_children(|parent| {
            // 添加标题
            parent.spawn(title_text);
            // 添加第一关按钮
            parent
                .spawn(level1_button)
                .with_children(|btn_parent| {
                    btn_parent.spawn(level1_text);
                });
            // 添加第二关按钮
            parent
                .spawn(level2_button)
                .with_children(|btn_parent| {
                    btn_parent.spawn(level2_text);
                });
        });
}

/// 处理关卡选择按钮交互系统
///
/// # 参数
/// - `interaction_query`: 查询按钮的交互状态、背景颜色和关卡按钮类型
/// - `next_state`: 下一个游戏状态资源
/// - `level_resource`: 当前关卡资源（可变）
///
/// # 功能
/// - 悬停时改变按钮颜色
/// - 点击时设置对应关卡并切换到游戏状态
pub fn handle_level_select_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &LevelButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_resource: ResMut<Level>,
) {
    for (interaction, mut color, level_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                color.0 = constants::MENU_BUTTON_HOVER_COLOR;
            }
            Interaction::Pressed => {
                // 根据按钮类型设置关卡
                match level_button {
                    LevelButton::Level1 => {
                        *level_resource = Level::Level1;
                    }
                    LevelButton::Level2 => {
                        *level_resource = Level::Level2;
                    }
                }
                // 切换到游戏状态
                next_state.set(GameState::Playing);
            }
            Interaction::None => {
                color.0 = constants::MENU_BUTTON_COLOR;
            }
        }
    }
}

/// 清理关卡选择界面系统
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `level_select_entities`: 查询所有带有 LevelSelectRoot 标记的实体
///
/// # 说明
/// 在切换到游戏状态前调用，清理关卡选择界面
pub fn cleanup_level_select(
    mut commands: Commands,
    level_select_entities: Query<Entity, With<LevelSelectRoot>>,
) {
    for entity in &level_select_entities {
        commands.entity(entity).despawn();
    }
}
