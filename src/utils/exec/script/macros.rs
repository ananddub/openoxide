// ── Condition atoms — dir/file/cmd/env ─────────────────────────
#[macro_export]
macro_rules! __cond_atom {
    (! dir($p:expr)) => { $crate::utils::exec::Condition::dir_exists($p).not() };
    (! file($p:expr)) => { $crate::utils::exec::Condition::file_exists($p).not() };
    (! cmd($c:expr)) => { $crate::utils::exec::Condition::cmd_succeeds($c).not() };
    (! env($k:expr)) => { $crate::utils::exec::Condition::env_set($k).not() };
    (! ($($inner:tt)+)) => { $crate::cond!($($inner)+).not() };
    (dir($p:expr)) => { $crate::utils::exec::Condition::dir_exists($p) };
    (file($p:expr)) => { $crate::utils::exec::Condition::file_exists($p) };
    (cmd($c:expr)) => { $crate::utils::exec::Condition::cmd_succeeds($c) };
    (env($k:expr)) => { $crate::utils::exec::Condition::env_set($k) };
    (($($inner:tt)+)) => { $crate::cond!($($inner)+) };
}

// ── && chain — left-to-right fold-left ──────────────────────────
#[macro_export]
macro_rules! __cond_and {
    (@start $($tt:tt)+) => { $crate::__cond_and!(@split [] [] $($tt)+) };

    // split: gather segments separated by &&
    (@split [$($segs:tt)*] [$($cur:tt)*] && $($rest:tt)+) => {
        $crate::__cond_and!(@split [$($segs)* [$($cur)*]] [] $($rest)+)
    };
    (@split [$($segs:tt)*] [$($cur:tt)*] $t:tt $($rest:tt)*) => {
        $crate::__cond_and!(@split [$($segs)*] [$($cur)* $t] $($rest)*)
    };
    // done splitting
    (@split [$($segs:tt)*] [$($cur:tt)*]) => {
        $crate::__cond_and!(@fold_start [$($segs)* [$($cur)*]])
    };

    // fold_start: extract the first segment as the initial accumulator
    (@fold_start [[$($first:tt)*] $($rest:tt)*]) => {
        $crate::__cond_and!(@fold $crate::__cond_atom!($($first)*), [$($rest)*])
    };

    // fold: base case
    (@fold $acc:expr, []) => { $acc };
    // fold: recursive step
    (@fold $acc:expr, [[$($next:tt)*] $($rest:tt)*]) => {
        $crate::__cond_and!(@fold $acc.and($crate::__cond_atom!($($next)*)), [$($rest)*])
    };
}

// ── || chain (lower precedence) ─────────────────────────────────
#[macro_export]
macro_rules! __cond_or {
    (@start $($tt:tt)+) => { $crate::__cond_or!(@collect [] $($tt)+) };
    (@collect [$($acc:tt)*] || $($rest:tt)+) => {
        $crate::__cond_and!(@start $($acc)*).or($crate::__cond_or!(@start $($rest)+))
    };
    (@collect [$($acc:tt)*] $head:tt $($rest:tt)*) => {
        $crate::__cond_or!(@collect [$($acc)* $head] $($rest)*)
    };
    (@collect [$($acc:tt)*]) => { $crate::__cond_and!(@start $($acc)*) };
}

#[macro_export]
macro_rules! cond {
    ($($tt:tt)+) => { $crate::__cond_or!(@start $($tt)+) };
}

// ── Step command: name(args...) → "name 'arg1' 'arg2'" ──────────
#[macro_export]
macro_rules! __step_cmd {
    ($name:ident($($arg:expr),* $(,)?)) => {{
        let mut parts = vec![stringify!($name).to_string()];
        $( parts.push($crate::shell_single_quote(&$arg.to_string())); )*
        parts.join(" ")
    }};
}

// ── Pipeline muncher ─────────────────────────────────────────────
#[macro_export]
macro_rules! __pipeline_munch {
    // Base cases
    ($p:ident) => {};
    ($p:ident,) => {};
    ($p:ident;) => {};
    ($p:ident, ;) => {};

    // 1. working_dir
    ($p:ident, working_dir $wd:expr; $($rest:tt)*) => {
        $p = $p.working_dir($wd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 2. env with ident key
    ($p:ident, env $k:ident = $v:expr; $($rest:tt)*) => {
        $p = $p.env(stringify!($k), $v);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 3. env with expr key
    ($p:ident, env $k:expr => $v:expr; $($rest:tt)*) => {
        $p = $p.env($k, $v);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 4. trace
    ($p:ident, trace $t:expr; $($rest:tt)*) => {
        $p = $p.trace($t);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 5. verbose_headers
    ($p:ident, verbose_headers $vh:expr; $($rest:tt)*) => {
        $p = $p.verbose_headers($vh);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 6. let mut binding
    ($p:ident, let mut $var:ident = $val:expr; $($rest:tt)*) => {
        let mut $var = $val;
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 7. let binding
    ($p:ident, let $var:ident = $val:expr; $($rest:tt)*) => {
        let $var = $val;
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 8. if-else block
    ($p:ident, if ($($c:tt)*) { $($then:tt)* } else { $($else_:tt)* } $($rest:tt)*) => {
        let cond_val = $crate::cond!($($c)*);
        let then_b = $crate::pipeline!($($then)*);
        let else_b = $crate::pipeline!($($else_)*);
        $p = $p.if_else(cond_val, then_b, Some(else_b));
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 9. if-only block
    ($p:ident, if ($($c:tt)*) { $($then:tt)* } $($rest:tt)*) => {
        let cond_val = $crate::cond!($($c)*);
        let then_b = $crate::pipeline!($($then)*);
        $p = $p.if_else(cond_val, then_b, None);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 10. and(...) with expression
    ($p:ident, and($cmd:expr); $($rest:tt)*) => {
        $p = $p.and($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 11. and with function-call: and echo("foo");
    ($p:ident, and $name:ident($($arg:expr),* $(,)?); $($rest:tt)*) => {
        let cmd_str = $crate::__step_cmd!($name($($arg),*));
        $p = $p.and(cmd_str);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 12. and with expression: and "echo foo";
    ($p:ident, and $cmd:expr; $($rest:tt)*) => {
        $p = $p.and($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 13. or(...) with expression
    ($p:ident, or($cmd:expr); $($rest:tt)*) => {
        $p = $p.or($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 14. or with function-call: or echo("foo");
    ($p:ident, or $name:ident($($arg:expr),* $(,)?); $($rest:tt)*) => {
        let cmd_str = $crate::__step_cmd!($name($($arg),*));
        $p = $p.or(cmd_str);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 15. or with expression: or "echo foo";
    ($p:ident, or $cmd:expr; $($rest:tt)*) => {
        $p = $p.or($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 16. pipe(...) with expression
    ($p:ident, pipe($cmd:expr); $($rest:tt)*) => {
        $p = $p.pipe($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 17. pipe with function-call: pipe grep("foo");
    ($p:ident, pipe $name:ident($($arg:expr),* $(,)?); $($rest:tt)*) => {
        let cmd_str = $crate::__step_cmd!($name($($arg),*));
        $p = $p.pipe(cmd_str);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 18. pipe with expression: pipe "grep foo";
    ($p:ident, pipe $cmd:expr; $($rest:tt)*) => {
        $p = $p.pipe($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 19. cmd(...) command wrapper
    ($p:ident, cmd($cmd:expr); $($rest:tt)*) => {
        $p = $p.cmd($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 20. function-call style command: echo("foo");
    ($p:ident, $name:ident($($arg:expr),* $(,)?); $($rest:tt)*) => {
        let cmd_str = $crate::__step_cmd!($name($($arg),*));
        $p = $p.cmd(cmd_str);
        $crate::__pipeline_munch!($p, $($rest)*);
    };

    // 21. expression fallback: containers_handle.create("alpine");
    ($p:ident, $cmd:expr; $($rest:tt)*) => {
        $p = $p.cmd($cmd);
        $crate::__pipeline_munch!($p, $($rest)*);
    };
}

#[macro_export]
macro_rules! pipeline {
    ($($tt:tt)*) => {{
        let mut __pipeline = $crate::utils::exec::ScriptPipeline::new();
        $crate::__pipeline_munch!(__pipeline, $($tt)*);
        __pipeline
    }};
}
