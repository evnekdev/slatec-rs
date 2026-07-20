# Generated-output instructions

<!-- agent-rule: generated-no-hand-edit -->
<!-- agent-rule: deterministic-regeneration -->
<!-- agent-rule: generated-no-native-artifacts -->

Scope: `generated/**`.

Do not hand-edit files in this tree.

For any requested change:

1. identify the file's generator and authoritative authored inputs;
2. edit those inputs or the generator;
3. regenerate using the documented command;
4. run the matching validator;
5. run same-root deterministic regeneration;
6. run clean-root deterministic regeneration when supported;
7. inspect for unrelated output churn.

Generated reports are evidence and navigation aids, not the first place to implement behaviour.

Do not add native objects, archives, executables, linker maps, source caches, raw compiler logs, or temporary probe output here. Those belong under ignored `target/` paths.

If the generator cannot be found, stop and report the missing ownership information rather than patching the generated file directly.
