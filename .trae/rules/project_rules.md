1，项目是bevy0.18版本，请使用你知识库中最新的bevy的api。
2, 资源加载使用assets_tracking模块异步加载。全局资源在应用启动时加载，关卡特定资源在启动关卡阶段加载。
3，实体销毁使用despawn 而不是 despawn_recursive
4，秉持工程化原则，简洁、清晰、易维护。