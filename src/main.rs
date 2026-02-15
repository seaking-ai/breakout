//! 打砖块游戏示例 - 经典游戏的简化实现
//! 游戏控制：
//! - 使用左右方向键控制挡板移动
//! - 球会反弹并摧毁砖块
//! - 球碰到挡板、墙壁或砖块时会播放音效

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

// 引入菜单模块
mod menu;
use menu::{setup_menu, handle_menu_input, cleanup_menu};

// 引入关卡选择模块
mod level_select;
use level_select::{setup_level_select, handle_level_select_input, cleanup_level_select};

// 引入配置模块
mod config;
pub use config::*;

// 引入资产加载模块
mod assets_tracking;
use assets_tracking::LoadResource;

// 引入组件模块
mod components;
pub use components::*;

// 引入系统模块
mod system;
pub use system::*;

// 引入状态模块
mod state;
pub use state::*;

// 引入游戏模块
mod game;
pub use game::*;

/// 初始化相机系统
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建相机实体
///
/// # 说明
/// 在应用启动时创建一次2D相机，供整个游戏使用
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// 程序入口函数
/// 这个函数负责初始化并运行Bevy游戏引擎的应用程序
fn main() {
    App::new()  // 创建一个新的Bevy应用实例
        .add_plugins(DefaultPlugins)  // 添加默认插件，提供基础功能
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        // 添加资产追踪插件
        .add_plugins(assets_tracking::plugin)
        // 使用 assets_tracking 异步加载音效资源
        .load_resource::<GameSounds>()
        // 初始化游戏状态
        .init_state::<GameState>()
        .init_state::<PlayingState>()
        .insert_resource(Score(0))  // 初始化分数资源为0
        .insert_resource(Lives::default())  // 初始化生命数资源为默认值3
        .insert_resource(Level::default())  // 初始化关卡资源为默认值（第一关）
        .insert_resource(ClearColor(BACKGROUND_COLOR))  // 设置背景颜色
        // 在启动时创建相机，只运行一次
        .add_systems(Startup, setup_camera)

        // ===== 菜单状态系统 =====
        .add_systems(OnEnter(GameState::Menu), (setup_menu, reset_game_state))
        .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)

        // ===== 关卡选择状态系统 =====
        .add_systems(OnEnter(GameState::LevelSelect), setup_level_select)
        .add_systems(Update, handle_level_select_input.run_if(in_state(GameState::LevelSelect)))
        .add_systems(OnExit(GameState::LevelSelect), cleanup_level_select)

        // ===== 游戏进行中状态系统 =====
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(OnExit(GameState::Playing), cleanup_game_play)
        .add_systems(OnEnter(PlayingState::ball_attached), reset_ball)
        
        // 球附着状态系统
        .add_systems(
            Update,
            (
                move_attached_ball,
                handle_ball_launch,
                update_hint_visibility,
            )
                .run_if(in_state(GameState::Playing).and(in_state(PlayingState::ball_attached))
                ).chain(),
        )
        
        // 球发射后游戏进行中系统
        .add_systems(
            Update,
            (
                update_scoreboard, 
                check_for_collisions
            )
                .run_if(in_state(GameState::Playing).and(in_state(PlayingState::ball_launched))),
        )
        
        // 两种状态都需要运行的系统
        .add_systems(
            Update,
            (
                animate_ball_sprite, 
                update_lives_ui
            )
                .run_if(in_state(GameState::Playing)),
        )
        
        // 固定时间步长系统
        .add_systems(
            FixedUpdate,
            (
                move_paddle, 
                apply_velocity
            )
                .run_if(in_state(GameState::Playing))
                .chain(),
        )


        // ===== 游戏结束状态系统 =====
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, handle_game_over_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)

        // ===== 游戏胜利状态系统 =====
        .add_systems(OnEnter(GameState::Victory), setup_victory)
        .add_systems(Update, handle_victory_input.run_if(in_state(GameState::Victory)))
        .add_systems(OnExit(GameState::Victory), cleanup_victory)

        .add_observer(play_collision_sound)
        .run();  // 运行应用程序
}






