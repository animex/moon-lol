
# Requirements

- Particle emitter and particle lifetime is managed uniformly by lifetime component, entities are despawned immediately when lifetime ends

- When particle emitter lifetime ends, particles may still need to be alive

- When champion lifetime ends, all particle emitters and particles need to be despawned

- Particle's transform should be under particle emitter's transform, and particle emitter's transform should be under champion

# Solution

## Hierarchy Structure

entity
└── emitter ParticleId
    ├── particle1
    └── particle2 ParticleDecal(Hash<Entity, Entity>) MeshMaterial3d<ParticleMaterialUnlitDecal>

map
└── geometry MapGeometry Mesh3d

decal_geometry Mesh3d MeshMaterial3d<ParticleMaterialUnlitDecal>

- Lifetime supports a mode where entities are not despawned when lifetime ends, only despawned when there are no child entities. Emitter should use this mode

- Despawn specified emitter through ParticleId, child particles are automatically despawned

- When emitter's is_local_orientation is true, emitter manually updates its own GlobalTransform, particle uses parent entity (emitter's manually updated GlobalTransform) to calculate world matrix passed to shader

- Iterate over (Entity, ParticleDecal) entities, get ParticleDecalGeometry
