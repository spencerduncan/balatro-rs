use crate::game::Game;
use crate::skip_tags::tag_error::{TagError, TagErrorKind};
use crate::skip_tags::tag_trait::{SkipTag, TagId};
use crate::skip_tags::tags::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe registry for all skip tags
/// 
/// The registry provides centralized access to all skip tag implementations
/// and ensures thread-safety for concurrent game instances.
pub struct TagRegistry {
    /// Map of tag ID to tag implementation
    tags: RwLock<HashMap<TagId, Arc<dyn SkipTag>>>,
}

impl TagRegistry {
    /// Create a new empty tag registry
    pub fn new() -> Self {
        Self {
            tags: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a tag implementation
    pub fn register(&self, tag: Arc<dyn SkipTag>) -> Result<(), TagError> {
        let mut tags = self.tags.write().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire write lock")
        })?;
        
        let tag_id = tag.id();
        if tags.contains_key(&tag_id) {
            return Err(TagError::new(
                TagErrorKind::Validation, 
                format!("Tag {:?} is already registered", tag_id)
            ));
        }
        
        tags.insert(tag_id, tag);
        Ok(())
    }
    
    /// Get a tag by ID
    pub fn get_tag(&self, tag_id: TagId) -> Result<Arc<dyn SkipTag>, TagError> {
        let tags = self.tags.read().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire read lock")
        })?;
        
        tags.get(&tag_id)
            .cloned()
            .ok_or_else(|| TagError::new(TagErrorKind::NotFound, format!("Tag {:?} not found", tag_id)))
    }
    
    /// Get all available tags that can be applied to the current game state
    pub fn get_available_tags(&self, game_state: &Game) -> Result<Vec<Arc<dyn SkipTag>>, TagError> {
        let tags = self.tags.read().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire read lock")
        })?;
        
        let available_tags = tags
            .values()
            .filter(|tag| tag.can_apply(game_state) && tag.availability_condition(game_state))
            .cloned()
            .collect();
            
        Ok(available_tags)
    }
    
    /// Get all registered tag IDs
    pub fn get_all_tag_ids(&self) -> Result<Vec<TagId>, TagError> {
        let tags = self.tags.read().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire read lock")
        })?;
        
        Ok(tags.keys().copied().collect())
    }
    
    /// Check if a tag is registered
    pub fn contains_tag(&self, tag_id: TagId) -> Result<bool, TagError> {
        let tags = self.tags.read().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire read lock")
        })?;
        
        Ok(tags.contains_key(&tag_id))
    }
    
    /// Get the number of registered tags
    pub fn tag_count(&self) -> Result<usize, TagError> {
        let tags = self.tags.read().map_err(|_| {
            TagError::new(TagErrorKind::Internal, "Failed to acquire read lock")
        })?;
        
        Ok(tags.len())
    }
    
    /// Initialize the registry with all default tag implementations
    /// 
    /// This method populates the registry with stub implementations for all tags
    /// to enable development and testing of the tag selection system.
    pub fn initialize_with_stubs(&self) -> Result<(), TagError> {
        // For now, use stub implementations for all tags
        // These will be replaced with actual implementations in Phase 2
        
        for &tag_id in TagId::all() {
            let stub_tag = create_stub_tag(tag_id);
            self.register(Arc::new(stub_tag))?;
        }
        
        Ok(())
    }
}

impl Default for TagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global tag registry instance
/// 
/// This provides a singleton pattern for accessing tags across the application.
/// The registry is initialized once and reused for all game instances.
pub static SKIP_TAG_REGISTRY: std::sync::OnceLock<TagRegistry> = std::sync::OnceLock::new();

/// Initialize the global skip tag registry
pub fn initialize_global_registry() -> Result<(), TagError> {
    let registry = TagRegistry::new();
    registry.initialize_with_stubs()?;
    
    SKIP_TAG_REGISTRY.set(registry).map_err(|_| {
        TagError::new(TagErrorKind::Internal, "Global registry already initialized")
    })?;
    
    Ok(())
}

/// Get the global skip tag registry
pub fn get_global_registry() -> Result<&'static TagRegistry, TagError> {
    SKIP_TAG_REGISTRY.get().ok_or_else(|| {
        TagError::new(TagErrorKind::Internal, "Global registry not initialized")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skip_tags::tag_trait::{StubTag, TagEffectType};
    
    #[test]
    fn test_registry_registration() {
        let registry = TagRegistry::new();
        let stub_tag = Arc::new(StubTag::new(
            TagId::Investment,
            "Investment",
            TagEffectType::ImmediateReward,
            "Test stub"
        ));
        
        assert!(registry.register(stub_tag).is_ok());
        assert!(registry.contains_tag(TagId::Investment).unwrap());
        assert_eq!(registry.tag_count().unwrap(), 1);
    }
    
    #[test]
    fn test_registry_duplicate_registration() {
        let registry = TagRegistry::new();
        let stub_tag1 = Arc::new(StubTag::new(
            TagId::Investment,
            "Investment",
            TagEffectType::ImmediateReward,
            "Test stub 1"
        ));
        let stub_tag2 = Arc::new(StubTag::new(
            TagId::Investment,
            "Investment",
            TagEffectType::ImmediateReward,
            "Test stub 2"
        ));
        
        assert!(registry.register(stub_tag1).is_ok());
        assert!(registry.register(stub_tag2).is_err());
    }
    
    #[test]
    fn test_registry_get_tag() {
        let registry = TagRegistry::new();
        let stub_tag = Arc::new(StubTag::new(
            TagId::Investment,
            "Investment",
            TagEffectType::ImmediateReward,
            "Test stub"
        ));
        
        registry.register(stub_tag).unwrap();
        let retrieved_tag = registry.get_tag(TagId::Investment).unwrap();
        assert_eq!(retrieved_tag.id(), TagId::Investment);
    }
    
    #[test]
    fn test_registry_tag_not_found() {
        let registry = TagRegistry::new();
        let result = registry.get_tag(TagId::Investment);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().kind, TagErrorKind::NotFound));
    }
}