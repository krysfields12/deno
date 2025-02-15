// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

use deno_core::url::Url;
use std::collections::HashMap;

static STD_NODE: &str = "https://deno.land/std/node/";

static SUPPORTED_MODULES: &[&str] = &[
  "assert",
  "assert/strict",
  "async_hooks",
  "buffer",
  "child_process",
  "cluster",
  "console",
  "constants",
  "crypto",
  "dgram",
  "dns",
  "domain",
  "events",
  "fs",
  "fs/promises",
  "http",
  "https",
  "module",
  "net",
  "os",
  "path",
  "path/posix",
  "path/win32",
  "perf_hooks",
  "process",
  "querystring",
  "readline",
  "stream",
  "stream/promises",
  "stream/web",
  "string_decoder",
  "sys",
  "timers",
  "timers/promises",
  "tls",
  "tty",
  "url",
  "util",
  "util/types",
  "v8",
  "vm",
  "zlib",
];

pub fn get_node_globals_url() -> Url {
  Url::parse(&format!("{}global.ts", STD_NODE)).unwrap()
}

/// Create a map that can be used to update import map.
///
/// Keys are built-in Node modules (and built-ins prefixed with "node:"), while
/// values are URLs pointing to relevant files in deno.land/std/node/ directory.
pub fn get_mapped_node_builtins() -> HashMap<String, String> {
  let mut mappings = HashMap::new();

  for module in SUPPORTED_MODULES {
    // TODO(bartlomieju): this is unversioned, and should be fixed to use latest stable?
    let module_url = format!("{}{}.ts", STD_NODE, module);
    mappings.insert(module.to_string(), module_url.clone());

    // Support for `node:<module_name>`
    // https://nodejs.org/api/esm.html#esm_node_imports
    let node_prefixed = format!("node:{}", module);
    mappings.insert(node_prefixed, module_url);
  }

  mappings
}
