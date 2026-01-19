#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use moon_lol::*;

    // ===== Test Constants =====
    const TEST_FPS: f32 = 30.0;
    const EPSILON: f32 = 1e-6;

    // ===== Test Utilities =====

    /// Test Harness - Encapsulates common test setup
    struct TestHarness {
        app: App,
        attacker: Entity,
        target: Entity,
    }

    impl TestHarness {
        /// Create a new test harness
        fn new() -> Self {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            app.add_plugins(PluginAttack);
            app.insert_resource(Time::<Fixed>::from_hz(TEST_FPS as f64));

            let target = app.world_mut().spawn_empty().id();
            let attacker = app.world_mut().spawn_empty().id();

            TestHarness {
                app,
                attacker,
                target,
            }
        }

        /// Configure attacker using builder pattern
        fn with_attacker(mut self, attack_component: Attack) -> Self {
            self.app
                .world_mut()
                .entity_mut(self.attacker)
                .insert(attack_component);
            self
        }

        /// Create an additional target entity
        fn spawn_target(&mut self) -> Entity {
            self.app.world_mut().spawn_empty().id()
        }

        // ===== Action Methods - Returns &mut Self =====

        /// Advance time
        fn advance_time(&mut self, seconds: f32) -> &mut Self {
            let ticks = (seconds * TEST_FPS).ceil() as u32;
            for _ in 0..ticks {
                let mut time = self.app.world_mut().resource_mut::<Time<Fixed>>();
                time.advance_by(std::time::Duration::from_secs_f64(1.0 / TEST_FPS as f64));
                drop(time);
                self.app.world_mut().run_schedule(FixedUpdate);
            }
            self
        }

        /// Send attack command
        fn attack(&mut self) -> &mut Self {
            self.app.world_mut().trigger(CommandAttackStart {
                entity: self.attacker,
                target: self.target,
            });
            self.app.update();
            self
        }

        /// Send cancel command
        fn cancel(&mut self) -> &mut Self {
            self.app.world_mut().trigger(CommandAttackStop {
                entity: self.attacker,
            });
            self.app.update();
            self
        }

        /// Send reset command
        fn reset(&mut self) -> &mut Self {
            self.app.world_mut().trigger(CommandAttackReset {
                entity: self.attacker,
            });
            self.app.update();
            self
        }

        /// Switch attacker's target
        fn switch_target(&mut self, new_target: Entity) -> &mut Self {
            self.target = new_target;
            self
        }

        /// Modify attacker component properties
        fn modify_attacker<F>(&mut self, modifier: F) -> &mut Self
        where
            F: FnOnce(&mut Attack),
        {
            let mut attack = self
                .app
                .world_mut()
                .get_mut::<Attack>(self.attacker)
                .unwrap();
            modifier(&mut attack);
            self
        }

        /// Get attack state
        fn attack_state(&self) -> Option<&AttackState> {
            self.app.world().get::<AttackState>(self.attacker)
        }

        /// Get attack component
        fn attack_component(&self) -> &Attack {
            self.app.world().get::<Attack>(self.attacker).unwrap()
        }

        // ===== Assertion Methods - Returns &mut Self =====

        /// Assert attack state is idle
        fn then_expect_idle(&mut self, message: &str) -> &mut Self {
            let state = self.attack_state();
            assert!(state.is_none(), "In attack state: {}", message);
            self
        }

        /// Assert attack state is windup
        fn then_expect_windup(&mut self, message: &str) -> &mut Self {
            let state = self.attack_state();
            assert!(state.is_some(), "Not in attack state: {}", message);
            let state = state.unwrap();
            assert!(
                state.is_windup(),
                "{} (expected Windup, found {:?})",
                message,
                state.status
            );
            self
        }

        /// Assert attack state is cooldown
        fn then_expect_cooldown(&mut self, message: &str) -> &mut Self {
            let state = self.attack_state();
            assert!(state.is_some(), "Not in attack state: {}", message);
            let state = state.unwrap();
            assert!(
                state.is_cooldown(),
                "{} (expected Cooldown, found {:?})",
                message,
                state.status
            );
            self
        }

        /// Assert attack target
        fn then_expect_target(&mut self, expected_target: Entity, message: &str) -> &mut Self {
            let state = self.attack_state();
            assert!(state.is_some(), "Not in attack state: {}", message);
            let state = state.unwrap();
            assert_eq!(state.target, Some(expected_target), "{}", message);
            self
        }

        /// Execute custom assertion
        fn then_custom_assert<F>(&mut self, assert_fn: F, message: &str) -> &mut Self
        where
            F: FnOnce(&Self) -> bool,
        {
            assert!(assert_fn(self), "Custom assertion failed: {}", message);
            self
        }
    }

    // ===== 1. Core State Machine & Flow =====

    /// Goal 1: Complete attack cycle
    #[test]
    fn test_complete_attack_cycle() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        let target = harness.target;

        harness
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .then_expect_target(target, "Attack target should be correct")
            .advance_time(0.3)
            .then_expect_cooldown("Should enter cooldown state after windup ends")
            .then_expect_target(target, "Target should remain the same during cooldown")
            .advance_time(0.7)
            .then_expect_windup("Should automatically start next attack after cooldown ends")
            .then_expect_target(target, "Next attack target should be the same");
    }

    /// Goal 2: Consecutive attacks on same target
    #[test]
    fn test_consecutive_attacks_same_target() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        let target = harness.target;

        harness
            .attack()
            .then_expect_windup("First attack should trigger windup state")
            .advance_time(1.0)
            .then_expect_windup("Should automatically start next attack after cooldown ends")
            .then_expect_target(target, "Next attack target should be the same")
            // Manually send second attack command (test same target doesn't restart)
            .attack()
            .then_expect_windup("Second attack should maintain windup state")
            .then_expect_target(target, "Attack target should remain unchanged");

        assert!(
            (harness.attack_component().windup_duration_secs() - 0.3).abs() < EPSILON,
            "Windup time configuration is correct"
        );
        assert!(
            (harness.attack_component().cooldown_time() - 0.7).abs() < EPSILON,
            "Cooldown time configuration is correct"
        );
    }

    // ===== 2. Attack Cancellation Mechanics =====

    /// Goal 4: Cancel windup during "cancellable" phase
    #[test]
    fn test_cancel_attack_during_cancellable_windup() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .advance_time(0.1)
            .cancel()
            .then_expect_idle("Attack should be cancelled during cancellable period")
            .attack()
            .then_expect_windup("Should be able to start new attack immediately");
    }

    // ===== 3. Attack Reset (Kiting) Mechanics =====

    /// Goal 6: Reset attack during Cooldown
    #[test]
    fn test_attack_reset_during_cooldown() {
        let target = TestHarness::new().target;
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .attack()
            .advance_time(0.3)
            .then_expect_cooldown("Should enter cooldown state")
            .reset()
            .then_expect_windup("Should immediately enter new windup state after reset")
            .then_expect_target(target, "Target should remain unchanged after reset");
    }

    /// Test attack reset event triggering
    #[test]
    fn test_attack_reset_event_triggering() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .attack()
            .advance_time(0.3)
            .then_expect_cooldown("Should enter cooldown state")
            .reset()
            .then_expect_windup("Should enter windup state after reset");
    }

    // ===== 4. Impact of Attack Speed =====

    /// Goal 7: Effect of attack speed changes on attack timing
    #[test]
    fn test_attack_speed_impact_on_timing() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.25, 1.0));

        let initial_attack = harness.attack_component().clone();
        harness
            .modify_attacker(|attack| {
                attack.bonus_attack_speed = 1.0;
            })
            .then_custom_assert(
                |h| {
                    h.attack_component().total_duration_secs()
                        < initial_attack.total_duration_secs()
                },
                "Attack interval should decrease",
            )
            .then_custom_assert(
                |h| {
                    h.attack_component().windup_duration_secs()
                        < initial_attack.windup_duration_secs()
                },
                "Windup time should decrease",
            )
            .then_custom_assert(
                |h| h.attack_component().cooldown_time() < initial_attack.cooldown_time(),
                "Cooldown time should decrease",
            )
            .then_custom_assert(
                |h| {
                    (h.attack_component().windup_duration_secs()
                        + h.attack_component().cooldown_time())
                        < (initial_attack.windup_duration_secs() + initial_attack.cooldown_time())
                },
                "Total attack time should decrease",
            );
    }

    /// Goal 8: Attack speed reaches cap
    #[test]
    fn test_attack_speed_cap() {
        let attack = Attack::from_legacy(0.0, 0.625, 0.0).with_bonus_attack_speed(10.0);

        let min_interval = 1.0 / 2.5;

        let mut harness = TestHarness::new().with_attacker(attack);

        // Verify basic attack speed limit
        assert!(
            (harness.attack_component().current_attack_speed() - 2.5).abs() < EPSILON,
            "Attack speed should be capped at 2.5"
        );
        assert!(
            (harness.attack_component().total_duration_secs() - min_interval).abs() < EPSILON,
            "Attack interval should be minimum value"
        );

        harness.modify_attacker(|attack| {
            attack.bonus_attack_speed = 20.0;
        });

        assert!(
            (harness.attack_component().current_attack_speed() - 2.5).abs() < EPSILON,
            "Further increasing bonus_attack_speed should not change result"
        );
        assert!(
            (harness.attack_component().total_duration_secs() - min_interval).abs() < EPSILON,
            "Attack interval should not change"
        );
    }

    // ===== 5. Windup Configuration & Modifiers =====

    /// Verify Legacy windup formula
    #[test]
    fn test_legacy_windup_formula() {
        let test_cases = [(0.1, 0.4), (-0.1, 0.2), (0.0, 0.3)];

        for (attack_offset, expected_windup) in test_cases {
            let attack = Attack::new(0.0, 0.3 + attack_offset, 1.0);
            let harness = TestHarness::new().with_attacker(attack);
            let actual_windup = harness.attack_component().windup_duration_secs();
            assert!(
                (actual_windup - expected_windup).abs() < EPSILON,
                "In Legacy mode, attack_offset={} should produce windup time {}, actual {}",
                attack_offset,
                expected_windup,
                actual_windup
            );
        }
    }

    /// Verify Modern windup formula
    #[test]
    fn test_modern_windup_formula() {
        let test_cases = [
            (0.25, 1.0, 1.0, 0.25),
            (0.25, 1.0, 2.0, 0.125),
            (0.3, 1.2, 1.5, 0.16666667),
        ];

        for (attack_cast_time, attack_total_time, base_speed, expected_windup) in test_cases {
            let attack = Attack::new(0.0, attack_cast_time, attack_total_time);
            let attack = Attack {
                base_attack_speed: base_speed,
                ..attack
            };
            let harness = TestHarness::new().with_attacker(attack);
            let actual_windup = harness.attack_component().windup_duration_secs();
            assert!(
                (actual_windup - expected_windup).abs() < EPSILON,
                "In Modern mode, config should produce windup time {}, actual {}",
                expected_windup,
                actual_windup
            );
        }
    }

    /// Verify windup_modifier effect
    #[test]
    fn test_windup_modifier_effect() {
        let test_cases = [(1.0, 0.3), (0.5, 0.15), (1.5, 0.45), (0.1, 0.03)];

        for (modifier, expected_windup) in test_cases {
            let attack = Attack::new(0.0, 0.3, 1.0);
            let attack = Attack {
                windup_modifier: modifier,
                ..attack
            };
            let harness = TestHarness::new().with_attacker(attack);
            let actual_windup = harness.attack_component().windup_duration_secs();
            assert!(
                (actual_windup - expected_windup).abs() < EPSILON,
                "Modifier={} should produce windup time {}, actual {}",
                modifier,
                expected_windup,
                actual_windup
            );
        }

        // Test modifier in Legacy mode
        let legacy_attack = Attack::new(0.0, 0.3 + 0.1, 1.0);
        let legacy_attack = Attack {
            windup_modifier: 0.8,
            ..legacy_attack
        };
        let expected_legacy = (0.3 + 0.1) * 0.8;
        let harness = TestHarness::new().with_attacker(legacy_attack);
        let actual_windup = harness.attack_component().windup_duration_secs();
        assert!(
            (actual_windup - expected_legacy).abs() < EPSILON,
            "Legacy mode modifier test, expected {}, actual {}",
            expected_legacy,
            actual_windup
        );
    }

    // ===== 7. Helper Test Functions and Floating Point Precision =====

    /// Attack speed calculation verification
    #[test]
    fn test_attack_speed_calculations() {
        let attack = Attack::from_legacy(0.0, 0.625, 0.0).with_bonus_attack_speed(1.0);

        let harness = TestHarness::new().with_attacker(attack);

        let component = harness.attack_component();
        assert!(
            (component.current_attack_speed() - 1.25).abs() < EPSILON,
            "Current attack speed calculation is incorrect"
        );
        assert!(
            (component.total_duration_secs() - 0.8).abs() < EPSILON,
            "Attack interval calculation is incorrect"
        );
        assert!(
            (component.windup_duration_secs() - 0.3).abs() < EPSILON,
            "Windup time calculation is incorrect"
        );
        assert!(
            (component.cooldown_time() - 0.5).abs() < EPSILON,
            "Cooldown time calculation is incorrect"
        );
    }

    /// Floating point precision test
    #[test]
    fn test_floating_point_precision() {
        let attack = Attack::new(0.0, 0.25, 1.0);
        let attack = Attack {
            base_attack_speed: 0.625,
            bonus_attack_speed: 0.6,
            ..attack
        };

        let expected_speed = 0.625 * (1.0 + 0.6);
        let expected_interval = 1.0 / expected_speed;
        let expected_windup = 0.25 / 1.0 * expected_interval;

        let harness = TestHarness::new().with_attacker(attack);

        let component = harness.attack_component();
        assert!(
            (component.current_attack_speed() - expected_speed).abs() < EPSILON,
            "Attack speed is imprecise"
        );
        assert!(
            (component.total_duration_secs() - expected_interval).abs() < EPSILON,
            "Attack interval is imprecise"
        );
        assert!(
            (component.windup_duration_secs() - expected_windup).abs() < EPSILON,
            "Windup time is imprecise"
        );
    }

    /// Reset attack during windup
    #[test]
    fn test_attack_reset_during_windup() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .reset()
            .then_expect_windup("Reset during windup should return to windup state");
    }

    /// Modern mode attack speed scaling test
    #[test]
    fn test_modern_windup_with_attack_speed_scaling() {
        let mut harness = TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.25, 1.0).with_bonus_attack_speed(1.0));

        let target = harness.target;

        // First verify time calculations
        assert!(
            (harness.attack_component().windup_duration_secs() - 0.125).abs() < EPSILON,
            "Windup time calculation is incorrect"
        );
        assert!(
            (harness.attack_component().cooldown_time() - 0.375).abs() < EPSILON,
            "Cooldown time calculation is incorrect"
        );

        harness
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .advance_time(0.125)
            .then_expect_cooldown("Should enter cooldown state after windup ends")
            .advance_time(0.375)
            .then_expect_windup("Should automatically start next attack after cooldown ends")
            .then_expect_target(target, "Next attack target should be the same");
    }

    /// Uncancellable grace period test
    #[test]
    fn test_uncancellable_grace_period() {
        let attack = Attack::new(0.0, 0.05, 0.95);
        let attack = Attack {
            base_attack_speed: 1.0,
            ..attack
        };

        assert!(attack.windup_duration_secs() < UNCANCELLABLE_GRACE_PERIOD);
    }

    // ===== 8. Target Switching & Attack Cancellation Interaction =====

    /// Test scenario 1: Attack target A, switch to target B during uncancellable period
    /// Expected: Current attack still attacks target A, but next auto attack should attack target B
    #[test]
    fn test_new_target_command_during_uncancellable_period() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        let target_a = harness.target;
        let target_b = harness.spawn_target();

        harness
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .then_expect_target(target_a, "Attack target should be A")
            .advance_time(0.234)
            .switch_target(target_b)
            .attack()
            .then_expect_windup("Attack should re-enter windup state")
            .then_expect_target(target_b, "Current attack target should be B")
            .advance_time(0.5)
            .then_expect_cooldown("Should enter cooldown state")
            .then_expect_target(target_b, "Next attack target is still B")
            .switch_target(target_a)
            .attack()
            .then_expect_cooldown("Attack should still be in cooldown state")
            .then_expect_target(target_a, "Next attack target should be A")
            .advance_time(0.5)
            .then_expect_windup("Should automatically start next attack after cooldown ends")
            .then_expect_target(target_a, "Next attack target should be A");
    }

    /// Test scenario 2: Attack target A, attack target B during cancellable period
    /// Expected: Will immediately cancel current attack, restart attacking target B with timer reset
    #[test]
    fn test_new_target_command_during_cancellable_period() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        let target_a = harness.target;
        let target_b = harness.spawn_target();

        harness
            .attack()
            .then_expect_windup("Attack command should trigger windup state")
            .then_expect_target(target_a, "Attack target should be A")
            .advance_time(0.1)
            .switch_target(target_b)
            .attack()
            .then_expect_windup("Should immediately start attacking target B")
            .then_expect_target(target_b, "Current attack target should be B")
            .advance_time(0.3)
            .then_expect_cooldown("Should enter cooldown state after windup ends")
            .then_expect_target(target_b, "Target should be B during cooldown")
            .advance_time(0.7)
            .then_expect_windup("Should automatically start next attack after cooldown ends")
            .then_expect_target(target_b, "Next attack target should be B");
    }

    // ===== 9. Edge Cases & Exception Handling =====

    /// Test sending cancel command during cooldown
    #[test]
    fn test_cancel_attack_during_cooldown() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .attack()
            .advance_time(0.3)
            .then_expect_cooldown("Should enter cooldown state after windup ends")
            .cancel()
            .then_expect_cooldown("Cancel during cooldown should still be in cooldown state")
            .attack()
            .then_expect_cooldown("Attack command during cooldown should not change state");
    }

    /// Test sending cancel or reset command in idle state
    #[test]
    fn test_cancel_and_reset_in_idle_state() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.3, 1.0))
            .then_expect_idle("Initial state should be idle")
            .cancel()
            .then_expect_idle("Cancel command in idle state should not change state")
            .reset()
            .then_expect_idle("Reset command in idle state should not change state");
    }

    /// Demo: continue_attack control test
    #[test]
    fn test_continue_attack_control() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        // Normal attack cycle
        harness
            .attack()
            .then_custom_assert(
                |h| h.attack_state().unwrap().target.is_some(),
                "Should continue attacking by default",
            )
            .advance_time(1.0)
            .then_expect_windup("Should automatically continue attacking without cancel command")
            // Test cancel command stops auto attack
            .cancel()
            .then_expect_idle("Should return to idle state after cancel");
    }

    // ===== 10. Complex Interaction Scenarios =====

    /// Complex scenario: Kiting and target switching combination
    #[test]
    fn test_complex_kiting_and_target_switching_scenario() {
        let mut harness = TestHarness::new().with_attacker(Attack::new(0.0, 0.3, 1.0));

        let initial_target = harness.target;
        let new_target = harness.spawn_target();

        harness
            .attack()
            .then_expect_windup("Should start attacking initial target")
            .then_expect_target(initial_target, "Target should be initial_target")
            .advance_time(0.1)
            .switch_target(new_target)
            .attack()
            .then_expect_windup("Attack cancelled, immediately start attacking new target")
            .then_expect_target(new_target, "Target should switch to new_target")
            .advance_time(0.3)
            .then_expect_cooldown("Attack on new target complete, enter cooldown")
            .advance_time(0.2)
            .reset()
            .then_expect_windup("Cooldown reset, immediately start new attack")
            .then_expect_target(new_target, "Target is still new_target")
            .advance_time(0.1)
            .switch_target(initial_target)
            .attack()
            .then_expect_windup("Can successfully start attacking initial target")
            .advance_time(0.1)
            .modify_attacker(|attack| {
                attack.bonus_attack_speed = 1.0;
            })
            .advance_time(0.1)
            .then_expect_windup("Adding attack speed during windup doesn't affect windup time")
            .advance_time(0.1)
            .then_expect_cooldown("Enter cooldown")
            .advance_time(0.35)
            .then_expect_windup("After adding attack speed, cooldown becomes 0.35s, enter next attack windup");
    }

    /// Complex scenario: Multi-target switching and cancellation sequence
    #[test]
    fn test_multi_target_switching_and_cancellation_sequence() {
        let mut harness = TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.2, 1.0).with_bonus_attack_speed(2.0));

        let target_a = harness.target;
        let target_b = harness.spawn_target();
        let target_c = harness.spawn_target();

        harness
            .attack()
            .then_expect_windup("Start attacking target A")
            .then_expect_target(target_a, "Confirm attacking target A")
            .advance_time(0.03)
            .switch_target(target_b)
            .attack()
            .then_expect_windup("Switch to attacking target B")
            .then_expect_target(target_b, "Confirm attacking target B")
            .advance_time(0.03)
            .switch_target(target_c)
            .attack()
            .then_expect_windup("Switch to attacking target C")
            .then_expect_target(target_c, "Confirm attacking target C")
            .advance_time(0.1)
            .then_expect_cooldown("Complete attack on target C")
            .advance_time(0.4)
            .then_expect_windup("Automatically start next attack on target C")
            .advance_time(0.04)
            .switch_target(target_a)
            .attack()
            .then_expect_windup("Successfully switch to attacking target A")
            .then_expect_target(target_a, "Target is now A");
    }

    /// Complex scenario: Precise timing control with attack speed changes
    #[test]
    fn test_attack_speed_scaling_with_precise_timing() {
        TestHarness::new()
            .with_attacker(Attack::new(0.0, 0.25, 1.0))
            .attack()
            .then_expect_windup("Start base attack speed attack")
            .then_custom_assert(
                |h| (h.attack_component().windup_duration_secs() - 0.25).abs() < EPSILON,
                "Base attack speed windup time should be 0.25s",
            )
            .advance_time(0.25)
            .then_expect_cooldown("Complete base attack speed windup")
            .then_custom_assert(
                |h| (h.attack_component().cooldown_time() - 0.75).abs() < EPSILON,
                "Base attack speed cooldown time should be 0.75s",
            )
            .advance_time(0.4)
            .modify_attacker(|attack| {
                attack.bonus_attack_speed = 1.0;
            })
            .then_custom_assert(
                |h| (h.attack_component().total_duration_secs() - 0.5).abs() < EPSILON,
                "After attack speed boost, interval should be 0.5s",
            )
            .advance_time(0.35)
            .then_expect_windup("Start attack with new attack speed")
            .then_custom_assert(
                |h| (h.attack_component().windup_duration_secs() - 0.125).abs() < EPSILON,
                "New attack speed windup time should be 0.125s",
            )
            .advance_time(0.125)
            .then_expect_cooldown("Complete new attack speed windup")
            .advance_time(0.375)
            .then_expect_windup("Next attack with new attack speed");
    }
}
