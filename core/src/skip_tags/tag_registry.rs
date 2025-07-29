//! Skip Tag Registry - Unified Implementation
//!
//! Central registry for all skip tags with thread-safe access and comprehensive tag support.
//! Combines the robust thread-safety of the main branch with shop enhancement functionality.

use super::{SkipTag, SkipTagId, TagRarity};
use super::shop_tags::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe registry for all skip tags - unified approach
#[derive(Debug)]
pub struct SkipTagRegistry {
    tags: RwLock<HashMap<SkipTagId, Arc<dyn SkipTag>>>,
}

impl SkipTagRegistry {
    /// Create a new registry with all available tags registered
    pub fn new() -> Self {
        let registry = Self {
            tags: RwLock::new(HashMap::new()),
        };

        // Register all available tag implementations
        registry.register_all_tags();
        registry
    }

    /// Register all available skip tags
    fn register_all_tags(&self) {
        // Register shop enhancement tags
        self.register_shop_enhancement_tags();
        
        // Register utility tags if available
        self.register_utility_tags();
    }

    /// Register all shop enhancement tags
    fn register_shop_enhancement_tags(&self) {
        let _ = self.register_tag(VoucherTag);
        let _ = self.register_tag(CouponTag);
        let _ = self.register_tag(D6Tag);
        let _ = self.register_tag(FoilTag);
        let _ = self.register_tag(HolographicTag);
        let _ = self.register_tag(PolychromeTag);
    }

    /// Register utility tags if the module is available
    fn register_utility_tags(&self) {
        // This will be populated when utility tags are available from main branch
        #[cfg(feature = "utility_tags")]
        {
            use super::utility_tags::*;
            let _ = self.register_tag(DoubleTag);
            let _ = self.register_tag(BossTag);
            let _ = self.register_tag(OrbitalTag);
            let _ = self.register_tag(JuggleTag);
        }
    }

    /// Register a skip tag (internal helper)
    fn register_tag<T: SkipTag + 'static>(&self, tag: T) -> Result<(), String> {
        let id = tag.tag_id();
        let tag_arc = Arc::new(tag);

        let mut tags = self
            .tags
            .write()
            .map_err(|_| "Failed to acquire write lock on skip tag registry")?;

        if tags.contains_key(&id) {
            return Err(format!("Skip tag {id} is already registered"));
        }

        tags.insert(id, tag_arc);
        Ok(())
    }

    /// Get a skip tag by ID
    pub fn get_tag(&self, id: SkipTagId) -> Option<Arc<dyn SkipTag>> {
        let tags = self.tags.read().ok()?;
        tags.get(&id).cloned()
    }

    /// Get all registered tag IDs
    pub fn get_all_ids(&self) -> Vec<SkipTagId> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.keys().copied().collect()
    }

    /// Get all registered tag IDs (alias for compatibility)
    pub fn get_all_tag_ids(&self) -> Vec<SkipTagId> {
        self.get_all_ids()
    }

    /// Get tags by rarity
    pub fn get_tags_by_rarity(&self, rarity: TagRarity) -> Vec<SkipTagId> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.values()
            .filter(|tag| tag.rarity() == rarity)
            .map(|tag| tag.tag_id())
            .collect()
    }

    /// Get weighted tag selection (for random generation)
    pub fn get_weighted_tags(&self) -> Vec<(SkipTagId, f64)> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.values()
            .map(|tag| (tag.tag_id(), tag.rarity().weight()))
            .collect()
    }

    /// Get all shop enhancement tags
    pub fn get_all_shop_enhancement_tags(&self) -> Vec<Arc<dyn SkipTag>> {
        let shop_tag_ids = [
            SkipTagId::Voucher,
            SkipTagId::Coupon,
            SkipTagId::D6,
            SkipTagId::Foil,
            SkipTagId::Holographic,
            SkipTagId::Polychrome,
        ];

        shop_tag_ids
            .iter()
            .filter_map(|&id| self.get_tag(id))
            .collect()
    }

    /// Check if a tag is registered
    pub fn has_tag(&self, id: SkipTagId) -> bool {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.contains_key(&id)
    }

    /// Check if a tag is registered (alias for compatibility)
    pub fn is_registered(&self, id: SkipTagId) -> bool {
        self.has_tag(id)
    }

    /// Get count of registered tags
    pub fn tag_count(&self) -> usize {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.len()
    }

    /// Get the number of registered tags (alias for compatibility)
    pub fn count(&self) -> usize {
        self.tag_count()
    }
}

impl Default for SkipTagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global registry singleton
static GLOBAL_REGISTRY: std::sync::OnceLock<SkipTagRegistry> = std::sync::OnceLock::new();

/// Get the global skip tag registry
pub fn get_registry() -> &'static SkipTagRegistry {
    GLOBAL_REGISTRY.get_or_init(SkipTagRegistry::new)
}

/// Get the global skip tag registry (alias for compatibility)
pub fn global_registry() -> &'static SkipTagRegistry {
    get_registry()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skip_tags::{TagEffectResult, TagEffectType};

    #[test]
    fn test_registry_creation() {
        let registry = SkipTagRegistry::new();
        
        // Should have all 6 shop enhancement tags by default
        assert_eq!(registry.tag_count(), 6);

        // All shop enhancement tags should be present
        assert!(registry.has_tag(SkipTagId::Voucher));
        assert!(registry.has_tag(SkipTagId::Coupon));
        assert!(registry.has_tag(SkipTagId::D6));
        assert!(registry.has_tag(SkipTagId::Foil));
        assert!(registry.has_tag(SkipTagId::Holographic));
        assert!(registry.has_tag(SkipTagId::Polychrome));
    }

    #[test]
    fn test_get_tag() {
        let registry = SkipTagRegistry::new();

        // Test getting existing tag
        let voucher_tag = registry.get_tag(SkipTagId::Voucher);
        assert!(voucher_tag.is_some());
        assert_eq!(voucher_tag.unwrap().tag_id(), SkipTagId::Voucher);

        // Test getting non-existent tag (not yet implemented)
        let economy_tag = registry.get_tag(SkipTagId::Economy);
        assert!(economy_tag.is_none());
    }

    #[test]
    fn test_get_all_shop_enhancement_tags() {
        let registry = SkipTagRegistry::new();
        let shop_tags = registry.get_all_shop_enhancement_tags();

        assert_eq!(shop_tags.len(), 6);

        // Verify all are shop enhancement tags
        for tag in &shop_tags {
            assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        }

        // Verify specific tags are present
        let tag_ids: Vec<SkipTagId> = shop_tags.iter().map(|tag| tag.tag_id()).collect();
        assert!(tag_ids.contains(&SkipTagId::Voucher));
        assert!(tag_ids.contains(&SkipTagId::Coupon));
        assert!(tag_ids.contains(&SkipTagId::D6));
        assert!(tag_ids.contains(&SkipTagId::Foil));
        assert!(tag_ids.contains(&SkipTagId::Holographic));
        assert!(tag_ids.contains(&SkipTagId::Polychrome));
    }

    #[test]
    fn test_has_tag() {
        let registry = SkipTagRegistry::new();

        // Should have shop enhancement tags
        assert!(registry.has_tag(SkipTagId::Voucher));
        assert!(registry.has_tag(SkipTagId::Coupon));

        // Should not have unimplemented tags
        assert!(!registry.has_tag(SkipTagId::Economy));
        assert!(!registry.has_tag(SkipTagId::Investment));
    }

    #[test]
    fn test_get_all_tag_ids() {
        let registry = SkipTagRegistry::new();
        let all_ids = registry.get_all_tag_ids();

        assert_eq!(all_ids.len(), 6);

        // Should contain all shop enhancement tag IDs
        assert!(all_ids.contains(&SkipTagId::Voucher));
        assert!(all_ids.contains(&SkipTagId::Coupon));
        assert!(all_ids.contains(&SkipTagId::D6));
        assert!(all_ids.contains(&SkipTagId::Foil));
        assert!(all_ids.contains(&SkipTagId::Holographic));
        assert!(all_ids.contains(&SkipTagId::Polychrome));
    }

    #[test]
    fn test_default_equals_new() {
        let registry_new = SkipTagRegistry::new();
        let registry_default = SkipTagRegistry::default();

        assert_eq!(registry_new.tag_count(), registry_default.tag_count());

        // Verify both have the same tags
        for tag_id in registry_new.get_all_tag_ids() {
            assert!(registry_default.has_tag(tag_id));
        }
    }

    #[test]
    fn test_global_registry_singleton() {
        let registry1 = get_registry();
        let registry2 = get_registry();

        // Should be the same instance
        assert!(std::ptr::eq(registry1, registry2));

        // Should have all shop enhancement tags
        assert_eq!(registry1.tag_count(), 6);
    }

    #[test]
    fn test_compatibility_aliases() {
        let registry = SkipTagRegistry::new();
        
        // Test that aliases work correctly
        assert_eq!(registry.count(), registry.tag_count());
        assert_eq!(registry.is_registered(SkipTagId::Voucher), registry.has_tag(SkipTagId::Voucher));
        assert_eq!(registry.get_all_ids(), registry.get_all_tag_ids());
    }

    #[test]
    fn test_rarity_and_weighted_tags() {
        let registry = SkipTagRegistry::new();
        
        // Test rarity filtering
        let common_tags = registry.get_tags_by_rarity(TagRarity::Common);
        assert!(!common_tags.is_empty());
        
        // Test weighted tags
        let weighted = registry.get_weighted_tags();
        assert_eq!(weighted.len(), registry.tag_count());
        
        // All weights should be positive
        for (_, weight) in weighted {
            assert!(weight > 0.0);
        }
    }

    #[test]
    fn test_tag_names_match_ids() {
        let registry = SkipTagRegistry::new();

        // Test that tag names match their IDs
        assert_eq!(registry.get_tag(SkipTagId::Voucher).unwrap().name(), "Voucher");
        assert_eq!(registry.get_tag(SkipTagId::Coupon).unwrap().name(), "Coupon");
        assert_eq!(registry.get_tag(SkipTagId::D6).unwrap().name(), "D6");
        assert_eq!(registry.get_tag(SkipTagId::Foil).unwrap().name(), "Foil");
        assert_eq!(registry.get_tag(SkipTagId::Holographic).unwrap().name(), "Holographic");
        assert_eq!(registry.get_tag(SkipTagId::Polychrome).unwrap().name(), "Polychrome");
    }
}
