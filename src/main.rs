//! 打砖块游戏示例 - 经典游戏的简化实现
//!
//! 演示了Bevy的步进调试功能（如果编译时启用了`bevy_debug_stepping`特性）
//!
//! 游戏控制：
//! - 使用左右方向键控制挡板移动
//! - 球会反弹并摧毁砖块
//! - 球碰到挡板、墙壁或砖块时会播放音效

//Aabb2d 二维轴对齐包围盒。 用来包裹一个复杂形状（角色模型）的最小矩形。通常用于碰撞检测的第一阶段（粗略检测），先快速排除掉明显不相交的物体。
//BoundingCircle 包围圆。定义一个中心点和半径，任何距离中心点小于半径的点都被认为在物体内。
//BoundingVolume (Trait) 这是一个抽象接口（Trait）。它定义了作为一个"包围体"必须具备的共同行为。
//IntersectsVolume (Trait) 抽象接口（Trait），专门用于定义"检测碰撞"的行为。
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

// 引入菜单模块
mod menu;
use menu::{setup_menu, handle_menu_input, cleanup_menu};


// 引入配置模块
mod config;
pub use config::*;

mod components;
pub use components::*;

/// 游戏状态枚举
/// 定义游戏的不同状态，用于状态管理
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    /// 菜单状态 - 显示开始菜单
    #[default]
    Menu,
    /// 游戏进行中状态
    Playing,
    /// 游戏结束状态
    GameOver,
}

/// 程序入口函数
/// 这个函数负责初始化并运行Bevy游戏引擎的应用程序
fn main() {
    App::new()  // 创建一个新的Bevy应用实例
        .add_plugins(DefaultPlugins)  // 添加默认插件，提供基础功能
        // 初始化游戏状态
        .init_state::<GameState>()
        // 添加步进调试插件
        // .add_plugins(
        //     stepping::SteppingPlugin::default()  // 使用默认配置创建步进调试插件
        //         .add_schedule(Update)  // 将步进功能添加到Update调度中
        //         .add_schedule(FixedUpdate)  // 将步进功能添加到FixedUpdate调度中
        //         .at(percent(35), percent(50)),  // 设置步进调试的触发位置在35%到50%之间
        // )
        .insert_resource(Score(0))  // 初始化分数资源为0
        .insert_resource(ClearColor(BACKGROUND_COLOR))  // 设置背景颜色
        
        // ===== 菜单状态系统 =====
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        
        // ===== 游戏进行中状态系统 =====
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            FixedUpdate,  // 使用固定时间步长运行系统
            (move_paddle, move_attached_ball, apply_velocity, check_for_collisions)
                .run_if(in_state(GameState::Playing))
                .chain(),
        )
        .add_systems(Update, (update_scoreboard, handle_ball_launch, update_hint_visibility).run_if(in_state(GameState::Playing)))
        .add_observer(play_collision_sound)
        
        .run();  // 运行应用程序
}



/// 设置游戏场景系统（游戏状态进入时调用）
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建和修改实体
/// - `meshes`: 网格资源集合，用于创建2D网格
/// - `materials`: 材质资源集合，用于创建材质
/// - `asset_server`: 资源服务器，用于加载游戏资源
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // Bevy 2D 渲染中最常用的一种材质类型。它通常包含一个颜色值和一个可选的纹理引用。用于定义 2D 网格（如精灵 Sprite）的外观。
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 注意：相机已在菜单状态创建，此处不再重复创建
    
    // 加载音效
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // 挡板的Y坐标位置  -300
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // 生成挡板
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            //.extend(1.0) 将其转换为 Vec3，通常用于 2D 游戏中保持 Z 轴缩放为 1。
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
    ));

    // 计算挡板的Y坐标位置
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    
    // 生成球（初始附着在挡板上）
    let ball_y = paddle_y + PADDLE_SIZE.y / 2.0 + BALL_DIAMETER / 2.0 + 5.0;
    commands.spawn((
        // 1. 2D 网格（形状）组件：添加一个默认的圆形网格到资源池，并关联到实体
        Mesh2d(meshes.add(Circle::default())),
        // 2. 2D 网格材质组件：添加指定颜色的材质到资源池，并关联到实体
        MeshMaterial2d(materials.add(BALL_COLOR)),
        // 3. 变换组件：控制实体的位置、缩放 with_scale(...)：设置小球的尺寸
        // 初始位置在挡板正上方
        Transform::from_translation(Vec3::new(0.0, ball_y, 1.0))
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
        Ball,
        // 初始时小球附着在挡板上，没有速度
        BallAttached,
    ));

    // 生成游戏提示文字
    // 计算小球所在的Y坐标（屏幕坐标系，原点在中心，向上为正）
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let ball_y = paddle_y + PADDLE_SIZE.y / 2.0 + BALL_DIAMETER / 2.0 + 5.0;
    // 文字放在小球上方50像素处
    let hint_y = ball_y + BALL_DIAMETER / 2.0 + 50.0;
    
    commands.spawn((
        Text2d::new("按 ↑ 方向键发射小球"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        GameHintUi,
        // 使用Transform在世界空间中定位文字
        Transform::from_translation(Vec3::new(0.0, hint_y, 10.0)),
    ));

    // 生成记分板
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            //绝对定位模式，脱离 Bevy 的自动布局流 top/left/bottom/right 控制位置（适合固定在屏幕角落的 UI）
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            //其他布局属性（如宽度、高度、背景色等）使用默认值。
            ..default()
        },
        //为父实体创建子实体，实现 UI 的嵌套结构；
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));

    // 生成四面墙壁
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));

    // 生成砖块
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // 根据可用空间计算可以放置多少行和列的砖块
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // 由于需要四舍五入列数，
    // 砖块顶部和两侧的空间只捕获下限值，而不是精确值
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // 砖块占用的空间
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        // 间隙占用的空间
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // 在Bevy中，实体的`translation`描述的是中心点，
    // 而不是左下角
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    // 生成砖块网格
    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // 生成砖块实体
            commands.spawn((
                Sprite {
                    color: BRICK_COLOR,
                    ..default()
                },
                Transform {
                    translation: brick_position.extend(0.0),
                    scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}

/// 挡板移动系统
///
/// # 参数
/// - `keyboard_input`: 键盘输入资源，用于检测键盘按键状态
/// - `paddle_transform`: 挡板变换组件，用于修改挡板的位置
/// - `time`: 时间资源，用于获取帧间时间差，实现平滑移动
fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    // 检测左右方向键输入 pressed持续按下   just_pressed（仅按键按下瞬间触发一次）。
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    // 根据玩家输入计算新的挡板水平位置
    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

    // 更新挡板位置，确保不会离开游戏区域
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;
    // clamp(left_bound, right_bound)：Rust 内置方法，将数值限制在指定区间内，是实现边界限制最简洁的方式。
    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

/// 应用速度系统，根据速度更新位置
///
/// # 参数
/// - `query`: 包含Transform和Velocity组件的查询
/// - `time`: 时间资源
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), Without<BallAttached>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

/// 移动附着在挡板上的小球系统
///
/// # 参数
/// - `ball_query`: 附着状态的小球查询
/// - `paddle_query`: 挡板变换组件查询
fn move_attached_ball(
    mut ball_query: Query<&mut Transform, With<BallAttached>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<BallAttached>)>,
) {
    //Bevy 的 Query 方法，要求查询结果必须只有 1 个实体，否则会返回错误（比如有多个挡板时 panic）；
    if let Ok(paddle_transform) = paddle_query.single() {
        for mut ball_transform in &mut ball_query {
            // 小球跟随挡板的X位置，Y位置保持在挡板上方
            let paddle_y = paddle_transform.translation.y;
            let ball_y = paddle_y + PADDLE_SIZE.y / 2.0 + BALL_DIAMETER / 2.0 + 5.0;
            ball_transform.translation.x = paddle_transform.translation.x;
            ball_transform.translation.y = ball_y;
        }
    }
}

/// 处理小球发射系统（按上方向键发射）
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `keyboard_input`: 键盘输入资源
/// - `ball_query`: 附着状态的小球查询
fn handle_ball_launch(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ball_query: Query<(Entity, &Transform), With<BallAttached>>,
) {
    // 检测上方向键是否被按下
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        for (ball_entity, _ball_transform) in &ball_query {
            // 移除附着状态，添加初始速度（垂直向上）
            commands.entity(ball_entity).remove::<BallAttached>();
            commands.entity(ball_entity).insert(Velocity(Vec2::new(0.0, BALL_SPEED)));
        }
    }
}

/// 更新提示文字可见性系统
///
/// # 参数
/// - `hint_query`: 提示UI查询
/// - `attached_ball_query`: 附着状态的小球查询
fn update_hint_visibility(
    mut hint_query: Query<&mut Visibility, With<GameHintUi>>,
    attached_ball_query: Query<(), With<BallAttached>>,
) {
    // 如果有小球附着在挡板上，显示提示；否则隐藏
    let has_attached_ball = !attached_ball_query.is_empty();
    for mut visibility in &mut hint_query {
        *visibility = if has_attached_ball {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// 更新记分板系统
///
/// # 参数
/// - `score`: 分数资源
/// - `score_root`: 记分板根实体
/// - `writer`: 文本UI写入器
fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

/// 碰撞检测系统
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `score`: 分数资源
/// - `ball_query`: 球实体查询
/// - `collider_query`: 碰撞器实体查询
/// - `paddle_query`: 挡板查询（用于检测是否碰撞到挡板）
fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    // 排除附着在挡板上的小球，避免与 move_attached_ball 系统冲突
    ball_query: Single<(&mut Velocity, &Transform), (With<Ball>, Without<BallAttached>)>,
    // 查询所有碰撞体：实体ID、位置、是否是砖块（Option<&Brick>），筛选带Collider标签的实体
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    // 解包小球的速度（可变）和位置组件
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    // 遍历所有碰撞体（墙、球拍、砖块）
    for (collider_entity, collider_transform, maybe_brick) in &collider_query {
        // 检测球与碰撞器的碰撞
        // ball_collision 是检测圆形（小球）与轴对齐矩形（AABB，碰撞体） 碰撞的核心函数，返回 Option<Collision>（None = 无碰撞，Some = 碰撞方向）。
        //enum Collision {
        //     Left,   // 小球撞到矩形的左侧
        //     Right,  // 小球撞到矩形的右侧
        //     Top,    // 小球撞到矩形的上侧
        //     Bottom, // 小球撞到矩形的下侧
        // }
        let collision = ball_collision(
            // BoundingCircle	小球碰撞盒	坐标（Vec2） + 半径（f32）
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            //Aabb2d	碰撞体碰撞盒	坐标（Vec2） + 半尺寸（Vec2）
            Aabb2d::new(
                collider_transform.translation.truncate(),  //将 Vec3（x/y/z）转为 Vec2（x/y），去掉 z 轴（2D 碰撞不需要 z 轴）；
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            // 触发碰撞事件
            commands.trigger(BallCollided);

            // 如果是砖块，销毁它并增加分数
            if maybe_brick.is_some() {
                commands.entity(collider_entity).despawn();
                **score += 1;
            }

            // 检查是否是挡板（不是砖块，且碰撞方向是顶部）
            let is_paddle = maybe_brick.is_none() && collision == Collision::Top;
            
            if is_paddle {
                // 挡板碰撞：根据击中位置改变反弹角度
                if let Ok(paddle_transform) = paddle_query.single() {
                    // 计算小球击中挡板的相对位置（-1.0到1.0，0表示中间）
                    let paddle_center = paddle_transform.translation.x;
                    let ball_x = ball_transform.translation.x;
                    let paddle_half_width = PADDLE_SIZE.x / 2.0;
                    
                    // 计算相对位置，范围 [-1, 1]
                    let relative_hit_pos = ((ball_x - paddle_center) / paddle_half_width).clamp(-1.0, 1.0);
                    
                    // 根据击中位置计算反弹角度
                    // 中间击中：垂直向上（0度）
                    // 边缘击中：最大60度倾斜
                    let max_angle = std::f32::consts::PI / 3.0; // 60度
                    let bounce_angle = relative_hit_pos * max_angle;
                    
                    // 计算新的速度方向
                    let new_velocity_x = bounce_angle.sin() * BALL_SPEED;
                    let new_velocity_y = bounce_angle.cos() * BALL_SPEED;
                    
                    ball_velocity.x = new_velocity_x;
                    ball_velocity.y = new_velocity_y.abs(); // 确保向上反弹
                }
            } else {
                // 墙壁和砖块碰撞：使用标准反射逻辑
                // 根据碰撞方向反射球的速度
                let mut reflect_x = false;
                let mut reflect_y = false;

                // 只有当速度方向与碰撞方向相反时才反射
                // 这可以防止球卡在挡板内部
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0, // 撞左墙 → 小球必须向右移才反射
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                }

                // 如果在x轴上发生碰撞，反射x轴速度
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // 如果在y轴上发生碰撞，反射y轴速度
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

/// 播放碰撞音效系统
///
/// # 参数
/// - `_collided`: 碰撞事件
/// - `commands`: 命令缓冲区
/// - `sound`: 碰撞音效资源
fn play_collision_sound(
    _collided: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>,
) {
    // 音效播放器组件：关联要播放的音效资源
    // 播放设置：播放完成后自动销毁播放器实体
    commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}

/// 碰撞方向枚举
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,   // 左侧碰撞
    Right,  // 右侧碰撞
    Top,    // 顶部碰撞
    Bottom, // 底部碰撞
}

/// 检测球与边界框的碰撞
///
/// # 参数
/// - `ball`: 球的边界圆
/// - `bounding_box`: 边界框
///
/// # 返回值
/// 如果发生碰撞，返回Some(Collision)表示碰撞的边；否则返回None
fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    //判断圆形是否与矩形有重叠区域（相交）。
    //如果小球完全在矩形外 → 返回 false，函数直接返回 None；
    //如果小球与矩形有重叠 → 返回 true，继续判断碰撞方向。
    if !ball.intersects(&bounding_box) {
        return None;
    }

    // 计算球心到边界框最近点的偏移量
    //找到矩形上离球心最近的点。
    let closest = bounding_box.closest_point(ball.center());
    //球心相对于矩形最近点的位置
    let offset = ball.center() - closest;

    // 根据偏移量判断碰撞的边
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
