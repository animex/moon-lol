---
trigger: always_on
---

# Bevy 0.17

1. Do not use EventReader<CustomEvent> in systems. Use On<CustomEntityEvent> in observers instead
2. Do not use commands.trigger_targets. Use commands.trigger(CustomEntityEvent { entity, ... }) instead
3. Do not use Camera3dBundle, MeshBundle. XxxBundle type APIs are deprecated. Bevy now uses the require macro to implicitly create required components
4. System naming convention: {schedule}_{function}, e.g.: update_movement, fixed_update_attack_enemy
5. Observer naming convention: on_{event_name}, e.g.: on_command_attack_start, on_event_died. First parameter should be named event, e.g. (event: On<CustomEvent>, ... )
6. Event naming convention: CommandXXX or EventXXX. Command is for external invocation to reduce coupling. Event is for external listeners
7. despawn_recursive is deprecated. Use despawn instead, which already despawns recursively
8. get_single is deprecated. Use single() instead
