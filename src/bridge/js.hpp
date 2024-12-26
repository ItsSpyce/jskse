#pragma once

#include "jskse_core/lib.rs.h"

namespace jskse {
inline void initialize_js_engine() {
  rlog::info("Initializing JavaScript engine.");
  initialize_engine();
}
}  // namespace jskse