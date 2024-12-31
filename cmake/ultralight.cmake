set(ULTRALIGHT_ROOT $ENV{ULRALIGHT_ROOT})

set(ULTRALIGHT_INCLUDE_DIR ${ULTRALIGHT_ROOT}/include)
set(ULTRALIGHT_LIBRARY_DIR ${ULTRALIGHT_ROOT}/lib)
set(ULTRALIGHT_LIB
  ${ULTRALIGHT_LIBRARY_DIR}/Ultralight.lib
  ${ULTRALIGHT_LIBRARY_DIR}/UltralightCore.lib
  ${ULTRALIGHT_LIBRARY_DIR}/AppCore.lib
  ${ULTRALIGHT_LIBRARY_DIR}/WebCore.lib
)

include_directories(${ULTRALIGHT_INCLUDE_DIR})
