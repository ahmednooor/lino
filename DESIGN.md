> It's a big a** state machine.

```
   ===
    |
    V
+----------------+
| Wait for Input | <----+
+----------------+      ^
    |                   |
    V                   ^
+--------------+        |
| Handle Input |        ^
+--------------+        |
    |                   ^
    V                   |
+-----------------+     ^
| Transform State |     |
+-----------------+     ^
    |                   |
    V                   ^
+---------------+       |
| Render Screen | >-----+
+---------------+
```

**Source Entry Point**
1. `src/lino/mod.rs`
2. `src/lino/init.rs`
3. Now go where the functions take you.
