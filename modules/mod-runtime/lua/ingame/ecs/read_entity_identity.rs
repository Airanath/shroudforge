use super::*;

/// `runtime.ecs.entity_id(entity)`: the real `keen::EntityId.id` of a current
/// ShroudForge handle, the inverse of `runtime.ecs.resolve`. Mods need it where
/// the game addresses an entity by its id (for example an interaction target)
/// but the mod only holds the handle from `query` or `query_bounds`. The id is
/// read from the generation-checked live entity behind the handle; a stale or
/// unknown handle returns `nil, reason`.
pub(super) fn lua_ecs_entity_id(
    lua: &mlua::Lua,
    args: FunctionArgs,
    r#mod: &Mod,
) -> mlua::Result<(LuaValue, Option<String>)> {
    let state = lua.app_data_ref::<AppState>().unwrap();
    // Same permission as resolve: both directions of the same identity mapping.
    if let Some(reason) = runtime_denial_reason(&state, r#mod, "runtime.ecs.resolve") {
        return Ok((LuaValue::Nil, Some(reason)));
    }
    let handle = args.get::<u32>(0)?;
    match runtime_provider::world_entity_identity(handle) {
        Some(entity_id) => Ok((LuaValue::Integer(i64::from(entity_id)), None)),
        None => Ok((
            LuaValue::Nil,
            Some(format!("live ECS entity unavailable for handle {handle}")),
        )),
    }
}
