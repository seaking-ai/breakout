use bevy::prelude::*;
use crate::config::*;
/// 挡板组件标记
#[derive(Component)]
pub struct Paddle;

/// 球组件标记
#[derive(Component)]
pub struct Ball;

/// 小球是否附着在挡板上
#[derive(Component)]
pub struct BallAttached;

/// 游戏提示UI组件标记
#[derive(Component)]
pub struct GameHintUi;



/// 速度组件，存储2D速度向量
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);



/// 碰撞事件，当球发生碰撞时触发
#[derive(Event)]
pub struct BallCollided;

/// 砖块组件标记
#[derive(Component)]
pub struct Brick;

/// 碰撞音效资源
#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

/// 碰撞器组件，用于碰撞检测
// 必须实现Default才能作为Wall的必需组件
#[derive(Component, Default)]
pub struct Collider;

/// 墙壁组件集合，定义游戏中的墙壁
#[derive(Component)]
#[require(Sprite, Transform, Collider)]
pub struct Wall;

/// 墙壁位置枚举，表示墙壁在游戏区域的哪一侧
#[derive(Component)]
pub enum WallLocation {
    Left,   // 左侧
    Right,  // 右侧
    Bottom, // 底部
    Top,    // 顶部
}

impl WallLocation {
    /// 返回墙壁中心的位置，用于transform.translation()
    /// 这是一个多行注释，说明了函数的功能和用途
    pub fn position(&self) -> Vec2 {
        match self {
            // 左墙位置，x坐标为LEFT_WALL常量-450，y坐标为0
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            // 右墙位置，x坐标为RIGHT_WALL常量，y坐标为0
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            // 底墙位置，x坐标为0，y坐标为BOTTOM_WALL常量
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            // 顶墙位置，x坐标为0，y坐标为TOP_WALL常量
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    /// 返回墙壁的(x, y)尺寸，用于transform.scale()
    /// 这是一个多行注释，说明了函数的功能和用途
    pub fn size(&self) -> Vec2 {
        // 计算竞技场高度，即顶墙和底墙之间的距离
        let arena_height = TOP_WALL - BOTTOM_WALL;
        // 计算竞技场宽度，即右墙和左墙之间的距离
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // 确保常量值正确，高度必须大于0
        assert!(arena_height > 0.0);
        // 确保常量值正确，宽度必须大于0
        assert!(arena_width > 0.0);

        match self {
            // 左墙和右墙的尺寸：宽度为墙壁厚度，高度为竞技场高度加上墙壁厚度
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            // 底墙和顶墙的尺寸：宽度为竞技场宽度加上墙壁厚度，高度为墙壁厚度
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    /// 构建器方法，用于创建墙壁实体
    // 注意Sprite和Transform与Wall一起使用，覆盖必需组件的默认值
    pub fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,                                    // 创建Wall组件实例
            Sprite::from_color(WALL_COLOR, Vec2::ONE), // 创建带有指定颜色和默认大小的精灵组件
            Transform {                             // 创建变换组件，定义墙壁的位置和大小
                // 将Vec2转换为Vec3，需要添加z坐标
                // 用于确定精灵的渲染顺序
                translation: location.position().extend(0.0), // 设置墙壁位置，从2D坐标扩展为3D坐标
                // 2D对象的z轴缩放必须始终为1.0，
                // 否则它们的渲染顺序会受到影响
                scale: location.size().extend(1.0),          // 设置墙壁大小，从2D尺寸扩展为3D尺寸
                ..default()                                // 使用Transform组件的默认值设置其他属性
            },
        )
    }
}

/// 分数资源，跟踪游戏得分
#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

/// 记分板UI组件标记
#[derive(Component)]
pub struct ScoreboardUi;
