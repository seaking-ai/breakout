// 游戏的主体部分

use bevy::prelude::*;
use crate::config::*;
use crate::components::*;
use crate::state::{GameState, Level, PlayingState};
//Aabb2d 二维轴对齐包围盒。 用来包裹一个复杂形状（角色模型）的最小矩形。通常用于碰撞检测的第一阶段（粗略检测），先快速排除掉明显不相交的物体。
//BoundingCircle 包围圆。定义一个中心点和半径，任何距离中心点小于半径的点都被认为在物体内。
//BoundingVolume (Trait) 这是一个抽象接口（Trait）。它定义了作为一个"包围体"必须具备的共同行为。
//IntersectsVolume (Trait) 抽象接口（Trait），专门用于定义"检测碰撞"的行为。
use bevy::{math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},};

/// 设置游戏场景系统（游戏状态进入时调用）
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建和修改实体
/// - `meshes`: 网格资源集合，用于创建2D网格
/// - `materials`: 材质资源集合，用于创建材质
/// - `asset_server`: 资源服务器，用于加载游戏资源
/// - `lives`: 生命数资源，用于显示剩余小球数量
/// - `current_level`: 当前关卡资源，决定生成多少行砖块
pub fn setup_game(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    // Bevy 2D 渲染中最常用的一种材质类型。它通常包含一个颜色值和一个可选的纹理引用。用于定义 2D 网格（如精灵 Sprite）的外观。
    _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    lives: Res<Lives>,
    current_level: Res<Level>,
) {
    // 注意：相机已在菜单状态创建，此处不再重复创建
    
    // 注意：音效资源现在通过 assets_tracking 模块在应用启动时异步加载
    // 此处不再需要手动加载和插入 CollisionSound 资源

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
        GamePlayRoot,
    ));

    // 计算挡板的Y坐标位置
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    
    // 加载精灵图集纹理 (256x64，4列，每帧64x64)
    let ball_texture = asset_server.load("images/sprite (1).png");
    // 创建纹理图集布局：4列1行，每个精灵64x64
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
    let atlas_layout_handle = atlas_layouts.add(atlas_layout);
    
    // 生成球（初始附着在挡板上）
    let ball_y = paddle_y + PADDLE_SIZE.y / 2.0 + BALL_DIAMETER / 2.0 + 5.0;
    commands.spawn((
        // 使用精灵图集，显示第0帧（第一列）
        Sprite::from_atlas_image(
            ball_texture,
            TextureAtlas {
                layout: atlas_layout_handle,
                index: 0,
            },
        ),
        // 变换组件：控制实体的位置、缩放
        // 初始位置在挡板正上方，设置缩放使图片大小符合BALL_DIAMETER
        Transform::from_translation(Vec3::new(0.0, ball_y, 1.0))
            .with_scale(Vec2::splat(BALL_DIAMETER / 64.0).extend(1.)),
        Ball,
        // 初始时小球附着在挡板上，没有速度
        BallAttached,
        // 添加动画组件：4帧，每帧0.15秒
        BallAnimation::new(4, 0.15),
        GamePlayRoot,
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
        GamePlayRoot,
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
        GamePlayRoot,
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

    // 生成生命数显示区域（右上角）
    spawn_lives_ui(&mut commands, lives.0);

    // 生成四面墙壁
    spawn_wall_with_marker(&mut commands, WallLocation::Left);
    spawn_wall_with_marker(&mut commands, WallLocation::Right);
    spawn_wall_with_marker(&mut commands, WallLocation::Bottom);
    spawn_wall_with_marker(&mut commands, WallLocation::Top);

    // 生成砖块
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // 根据可用空间计算可以放置多少行和列的砖块
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    // 根据当前关卡决定行数：第一关3行，第二关填满
    let max_n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = match *current_level {
        Level::Level1 => max_n_rows.min(3), // 第一关最多3行
        Level::Level2 => max_n_rows,        // 第二关填满
    };
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
                GamePlayRoot,
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
pub fn move_paddle(
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
pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), Without<BallAttached>>, time: Res<Time>) {
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
pub fn move_attached_ball(
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
pub fn handle_ball_launch(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ball_query: Query<(Entity, &Transform), With<BallAttached>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    // 检测上方向键是否被按下
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        for (ball_entity, _ball_transform) in &ball_query {
            // 移除附着状态，添加初始速度（垂直向上）
            commands.entity(ball_entity).remove::<BallAttached>();
            commands.entity(ball_entity).insert(Velocity(Vec2::new(0.0, BALL_SPEED)));
        }
        // 切换到游戏进行中状态
        next_state.set(PlayingState::ball_launched);
    }
}

/// 更新提示文字可见性系统
///
/// # 参数
/// - `hint_query`: 提示UI查询
/// - `attached_ball_query`: 附着状态的小球查询
pub fn update_hint_visibility(
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
pub fn update_scoreboard(
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
/// - `lives`: 生命数资源
/// - `ball_query`: 球实体查询
/// - `collider_query`: 碰撞器实体查询（包含墙壁位置信息）
/// - `paddle_query`: 挡板查询（用于检测是否碰撞到挡板）
/// - `next_playing_state`: 下一个游戏进行中状态
/// - `next_game_state`: 下一个游戏状态
///
/// # 逻辑
/// 检测小球与各种碰撞体的碰撞：
/// - 砖块：销毁砖块并增加分数
/// - 挡板：根据击中位置改变反弹角度
/// - 底部墙壁：触发失败逻辑，减少生命数
/// - 其他墙壁：标准反射
pub fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    // 排除附着在挡板上的小球，避免与 move_attached_ball 系统冲突
    ball_query: Single<(Entity, &mut Velocity, &Transform), (With<Ball>, Without<BallAttached>)>,
    // 查询所有碰撞体：实体ID、位置、是否是砖块（Option<&Brick>）、墙壁位置（Option<&WallLocation>），筛选带Collider标签的实体
    collider_query: Query<(Entity, &Transform, Option<&Brick>, Option<&WallLocation>), With<Collider>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // 解包小球的实体ID、速度（可变）和位置组件
    //into_inner() 的作用：将 Bevy Query 返回的 "结果包装类型"转换为直接可操作的引用 / 值，确保单个
    let (ball_entity, mut ball_velocity, ball_transform) = ball_query.into_inner();

    // 遍历所有碰撞体（墙、球拍、砖块）
    for (collider_entity, collider_transform, maybe_brick, maybe_wall_location) in &collider_query {
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

            // 检查是否碰撞到底部墙壁（失败条件）
            let is_bottom_wall = maybe_wall_location.map_or(false, |loc| matches!(loc, WallLocation::Bottom));
            
            if is_bottom_wall {
                // 小球碰到底部墙壁，触发失败逻辑
                // 减少生命数
                lives.0 = lives.0.saturating_sub(1);

                if lives.0 == 0 {
                    // 生命数归零，游戏结束
                    next_game_state.set(GameState::GameOver);
                } else {
                    // 还有剩余生命，销毁当前小球并重置
                    commands.entity(ball_entity).despawn();
                    next_playing_state.set(PlayingState::ball_attached);
                }
                // 底部墙壁碰撞后不再执行其他碰撞逻辑
                return;
            }

            // 如果是砖块，销毁它并增加分数
            if maybe_brick.is_some() {
                commands.entity(collider_entity).despawn();
                **score += 1;

                // 检查是否还有剩余砖块
                // 通过查询所有带有Brick组件的实体来统计剩余砖块数量
                let remaining_bricks = collider_query.iter().filter(|(_, _, brick, _)| brick.is_some()).count();
                // 注意：当前被碰撞的砖块已经被despawn，但查询结果中仍然包含它
                // 所以剩余砖块数量为1时表示这是最后一个砖块
                if remaining_bricks <= 1 {
                    // 所有砖块被消灭，切换到胜利状态
                    next_game_state.set(GameState::Victory);
                    return;
                }
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
pub fn play_collision_sound(
    _collided: On<BallCollided>,
    mut commands: Commands,
    sound: Res<GameSounds>,
) {
    // 音效播放器组件：关联要播放的音效资源
    // 播放设置：播放完成后自动销毁播放器实体
    commands.spawn((AudioPlayer(sound.collision.clone()), PlaybackSettings::DESPAWN));
}



/// 检测球与边界框的碰撞
///
/// # 参数
/// - `ball`: 球的边界圆
/// - `bounding_box`: 边界框
///
/// # 返回值
/// 如果发生碰撞，返回Some(Collision)表示碰撞的边；否则返回None
pub fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
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

/// 生成生命数UI显示
///
/// # 参数
/// - `commands`: 命令缓冲区，用于创建实体
/// - `lives_count`: 当前生命数量
fn spawn_lives_ui(commands: &mut Commands, lives_count: usize) {
    // 创建生命指示器容器
    let mut container = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: LIVES_UI_TOP_PADDING,
            right: LIVES_UI_PADDING,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(LIFE_INDICATOR_GAP),
            ..default()
        },
        LivesUi,
        GamePlayRoot,
    ));

    // 添加生命指示器（小球图标）
    for _i in 0..lives_count {
        container.with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(LIFE_INDICATOR_SIZE),
                    height: Val::Px(LIFE_INDICATOR_SIZE),
                    border_radius: BorderRadius::all(Val::Px(LIFE_INDICATOR_SIZE / 2.0)),
                    ..default()
                },
                BackgroundColor(BALL_COLOR),
                LifeIndicator,
            ));
        });
    }
}

/// 更新生命数UI显示系统
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `lives`: 生命数资源
/// - `lives_ui_query`: 生命数UI容器查询
/// - `life_indicators_query`: 生命指示器查询
pub fn update_lives_ui(
    mut commands: Commands,
    lives: Res<Lives>,
    lives_ui_query: Query<Entity, With<LivesUi>>,
    life_indicators_query: Query<Entity, With<LifeIndicator>>,
) {
    // 只有当生命数发生变化时才更新
    if lives.is_changed() {
        // 删除所有现有的生命指示器
        for entity in &life_indicators_query {
            commands.entity(entity).despawn();
        }

        // 重新生成生命指示器
        if let Ok(container_entity) = lives_ui_query.single() {
            commands.entity(container_entity).with_children(|parent| {
                for _ in 0..lives.0 {
                    parent.spawn((
                        Node {
                            width: Val::Px(LIFE_INDICATOR_SIZE),
                            height: Val::Px(LIFE_INDICATOR_SIZE),
                            border_radius: BorderRadius::all(Val::Px(LIFE_INDICATOR_SIZE / 2.0)),
                            ..default()
                        },
                        BackgroundColor(BALL_COLOR),
                        LifeIndicator,
                    ));
                }
            });
        }
    }
}

/// 生成带标记的墙壁实体
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `location`: 墙壁位置
///
/// # 说明
/// 创建墙壁实体并添加 GamePlayRoot 标记，便于在游戏结束时清理
fn spawn_wall_with_marker(commands: &mut Commands, location: WallLocation) {
    let (wall, location, sprite, transform) = Wall::new(location);
    commands.spawn((
        wall,
        location,
        sprite,
        transform,
        Collider,
        GamePlayRoot,
    ));
}

/// 清理游戏实体系统
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `game_play_entities`: 查询所有带有 GamePlayRoot 标记的实体
///
/// # 说明
/// 在游戏状态切换时调用，清理所有游戏场景实体（挡板、小球、墙壁、砖块等）
pub fn cleanup_game_play(
    mut commands: Commands,
    game_play_entities: Query<Entity, With<GamePlayRoot>>,
) {
    for entity in &game_play_entities {
        commands.entity(entity).despawn();
    }
}

/// 重置小球系统（在切换到ball_attached状态时调用）
///
/// # 参数
/// - `commands`: 命令缓冲区
/// - `asset_server`: 资源服务器，用于加载图片资源
/// - `atlas_layouts`: 纹理图集布局资源
/// - `paddle_query`: 挡板查询
pub fn reset_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    if let Ok(paddle_transform) = paddle_query.single() {
        let paddle_y = paddle_transform.translation.y;
        let ball_y = paddle_y + PADDLE_SIZE.y / 2.0 + BALL_DIAMETER / 2.0 + 5.0;

        // 加载精灵图集纹理 (256x64，4列，每帧64x64)
        let ball_texture = asset_server.load("images/sprite (1).png");
        // 创建纹理图集布局：4列1行，每个精灵64x64
        let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
        let atlas_layout_handle = atlas_layouts.add(atlas_layout);

        // 生成新的小球，附着在挡板上
        commands.spawn((
            Sprite::from_atlas_image(
                ball_texture,
                TextureAtlas {
                    layout: atlas_layout_handle,
                    index: 0,
                },
            ),
            Transform::from_translation(Vec3::new(paddle_transform.translation.x, ball_y, 1.0))
                .with_scale(Vec2::splat(BALL_DIAMETER / 64.0).extend(1.)),
            Ball,
            BallAttached,
            // 添加动画组件：4帧，每帧0.15秒
            BallAnimation::new(4, 0.15),
            GamePlayRoot,
        ));
    }
}

/// 小球动画系统
/// 循环切换精灵图集的帧，实现动画效果
///
/// # 参数
/// - `time`: 时间资源，用于获取帧间时间差
/// - `query`: 查询带有BallAnimation和Sprite组件的小球
///
/// # 逻辑
/// 每帧累加时间到timer，当超过frame_duration时切换到下一帧
pub fn animate_ball_sprite(
    time: Res<Time>,
    mut query: Query<(&mut BallAnimation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        // 累加时间
        animation.timer += time.delta_secs();

        // 检查是否需要切换到下一帧
        if animation.timer >= animation.frame_duration {
            animation.timer = 0.0;
            // 切换到下一帧，循环播放
            animation.current_frame = (animation.current_frame + 1) % animation.total_frames;
            // 更新图集索引
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation.current_frame;
            }
        }
    }
}
