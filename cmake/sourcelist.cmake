set(headers ${headers}
    include/PCH.h
    include/skse/cosave.h
    include/skse/hooks.h
    include/skse/papyrus.h
    include/skse/sinks.h
    include/log.h
    include/util.h
)
set(sources ${sources}
    ${headers}
    src/main.cpp
    src/skse/cosave.cpp
    src/skse/hooks.cpp
    src/skse/papyrus.cpp
    src/skse/sinks.cpp
    src/util.cpp
)
