//! A high-level way to load collections of asset handles as resources.

use std::collections::VecDeque;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();  // 初始化资源追踪器
    app.add_systems(PreUpdate, load_resource_assets);  // 注册加载检查系统
}

pub trait LoadResource {
    /// This will load the [`Resource`] as an [`Asset`]. When all of its asset dependencies
    /// have been loaded, it will be inserted as a resource. This ensures that the resource only
    /// exists when the assets are ready.
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        // 步骤1：初始化T为Asset类型（让Bevy识别T是可加载的资产）
        self.init_asset::<T>();
         // 步骤2：从World创建T的实例（FromWorld trait：从世界中获取依赖来初始化）
        let world = self.world_mut();
        let value = T::from_world(world);
        // 步骤3：通过AssetServer添加实例，生成资产句柄
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
             // 步骤4：将句柄和插入函数加入等待队列
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

/// A function that inserts a loaded resource.
type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    // Use a queue for waiting assets so they can be cycled through and moved to
    // `finished` one at a time.
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>, // 待加载的句柄+插入函数
    finished: Vec<UntypedHandle>,  // 已加载完成的句柄
}

impl ResourceHandles {
    /// Returns true if all requested [`Asset`]s have finished loading and are available as [`Resource`]s.
    pub fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }
}

fn load_resource_assets(world: &mut World) {
        // 作用域1：获取ResourceHandles的可变引用
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
         // 作用域2：获取AssetServer的可变引用
        world.resource_scope(|world, assets: Mut<AssetServer>| {
             // 遍历当前等待队列的所有元素（按长度循环，避免队列变化影响）
            for _ in 0..resource_handles.waiting.len() {
                 // 弹出队首元素
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                // 检查资产及其所有依赖是否加载完成
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn)); // 放回队尾，下次再检查
                }
            }
        });
    });
}