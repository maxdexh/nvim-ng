use std::{
    any::{Any, TypeId, type_name},
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, RwLock},
};

#[derive(Clone)]
struct RegisteredEntry {
    type_name: &'static str,
    value: Arc<dyn Any + Send + Sync>,
}
impl RegisteredEntry {
    fn get_val<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.value.clone().downcast().unwrap_or_else(|_| {
            unreachable!("expected {:?}, got {:?}", type_name::<T>(), self.type_name)
        })
    }
    fn new<T: 'static + Send + Sync>(val: T) -> Self {
        Self {
            type_name: type_name::<T>(),
            value: Arc::new(val),
        }
    }
}
impl std::fmt::Debug for RegisteredEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredEntry")
            .field("type_name", &self.type_name)
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Registry {
    inner: Arc<RwLock<HashMap<TypeId, RegisteredEntry>>>,
}
impl Registry {
    pub fn try_register<T: 'static + Send + Sync>(&self, entry: T) -> Result<(), T> {
        let mut inner = self.inner.write().unwrap_or_else(|pe| pe.into_inner());
        match inner.entry(TypeId::of::<T>()) {
            Entry::Occupied(_) => Err(entry),
            Entry::Vacant(vac) => {
                vac.insert(RegisteredEntry::new(entry));
                Ok(())
            }
        }
    }
    #[allow(dead_code)]
    pub fn register<T: 'static + Send + Sync>(&self, entry: T) {
        self.try_register(entry)
            .unwrap_or_else(|_| panic!("double registration of entry type {:?}", type_name::<T>()))
    }

    pub fn try_get<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
        self.inner
            .read()
            .unwrap_or_else(|pe| pe.into_inner())
            .get(&TypeId::of::<T>())
            .map(|entry| entry.get_val())
    }
    #[allow(dead_code)]
    pub fn get<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.try_get::<T>()
            .unwrap_or_else(|| panic!("missing registration for entry type {:?}", type_name::<T>()))
    }
    pub fn get_or_insert<T: 'static + Send + Sync + Default>(
        &self,
        f: impl FnOnce() -> T,
    ) -> Arc<T> {
        if let Some(entry) = self
            .inner
            .read()
            .unwrap_or_else(|pe| pe.into_inner())
            .get(&TypeId::of::<T>())
        {
            return entry.get_val();
        }

        std::hint::cold_path();

        match self
            .inner
            .write()
            .unwrap_or_else(|pe| pe.into_inner())
            .entry(TypeId::of::<T>())
        {
            Entry::Occupied(occ) => occ.get().get_val(),
            Entry::Vacant(vac) => vac.insert(RegisteredEntry::new(f())).get_val(),
        }
    }
    pub fn get_or_default<T: 'static + Send + Sync + Default>(&self) -> Arc<T> {
        self.get_or_insert(Default::default)
    }
}
