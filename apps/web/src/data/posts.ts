export interface Post {
  title: string;
  desc: string;
  date: string;
  tag: string;
  content: string;
}

export const posts: Record<string, Post> = {
  architecture: {
    title: "Architecture",
    desc: "High-level system design of Moon LoL: Rust Core and Web Frontend.",
    date: "2025.11.28",
    tag: "ARCHITECTURE",
    content: `
# Architecture

\`\`\`mermaid
graph TD
    subgraph "Rust Core (Bevy)"
        App[Bevy App]
        ECS[ECS World]
        Plugins[Core Plugins]
    end

    subgraph "Frontend (Vue)"
        Web[Vue App]
        Render[Renderer]
    end

    App -->|Update| ECS
    ECS -->|State/Events| Web
    Web -->|Input| ECS
\`\`\`

## Core Design Philosophy

Moon LoL adopts a high-performance layered architecture. The core of the system is based on the Rust Bevy engine, which provides ultimate performance and a type-safe ECS architecture.

## Core Pillars

- **Rust Core (Bevy):** Handles all game logic, physics simulation, and state management. Uses ECS pattern to achieve high concurrency and memory-friendly data processing.
- **Frontend (Vue 3):** Fetches game state via WebSocket or HTTP polling, uses WebGL/Canvas for real-time rendering, mainly for debugging and observation.

## High-Performance Communication

The frontend and backend communicate through efficient data protocols, ensuring smooth frame rates even when rendering large numbers of entities.
    `,
  },
  "data-flow": {
    title: "Data Flow",
    desc: "Data pipeline from Bevy ECS to Web Frontend.",
    date: "2025.11.28",
    tag: "DATA",
    content: `
# Data Flow

\`\`\`mermaid
sequenceDiagram
    participant Bevy as Bevy System
    participant Frontend as Vue Client

    loop Game Loop
        Bevy->>Bevy: Run Systems (Movement, Attack, etc.)
        Bevy->>Bevy: Update State
    end

    loop Render Loop
        Frontend->>Bevy: Request State
        Bevy-->>Frontend: JSON Snapshot
        Frontend->>Frontend: Render Frame
    end
\`\`\`

## Game Loop

The system's data flow is entirely driven by Bevy's Schedule.

## Core Loop (Update)

The Bevy engine executes the Update Schedule once per frame:

1. Process input events.
2. Run all Systems (movement, attack, damage calculation).
3. Update component states.

## Render Loop

The frontend Vue application doesn't control game logic; it's just an observer. It periodically pulls the latest game state snapshots (usually in JSON format) via API, then updates the DOM or Canvas. This decoupled design allows the backend to run at maximum speed while the frontend only handles visualization.
    `,
  },
  ecs: {
    title: "ECS Components and Systems",
    desc: "Deep dive into game core logic: plugin system and entity-component design.",
    date: "2025.11.28",
    tag: "CORE",
    content: `
# ECS Components and Systems

## Everything is an Entity

In Moon LoL, whether it's a champion, minion, turret, or a flying skill projectile, they are all essentially Entities in the ECS world. Their behavioral differences come solely from the different Component sets they have attached.

## Core Components

- **Health:** Stores current health and maximum health. When health reaches zero, death logic is triggered.
- **Controller:** Marks that the entity is controlled externally (such as by an RL Agent or player input).
- **Transform:** Bevy built-in component that defines the entity's position, rotation, and scale in 3D space.
- **Skill:** Manages skill cooldowns, levels, and casting states.

## System Plugins

We modularize functionality into Bevy Plugins. Each Plugin registers related Systems and Resources. Here are all the core plugins currently registered in the system:

- PluginFioraPassive
- PluginFioraE
- PluginFioraR
- PluginBarrack
- PluginChampion
- PluginCharacter
- PluginDebugSphere
- PluginFiora
- PluginHwei
- PluginMinion
- PluginTurret
- PluginAction
- PluginAnimation
- PluginAttack
- PluginAttackAuto
- PluginAggro
- PluginBase
- PluginCamera
- PluginController
- PluginDamage
- PluginGame
- PluginLife
- PluginLifetime
- PluginMap
- PluginMissile
- PluginMovement
- PluginNavigaton
- PluginParticle
- PluginResource
- PluginRotate
- PluginRun
- PluginSkill
- PluginSkin
- PluginState
- PluginUI
    `,
  },
};
