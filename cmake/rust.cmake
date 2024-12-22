# Where the Cargo.toml file is located
set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
# Where Rust artifacts get generated to
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
# Where CXX Bridge artifacts are located
set(CXX_BRIDGE_ARTIFACTS ${CARGO_TARGET_DIR}/cxxbridge)
# Rust monorepo directory
set(RUST_MONOREPO_DIR ${CMAKE_SOURCE_DIR}/packages)
# JSKSE_CORE directory. This is the entry point for the Rust implementation
set(RUST_JSKSE_CORE ${RUST_MONOREPO_DIR}/jskse_core)
# JSKSE_MODULES directory. All JS modules are from here
set(RUST_JSKSE_MODULES ${RUST_MONOREPO_DIR}/jskse_modules)

include_directories(${CXX_BRIDGE_ARTIFACTS})

macro (add_rust_monorepo_project _TARGET_NAME)

  set(TARGET_NAME_LOWERED)
  string(TOLOWER ${_TARGET_NAME} TARGET_NAME_LOWERED)
  set(TARGET_NAME ${_TARGET_NAME})
  message(STATUS "Adding Rust monorepo project: ${TARGET_NAME}")
  set("RUST_${TARGET_NAME}_DIR" "${RUST_MONOREPO_DIR}/${TARGET_NAME_LOWERED}")
  message(STATUS "\tRUST_${TARGET_NAME}_DIR: ${RUST_${TARGET_NAME}_DIR}")
  set("RUST_${TARGET_NAME}_SOURCES" "${RUST_${TARGET_NAME}_DIR}/lib.rs")
  message(STATUS "\tRUST_${TARGET_NAME}_SOURCES: ${RUST_${TARGET_NAME}_SOURCES}")
  set("RUST_${TARGET_NAME}_CPP" "${CXX_BRIDGE_ARTIFACTS}/${TARGET_NAME_LOWERED}/lib.rs.cc")
  message(STATUS "\tRUST_${TARGET_NAME}_CPP: ${RUST_${TARGET_NAME}_CPP}")
  set("RUST_${TARGET_NAME}_LIB" "${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}${TARGET_NAME_LOWERED}${CMAKE_STATIC_LIBRARY_SUFFIX}")
  message(STATUS "\tRUST_${TARGET_NAME}_LIB: ${RUST_${TARGET_NAME}_LIB}")

  add_custom_command(
    OUTPUT ${RUST_${TARGET_NAME}_CPP} ${RUST_${TARGET_NAME}_LIB}
    COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --release
    DEPENDS ${RUST_${TARGET_NAME}_SOURCES}
    USES_TERMINAL
    COMMENT "Running Cargo for target ${TARGET_NAME}..."
  )
endmacro ()

add_rust_monorepo_project(JSKSE_CORE)
add_rust_monorepo_project(JSKSE_MODULES)