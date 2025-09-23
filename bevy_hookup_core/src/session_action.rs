use serde::{Deserialize, Serialize};

use crate::{external_component::ExternalComponent, sync_entity_id::SyncEntityId};

#[derive(Clone, Serialize, Deserialize)]
pub enum SessionAction<TSendables> {
    AddEntity {
        id: SyncEntityId,
    },
    RemoveEntity {
        id: SyncEntityId,
    },
    AddComponent {
        component_data: TSendables,
        external_component: ExternalComponent,
    },
    UpdateComponent {
        component_data: TSendables,
        external_component: ExternalComponent,
    },
    UpdateSharedComponent {
        component_data: TSendables,
        external_component: ExternalComponent,
    },
    RemoveComponent {
        external_component: ExternalComponent,
    },
}

impl<TSendables> SessionAction<TSendables> {
    pub fn to_counterpart(self) -> Self {
        match self {
            Self::AddEntity { id } => Self::AddEntity {
                id: id.counterpart(),
            },
            Self::RemoveEntity { id } => Self::RemoveEntity {
                id: id.counterpart(),
            },
            Self::AddComponent {
                component_data,
                external_component,
            } => Self::AddComponent {
                component_data,
                external_component: external_component.counterpart(),
            },
            Self::UpdateComponent {
                component_data,
                external_component,
            } => Self::UpdateComponent {
                component_data,
                external_component: external_component.counterpart(),
            },
            Self::UpdateSharedComponent {
                component_data,
                external_component,
            } => Self::UpdateSharedComponent {
                component_data,
                external_component: external_component.counterpart(),
            },
            Self::RemoveComponent { external_component } => Self::RemoveComponent {
                external_component: external_component.counterpart(),
            },
        }
    }
}
