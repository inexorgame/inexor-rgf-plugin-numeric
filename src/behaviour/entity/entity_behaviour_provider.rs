use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::gate::NumericGate;
use crate::behaviour::entity::gate::NUMERIC_GATES;
use crate::behaviour::entity::operation::NumericOperation;
use crate::behaviour::entity::operation::NUMERIC_OPERATIONS;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct NumericOperationStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<NumericOperation<'static>>>>,
);

#[wrapper]
pub struct NumericGateStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<NumericGate<'static>>>>,
);

#[waiter_di::provides]
fn create_numeric_operation_storage() -> NumericOperationStorage {
    NumericOperationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_numeric_gate_storage() -> NumericGateStorage {
    NumericGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait NumericEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct NumericEntityBehaviourProviderImpl {
    numeric_operations: NumericOperationStorage,
    numeric_gates: NumericGateStorage,
}

interfaces!(NumericEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl NumericEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            numeric_operations: create_numeric_operation_storage(),
            numeric_gates: create_numeric_gate_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl NumericEntityBehaviourProvider for NumericEntityBehaviourProviderImpl {
    fn create_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = NUMERIC_OPERATIONS.get(entity_instance.type_name.as_str());
        let numeric_operation = match function {
            Some(function) => Some(Arc::new(NumericOperation::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if numeric_operation.is_some() {
            // let numeric_operation = Arc::new(numeric_operation.unwrap());
            self.numeric_operations
                .0
                .write()
                .unwrap()
                .insert(id, numeric_operation.unwrap());
            debug!(
                "Added behaviour numeric_operation to entity instance {}",
                id
            );
        }
    }

    fn create_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = NUMERIC_GATES.get(entity_instance.type_name.as_str());
        let numeric_gate = match function {
            Some(function) => Some(Arc::new(NumericGate::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if numeric_gate.is_some() {
            // let numeric_operation = Arc::new(numeric_operation.unwrap());
            self.numeric_gates
                .0
                .write()
                .unwrap()
                .insert(id, numeric_gate.unwrap());
            debug!("Added behaviour numeric_gate to entity instance {}", id);
        }
    }

    fn remove_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.numeric_operations
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour numeric_operation from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.numeric_gates
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour numeric_gates from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.numeric_operations.0.write().unwrap().contains_key(&id) {
            self.numeric_operations.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour numeric_operation from entity instance {}",
                id
            );
        }
        if self.numeric_gates.0.write().unwrap().contains_key(&id) {
            self.numeric_gates.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour numeric_gates from entity instance {}",
                id
            );
        }
    }
}

impl EntityBehaviourProvider for NumericEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_numeric_operation(entity_instance.clone());
        self.create_numeric_gate(entity_instance.clone());
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_numeric_operation(entity_instance.clone());
        self.remove_numeric_gate(entity_instance.clone());
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}