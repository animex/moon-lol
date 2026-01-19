## Systems

- Movement System
  Doesn't care who issued the movement command, only cares about movement mode and speed, responsible for modifying transform component per frame.
- Attack System
  Doesn't care about distance to target, only cares about who to attack, responsible for updating attack state (windup, cooldown).
- Auto Attack System
  Queries distance between attacker and target, if not in attack range issues pathfinding movement command to navigation system, if in attack range issues attack command and movement stop command.
- Ability System
  When receiving an ability with displacement effect, commands movement system to move in the direction indicated by the ability.
- Controller System
  Registers hotkeys for abilities in the entity's ability array, input triggers corresponding ability cast, including movement and stop.

When auto attack command, movement command, and displacement command compete for final movement system control, e.g. when auto attack is moving toward target and receives a movement command needing to stop attack and switch to movement, when displacement ability is displacing both attack and movement commands are ignored. Traditional imperative design requires movement command to manually stop auto attack behavior, movement command needs to check if in displacement state. As more commands compete for movement control, each system needs to know about other systems. How can we use declarative design to solve this problem?

## Movement Control Contest Problem

Run command, displacement command

1. Imperative

When Run command triggers, command movement system to move once. When displacement command triggers, command movement system to rapidly displace along path.

Problem: When commands are concurrent, new command overwrites old command's movement, no priority.

2. Priority-based

Movement system saves priority of last movement command, compares with current movement command's priority, decides whether to perform new movement.

Advantage: High priority commands won't be overwritten by low priority commands.
Problem: Run command first, then displacement command, after displacement ends, cannot continue previous Run command.

3. Priority + Continuous Command

Save priority of last movement command, compare with current movement command's priority, decide whether to perform new movement. Run command system adds Run component, Run system continuously issues low priority movement commands.

Advantage: When high priority movement command ends, immediately resumes low priority movement command.
Problem: Need to remove Run component

## When to Remove Run Component

Removed when reaching Run destination

How to determine if destination reached

1. In Run system detect distance to destination, if less than movement speed * dt distance then consider arrived at destination.

Problem: Movement system already has destination arrival detection, would be redundant

2. Use movement system's EventMovementEnd event, remove Run component.

Problem: Movement end from displacement should not remove Run component.

3. EventMovementEnd event adds a type field, when type is Run, remove Run component

Problem: Need to maintain a MovementType enum containing all types, every new movement type requires a new variant.

## Pathfinding First or Movement Decision First

If pathfinding first, pathfinding takes time, if after that time decide not to move then wasted, so decide whether to move first, then pathfind

## Auto Attack System and Movement

Auto Attack System = Run System + Attack Loop System

When outside attack range: Add Run component, command attack system to stop attacking once
When inside attack range: Remove Run component, command attack system to attack once

Problem: After removing Run component, movement system may still be moving.

1. Run component removal issues movement stop command.

Advantage: Solves the problem of continuing to move after Run component is removed
Problem: When removing Run component, if currently in displacement movement state, displacement will be interrupted by Run component removal.

2. Run component removal issues movement stop command, movement stop command's priority must compare with last movement command's priority to decide whether to stop movement.

Advantage: Solves the problem of high priority movement being interrupted by low priority stop movement command

## Original Logic

1. Within attack range

Regardless of whether attack is ready, no movement is performed. When attack is ready, start attack immediately. When attack is not ready, wait for attack to be ready then start attack.

2. Outside attack range

Will move toward target, after entering attack range:

- When attack is ready

Stop moving, start attacking target.

- When attack is not ready

Continue moving until enemy's bounding is completely within attack range then stop moving, wait for attack ready, once attack is ready start attack.

## Auto Attack Implementation

Adjust attack system, when receiving attack command check if within attack range, if not within attack range don't start attack.

Issue attack command every frame.

Add Run component, issue move to target command every frame.

When receiving attack start event, remove Run component, i.e. stop moving.

Advantage: Constantly initiate attack, once attack successfully initiated, stop moving.
Problem: When attack not ready will keep moving toward target.

Every frame check if in attack windup, if in attack windup don't perform attack range detection and subsequent logic.

Every frame check if within attack range:

- If within attack range, try to remove Run component, ensure no movement, and issue attack command every frame.
- If not within attack range, add Run component, issue move to target command every frame.

Advantages:

- Constantly initiate attack, when within attack range regardless of whether attack is ready won't move, when outside attack range will move toward target, when moving into attack range will stop moving and attempt attack.
- During attack windup, when enemy leaves attack range, won't have the problem of moving toward enemy while performing attack.
  Problem: When attack not ready and entering attack range will stop moving, but original version continues moving when attack not ready, until enemy's bounding is completely within attack range before stopping.

## Auto Attack System and Movement System Interaction

AutoAttack FixedUpdate
├── Attack FixedUpdate
└── Run FixedUpdate
└── Movement PostUpdate

Run issues CommandMovementStart command in FixedUpdate
When within attack range, AutoAttack issues CommandRunStop command in FixedUpdate, CommandRunStop listener removes Run component and issues CommandMovementStop command
Both Run and AutoAttack are in FixedUpdate, so Run issues CommandMovementStart command first, then AutoAttack issues CommandRunStop command, causing Run removal lag.
When Movement receives both CommandMovementStart command and CommandMovementStop command in one frame, since CommandMovementStart needs to be collected to RequestBuffer first then Reduce and Apply, CommandMovementStop also needs to clean up CommandMovementStart commands in RequestBuffer.
Even if RequestBuffer is cleaned, if CommandMovementStart executes after CommandMovementStop, Stop then Start, movement still cannot be stopped.

- Is there a problem with Run issuing CommandMovementStart command every frame?
  Run needs to continuously issue low priority movement commands to ensure low priority Run executes immediately when high priority movement ends

- Is there a problem with AutoAttack issuing CommandRunStop command every frame when within attack range?
  AutoAttack issues movement stop command every frame to ensure that when within attack range, movement is stopped regardless of whether attack is ready.

1. Adjust execution order of AutoAttack and Run, ensure AutoAttack executes before Run.

Run is a more fundamental system, shouldn't adjust its execution order because of upper layer systems, so can only adjust AutoAttack's execution stage to FixedPreUpdate.

Advantage: Run component is removed before issuing movement command.
Problem: Within one frame, when CommandMovementStop appears after CommandMovementStart, movement will still start.

## Movement Command Problem

1. Put Movement start and stop commands into RequestBuffer, during Reduce stage when stop command exists in RequestBuffer, only use stop movement as final decision.

Problem: Within same frame, when Stop received first then Start, will blindly Stop

2. When receiving CommandMovementStop, clean up CommandMovementStart commands in RequestBuffer.

Advantage: Ensures within same frame, when Start received first then Stop, movement won't start.

3. Merge CommandMovementStart and CommandMovementStop into CommandMovement command, take the one with highest priority as final decision.

[Start-0, Stop-0]
Take Stop-0 as final decision

[Start-0, Stop-1]
Take Stop-1 as final decision

[Start-1, Stop-0]
Take Start-1 as final decision

[Start-1, Stop-1]
Take Stop-1 as final decision

[Start-1, Stop-0, Start-2]
Take Start-2 as final decision

[Start-1, Stop-2, Start-2]
Take Start-2 as final decision

[Start-2, Stop-1, Start-1]
Take Start-2 as final decision

[Start-1, Stop-2, Start-1]
Take Stop-2 as final decision

Problems:

- Doesn't consider last final decision, e.g. if last final decision was Start-1, this time receives Start-0, should ignore
- [Start-1, Stop-2, Start-1] should actually take Start-1 as final decision

4. Merge CommandMovementStart and CommandMovementStop into CommandMovement command, buffer adds last decision at the beginning, simulate execution in order, high commands overwrite low commands, except start always overwrites high stop regardless of how low.

[Start-1, Stop-2, Start-1]
Take Start-1 as final decision

# Damage and Life

Problem 1:
