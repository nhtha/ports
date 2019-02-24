--- src/librustc_target/spec/powerpc64_unknown_freebsd.rs.orig	2019-02-15 13:41:07 UTC
+++ src/librustc_target/spec/powerpc64_unknown_freebsd.rs
@@ -0,0 +1,22 @@
+use spec::{LinkerFlavor, Target, TargetResult};
+
+pub fn target() -> TargetResult {
+    let mut base = super::freebsd_base::opts();
+    base.cpu = "ppc64".to_string();
+    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
+    base.max_atomic_width = Some(64);
+
+    Ok(Target {
+        llvm_target: "powerpc64-unknown-freebsd".to_string(),
+        target_endian: "big".to_string(),
+        target_pointer_width: "64".to_string(),
+        target_c_int_width: "32".to_string(),
+        data_layout: "E-m:e-i64:64-n32:64".to_string(),
+        arch: "powerpc64".to_string(),
+        target_os: "freebsd".to_string(),
+        target_env: String::new(),
+        target_vendor: "unknown".to_string(),
+        linker_flavor: LinkerFlavor::Gcc,
+        options: base,
+    })
+}