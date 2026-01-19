# Priority

- [ ] Riven Q ability

- [ ] Riven W ability

- [ ] Riven E ability

- [ ] Riven R ability

# Game Experiments

- [x] Does resetting auto attack within 2 frames cancel the first attack
      Yes, there's no mechanism that prevents canceling within 2 frames, it's just network latency

- [x] Does A then immediately flashing backwards cancel the attack
      No, after A-flash the attack will still go through as long as you don't interrupt it, so don't move after flashing

- [x] Attack animation needs to adjust based on attack speed, animation duration equals the attack component's total_duration time. Other animations may also need duration adjustments, consider a more elegant implementation

- [ ] After attack backswing, does A-flash trigger the next attack

# Laning

- [x] Turret aggro mechanism

- [x] Minion death drops experience

- [x] Champion panel

- [x] Attack range detection

- [x] Missile system

- [x] Mana

- [x] Armor reduces physical damage

- [ ] Attribute increase on level up

- [ ] Flash

- [ ] Ignite

# Fiora

- [x] Trigger different position particle effects based on Fiora vital direction

- [x] Understand Fiora vital lifecycle

warning active timeout_red

- [x] Particle emission direction

# Particle System

- [x] In bevy, shaders have different variants, how to pass CPU data to GPU, should we create an AsBindGroup struct for each variant?

mat -> key -> def

- [x] How to pass different data combinations for different defs

- [x] Particle system supports birthRotation0

- [x] Particle system supports blend mode

- [x] Fix particle jitter when character moves

- [x] Support single particle attribute

- [x] Support birth scale

- [x] Support velocity

- [ ] Understand particle system pass attribute usage

- [x] Particle system supports setting color vertex attribute

- [x] Understand particle system texture mult sub-attribute usage

- [x] Understand particle system direction

- [ ] Hit particle effect

# Ability System

- [x] How to implement multi-ability, hot-swappable ability system

A champion has multiple ability entities, ability components are attached to ability entities

- [x] How to trigger corresponding ability when receiving ability input

- [x] Ability cooldown

- [x] Ability point allocation mechanism

- [x] Reduce mana

- [ ] Get ability data from ability resources

- [ ] Hotkey ability point allocation

# Movement System

- [x] Move at specified rate

# Pathfinding System

- [x] Minion collision volume too large issue

- [ ] Minion spacing too small issue

# UI System

- [x] UI system should display damage numbers after Health component receives damage event, numbers should float up then fall down from entity position, starting large then gradually shrinking

- [x] UI scaling issue

- [x] Experience level UI

- [x] Health UI

- [x] Mana UI

- [x] Turret health bar

- [x] Ability icons

- [x] Champion icon

- [x] Point allocation button

- [x] Ability upgrade animation

- [ ] Attribute panel UI

- [ ] Ability cooldown timer UI

- [ ] Fix floating damage numbers

- [ ] Level text UI

- [ ] Health bar text UI

- [ ] Refactor health bar with UIElement

- [ ] Ability level UI

- [ ] Cursor

- [ ] Movement animation
