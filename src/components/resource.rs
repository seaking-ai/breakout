use bevy::prelude::*;

/// 分数资源，跟踪游戏得分
#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

/// 玩家生命数资源，跟踪剩余小球数量
///
/// # 默认值
/// 默认初始值为3，表示玩家有3次机会
#[derive(Resource, Deref, DerefMut)]
pub struct Lives(pub usize);

impl Default for Lives {
    fn default() -> Self {
        Lives(3)
    }
}


