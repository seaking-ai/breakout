// 游戏胜利界面模块

use bevy::prelude::*;
use crate::state::GameState;
use crate::components::{VictoryRoot, BackToMenuButton};
use crate::Score;

/// 游戏胜利菜单常量配置
mod constants {
    use bevy::prelude::*;

    /// 游戏胜利背景颜色（半透明黑色遮罩）
    pub const VICTORY_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
    /// 游戏胜利标题颜色（金色）
    pub const VICTORY_TITLE_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
    /// 最终分数颜色
    pub const FINAL_SCORE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
    /// 按钮颜色
    pub const BUTTON_COLOR: Color = Color::srgb(0.3, 0.5, 0.8);
    /// 按钮悬停颜色
    pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.4, 0.6, 0.9);
    /// 标题字体大小
    pub const TITLE_FONT_SIZE: f32 = 60.0;
    /// 分数字体大小
    pub const SCORE_FONT_SIZE: f32 = 40.0;
    /// 按钮字体大小
    pub const BUTTON_FONT_SIZE: f32 = 30.0;
    /// 按钮尺寸
    pub const BUTTON_SIZE: Vec2 = Vec2::new(200.0, 60.0);
}

/// 设置游戏胜利界面系统
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建UI实体
/// - `asset_server`: 资源服务器，用于加载字体资源
/// - `score`: 分数资源，用于显示最终得分
///
/// # 功能
/// 创建游戏胜利界面，包含标题、最终分数和返回菜单按钮
pub fn setup_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    // 游戏胜利界面根节点（全屏遮罩）
    let victory_root = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(constants::VICTORY_BACKGROUND),
        VictoryRoot,
    );

    // 游戏胜利标题
    let title_text = (
        Text::new("VICTORY!"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(constants::VICTORY_TITLE_COLOR),
    );

    // 最终分数显示
    let score_text = (
        Text::new(format!("最终分数: {}", score.0)),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::SCORE_FONT_SIZE,
            ..default()
        },
        TextColor(constants::FINAL_SCORE_COLOR),
    );

    // 返回菜单按钮
    let back_button = (
        Button,
        Node {
            width: Val::Px(constants::BUTTON_SIZE.x),
            height: Val::Px(constants::BUTTON_SIZE.y),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(constants::BUTTON_COLOR),
        BackToMenuButton,
    );

    // 按钮文本
    let button_text = (
        Text::new("返回菜单"),
        TextFont {
            font: asset_server.load("fonts/songti.ttf"),
            font_size: constants::BUTTON_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
    );

    // 生成游戏胜利界面实体树
    commands
        .spawn(victory_root)
        .with_children(|parent| {
            parent.spawn(title_text);
            parent.spawn(score_text);
            parent
                .spawn(back_button)
                .with_children(|btn_parent| {
                    btn_parent.spawn(button_text);
                });
        });
}

/// 处理游戏胜利界面按钮交互系统
///
/// # 参数
/// - `interaction_query`: 查询按钮的交互状态、背景颜色和子实体
/// - `next_state`: 下一个游戏状态资源，用于切换状态
/// - `text_query`: 查询文本组件，用于修改按钮文本颜色
///
/// # 功能
/// - 悬停时改变按钮颜色
/// - 点击时返回主菜单状态
pub fn handle_victory_input(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<BackToMenuButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                color.0 = constants::BUTTON_HOVER_COLOR;
                if let Ok(mut text_color) = text_query.get_mut(children[0]) {
                    text_color.0 = Color::srgb(0.9, 0.9, 0.9);
                }
            }
            Interaction::Pressed => {
                // 返回主菜单状态
                next_state.set(GameState::Menu);
            }
            Interaction::None => {
                color.0 = constants::BUTTON_COLOR;
                if let Ok(mut text_color) = text_query.get_mut(children[0]) {
                    text_color.0 = Color::WHITE;
                }
            }
        }
    }
}

/// 清理游戏胜利界面系统
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `victory_entities`: 查询所有带有 VictoryRoot 标记的实体
///
/// # 说明
/// 递归销毁所有游戏胜利界面相关实体
pub fn cleanup_victory(
    mut commands: Commands,
    victory_entities: Query<Entity, With<VictoryRoot>>,
) {
    for entity in &victory_entities {
        commands.entity(entity).despawn();
    }
}
