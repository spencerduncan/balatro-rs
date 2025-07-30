Implementing SKIP-001: Skip Tag Trait Definition and Registry System

This is the foundational infrastructure for the entire skip tag system (issue #228).

Key Requirements:
1. Define SkipTag trait with all required methods
2. Create TagId enum with all 26 skip tags
3. Implement TagEffectType classification
4. Build tag registry and factory system
5. Add comprehensive error handling
6. Ensure performance: Tag lookup <1μs

This task BLOCKS all other skip tag work and must be completed first.

Reference: GitHub issue #689 and architecture spec in /home/spduncan/balatro-rs-ws/feature-228-skip-tags/SKIP_TAG_ARCHITECTURE.md
