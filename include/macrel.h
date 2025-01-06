#pragma once

// I like fun macros, here's where they live

#define RETURN_IF_INITIALIZED()                        \
  if (static auto initialized = false; !initialized) { \
    initialized = true;                                \
  } else {                                             \
    return;                                            \
  }